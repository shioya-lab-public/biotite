use crate::riscv_isa::{Addr, FReg, Reg};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

static TYPE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"({ty}) \((({ty})(, ({ty}))*)?\)\*?|{ty}",
        ty = r"void|i[[:digit:]]{1,2}\*?|float\*?|double\*?"
    ))
    .unwrap()
});
static VAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(%|@)[[:word:]]+").unwrap());
static FUNC_PROTO: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"define (dso_local )?(?<ret_ty>.+) (?<func>{var})\((?<args_str>.*)\)",
        var = VAR.as_str()
    ))
    .unwrap()
});
static CALL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"(?<ret_var>{var}) = (tail )?call (?<ret_ty>.+) (?<func>{var})\((?<args_str>.*)\)",
        var = VAR.as_str()
    ))
    .unwrap()
});
static LOAD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"(?<dest_arg>{var}) = load (?<dest_ty>.+), (?<src_ty>.+) (?<src_arg>{var})",
        var = VAR.as_str()
    ))
    .unwrap()
});
static STORE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"store (?<src_ty>.+) (?<src_arg>{var}), (?<dest_ty>.+) (?<dest_arg>{var})",
        var = VAR.as_str()
    ))
    .unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Void,
    Int(usize),
    FP(usize),
    Func(Box<Type>, Vec<Type>),
    Ptr(Box<Type>),
}

impl Type {
    pub fn new(s: &str) -> Self {
        if let Some(s) = s.strip_suffix('*') {
            Self::Ptr(Box::new(Self::new(s)))
        } else {
            Self::new_direct(s)
        }
    }

    fn new_direct(s: &str) -> Self {
        use Type::*;

        match s {
            "void" => Void,
            "i1" => Int(1),
            "i8" => Int(8),
            "i16" => Int(16),
            "i32" => Int(32),
            "i64" => Int(64),
            "float" => FP(32),
            "double" => FP(64),
            _ => {
                if let Some(caps) = TYPE.captures(s) {
                    let ret_ty = Self::new(
                        caps.get(1)
                            .unwrap_or_else(|| panic!("Unsupported type `{s}`"))
                            .as_str(),
                    );
                    let args_str = caps.get(2).map_or("", |m| m.as_str());
                    let args = split_args_str(args_str)
                        .iter()
                        .map(|arg_str| Self::new(arg_str))
                        .collect();
                    Func(Box::new(ret_ty), args)
                } else {
                    panic!("Unsupported type `{s}`");
                }
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Type::*;

        match self {
            Void => write!(f, "void"),
            Int(s) => write!(f, "i{s}"),
            FP(32) => write!(f, "float"),
            FP(64) => write!(f, "double"),
            Ptr(ty) => write!(f, "{ty}*"),
            Func(ret_ty, args) => write!(
                f,
                "{ret_ty} ({})",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => unreachable!(),
        }
    }
}

pub fn run(
    ir_funcs: &[String],
    ir_files: Vec<String>,
    symbols: &HashMap<String, Addr>,
) -> Vec<String> {
    ir_files
        .into_iter()
        .map(|ir_file| trans_ir_file(ir_funcs, ir_file, symbols))
        .collect()
}

fn split_args_str(args_str: &str) -> Vec<String> {
    let mut arg_strs: Vec<String> = Vec::new();
    for part in args_str.split(", ") {
        if !part.contains('(') && part.contains(')') {
            arg_strs.last_mut().unwrap().push_str(&format!(", {part}"));
        } else {
            arg_strs.push(part.to_string());
        }
    }
    arg_strs
}

fn trans_ir_file(ir_funcs: &[String], ir_file: String, symbols: &HashMap<String, Addr>) -> String {
    let mut translating = false;
    let mut lines: Vec<String> = Vec::new();
    let mut extern_func_addrs = Vec::new();
    for (line_no, line) in ir_file.lines().enumerate() {
        if let Some(caps) = FUNC_PROTO.captures(line) {
            let ret_ty = Type::new(&caps["ret_ty"]);
            let func = &caps["func"];
            let args_str = caps.name("args_str").map_or("", |m| m.as_str());
            if ir_funcs.contains(&func[1..].to_string()) {
                translating = true;
                let mut attrs = None;
                if let Some(line) = lines.last() {
                    if line.starts_with("; Function Attrs:") {
                        attrs = lines.pop();
                    }
                }
                let addr = symbols
                    .get(&func[1..])
                    .unwrap_or_else(|| panic!("Function `{func}` does not exist in the binary"));
                lines.push(format!("define i64 @.u{addr}(i64) {{"));
                let mut regs = vec![
                    Reg::A7,
                    Reg::A6,
                    Reg::A5,
                    Reg::A4,
                    Reg::A3,
                    Reg::A2,
                    Reg::A1,
                    Reg::A0,
                ];
                let mut fregs = vec![
                    FReg::Fa7,
                    FReg::Fa6,
                    FReg::Fa5,
                    FReg::Fa4,
                    FReg::Fa3,
                    FReg::Fa2,
                    FReg::Fa1,
                    FReg::Fa0,
                ];
                let mut call = String::new();
                for (no, arg_str) in split_args_str(args_str).iter().enumerate() {
                    let caps = TYPE
                        .captures(arg_str)
                        .unwrap_or_else(|| panic!("Unsupported type `{arg_str}`"));
                    let ty = Type::new(&caps[0]);
                    match &ty {
                        Type::Int(64) => lines.push(format!(
                            "  %arg_{no} = load i64, i64* @.{}",
                            regs.pop().unwrap_or_else(|| panic!(
                                "Too many arguments in function `{func}`"
                            ))
                        )),
                        Type::Int(sz) => lines.extend(vec![
                            format!(
                                "  %arg_{no}_val = load i64, i64* @.{}",
                                regs.pop().unwrap_or_else(|| panic!(
                                    "Too many arguments in function `{func}`"
                                ))
                            ),
                            format!("  %arg_{no} = trunc i64 %arg_{no}_val to i{sz}"),
                        ]),
                        Type::FP(64) => lines.push(format!(
                            "  %arg_{no} = load double, double* @.{}",
                            fregs.pop().unwrap_or_else(|| panic!(
                                "Too many arguments in function `{func}`"
                            ))
                        )),
                        Type::FP(32) => lines.extend(vec![
                            format!(
                                "  %arg_{no}_val = load double, double* @.{}",
                                fregs.pop().unwrap_or_else(|| panic!(
                                    "Too many arguments in function `{func}`"
                                ))
                            ),
                            format!("  %arg_{no} = fptrunc double %arg_{no}_val to float"),
                        ]),
                        Type::Ptr(ty) => lines.extend(vec![
                            format!(
                                "  %arg_{no}_val = load i64, i64* @.{}",
                                regs.pop().unwrap_or_else(|| panic!(
                                    "Too many arguments in function `{func}`"
                                ))
                            ),
                            format!("  %arg_{no} = inttoptr i64 %arg_{no}_val to {ty}*"),
                        ]),
                        _ => unreachable!(),
                    }
                    call += &format!("{ty} %arg_{no}, ");
                }
                if !call.is_empty() {
                    call.pop();
                    call.pop();
                }
                match &ret_ty {
                    Type::Void => lines.push(format!("  call void {func}({call})")),
                    Type::Int(64) => lines.extend(vec![
                        format!("  %rslt = call i64 {func}({call})"),
                        format!("  store i64 %rslt, i64* @.a0"),
                    ]),
                    Type::Int(sz) => lines.extend(vec![
                        format!("  %rslt_val = call {ret_ty} {func}({call})"),
                        format!("  %rslt = sext i{sz} %rslt_val to i64"),
                        format!("  store i64 %rslt, i64* @.a0"),
                    ]),
                    Type::FP(64) => lines.extend(vec![
                        format!("  %rslt = call double {func}({call})"),
                        format!("  store double %rslt, double* @.fa0"),
                    ]),
                    Type::FP(32) => lines.extend(vec![
                        format!("  %rslt_val = call float {func}({call})"),
                        format!("  %rslt = fpext float %rslt_val to double"),
                        format!("  store double %rslt, double* @.fa0"),
                    ]),
                    Type::Ptr(ty) => lines.extend(vec![
                        format!("  %rslt_val = call {ty}* {func}({call})"),
                        format!("  %rslt = ptrtoint {ty}* %rslt_val to i64"),
                        format!("  store i64 %rslt, i64* @.a0"),
                    ]),
                    _ => unreachable!(),
                }
                lines.extend(vec![
                    format!("  %ra = load i64, i64* @.ra"),
                    format!("  ret i64 %ra"),
                    format!("}}"),
                    if let Some(line) = attrs {
                        format!("\n{line}")
                    } else {
                        String::new()
                    },
                    line.to_string(),
                ]);
            } else {
                lines.push(line.to_string());
            }
        } else if line == "}" {
            translating = false;
            lines.push(line.to_string());
        } else if translating {
            if let Some(caps) = CALL.captures(line) {
                let ret_var = &caps["ret_var"];
                let ret_ty = Type::new(&caps["ret_ty"]);
                let func = &caps["func"];
                let args_str = caps.name("args_str").map_or("", |m| m.as_str());
                let mut regs = vec![
                    Reg::A7,
                    Reg::A6,
                    Reg::A5,
                    Reg::A4,
                    Reg::A3,
                    Reg::A2,
                    Reg::A1,
                    Reg::A0,
                ];
                let mut fregs = vec![
                    FReg::Fa7,
                    FReg::Fa6,
                    FReg::Fa5,
                    FReg::Fa4,
                    FReg::Fa3,
                    FReg::Fa2,
                    FReg::Fa1,
                    FReg::Fa0,
                ];
                let mut func_ty = format!("{ret_ty} (");
                for (no, arg_str) in split_args_str(args_str).iter().enumerate() {
                    let caps = TYPE
                        .captures(arg_str)
                        .unwrap_or_else(|| panic!("Unsupported type `{arg_str}`"));
                    let ty = Type::new(&caps[0]);
                    let arg = arg_str.rsplit_once(' ').unwrap().1;
                    match &ty {
                        Type::Int(64) => lines.push(format!(
                            "  store i64 {arg}, i64* @.{}",
                            regs.pop().unwrap_or_else(|| panic!(
                                "Too many arguments in function `{func}`"
                            ))
                        )),
                        Type::Int(sz) => lines.extend(vec![
                            format!("  %arg_{no}_{line_no} = sext i{sz} {arg} to i64"),
                            format!(
                                "  store i64 %arg_{no}_{line_no}, i64* @.{}",
                                regs.pop().unwrap_or_else(|| panic!(
                                    "Too many arguments in function `{func}`"
                                ))
                            ),
                        ]),
                        Type::FP(64) => lines.push(format!(
                            "  store double {arg}, double* @.{}",
                            fregs.pop().unwrap_or_else(|| panic!(
                                "Too many arguments in function `{func}`"
                            ))
                        )),
                        Type::FP(32) => lines.extend(vec![
                            format!("  %arg_{no}_{line_no} = fpext float {arg} to double"),
                            format!(
                                "  store double %arg_{no}_{line_no}, double* @.{}",
                                fregs.pop().unwrap_or_else(|| panic!(
                                    "Too many arguments in function `{func}`"
                                ))
                            ),
                        ]),
                        Type::Ptr(ty) => lines.extend(vec![
                            format!("  %arg_{no}_{line_no} = ptrtoint {ty}* {arg} to i64"),
                            format!(
                                "  store i64 %arg_{no}_{line_no}, i64* @.{}",
                                regs.pop().unwrap_or_else(|| panic!(
                                    "Too many arguments in function `{func}`"
                                ))
                            ),
                        ]),
                        _ => unreachable!(),
                    }
                    func_ty += &format!("{ty}, ");
                }
                if !func_ty.is_empty() {
                    func_ty.pop();
                    func_ty.pop();
                }
                func_ty += ")*";
                if let Some(func) = func.strip_prefix('@') {
                    let addr = symbols.get(func).unwrap_or_else(|| {
                        panic!("Function `{func}` does not exist in the binary")
                    });
                    extern_func_addrs.push(addr);
                    lines.push(format!(
                        "  %ra_val_{line_no} = call i64 @.u{addr}(i64 u{addr})"
                    ));
                } else {
                    lines.extend(vec![
                        format!("  %func_val_{line_no} = ptrtoint {func_ty} {func} to i64"),
                        format!(
                            "  %ra_val_{line_no} = call i64 @.dispatch_func(i64 %func_val_{line_no})"
                        ),
                    ]);
                }
                match ret_ty {
                    Type::Int(64) => lines.push(format!("  {ret_var} = load i64, i64* @.a0")),
                    Type::Int(sz) => lines.extend(vec![
                        format!("  %rslt_{line_no} = load i64, i64* @.a0"),
                        format!("  {ret_var} = trunc i64 %rslt_{line_no} to i{sz}"),
                    ]),
                    Type::FP(64) => lines.push(format!("  {ret_var} = load double, double* @.fa0")),
                    Type::FP(32) => lines.extend(vec![
                        format!("  %rslt_{line_no} = load double, double* @.fa0"),
                        format!("  {ret_var} = fptrunc double %rslt_{line_no} to float"),
                    ]),
                    Type::Ptr(ty) => lines.extend(vec![
                        format!("  %rslt_{line_no} = load i64, i64* @.a0"),
                        format!("  {ret_var} = inttoptr i64 %rslt_{line_no} to {ty}*"),
                    ]),
                    _ => unreachable!(),
                }
            } else if let Some(caps) = LOAD.captures(line) {
                let dest_arg = &caps["dest_arg"];
                let dest_ty = Type::new(&caps["dest_ty"]);
                let src_ty = Type::new(&caps["src_ty"]);
                let src_arg = &caps["src_arg"];
                if let Some(src_arg) = src_arg.strip_prefix('@') {
                    let addr = symbols.get(src_arg).unwrap_or_else(|| {
                        panic!("Variable `{}` does not exist in the binary", &src_arg[1..])
                    });
                    lines.push(format!(
                        "  %ptr_b_{line_no} = call i8* @.get_mem_ptr(i64 u{addr})"
                    ));
                } else {
                    lines.extend(vec![
                        format!("  %ptr_val_{line_no} = ptrtoint {src_ty} {src_arg} to i64"),
                        format!(
                            "  %ptr_b_{line_no} = call i8* @.get_mem_ptr(i64 %ptr_val_{line_no})"
                        ),
                    ]);
                }
                match dest_ty {
                    Type::Int(8) => {
                        lines.push(format!("  {dest_arg} = load i8, i8* %ptr_b_{line_no}"))
                    }
                    _ => lines.extend(vec![
                        format!("  %ptr_{line_no} = bitcast i8* %ptr_b_{line_no} to {src_ty}"),
                        format!("  {dest_arg} = load {dest_ty}, {src_ty} %ptr_{line_no}"),
                    ]),
                }
            } else if let Some(caps) = STORE.captures(line) {
                let src_ty = Type::new(&caps["src_ty"]);
                let src_arg = &caps["src_arg"];
                let dest_ty = Type::new(&caps["dest_ty"]);
                let dest_arg = &caps["dest_arg"];
                if let Some(dest_arg) = dest_arg.strip_prefix('@') {
                    let addr = symbols.get(dest_arg).unwrap_or_else(|| {
                        panic!("Variable `{}` does not exist in the binary", &dest_arg[1..])
                    });
                    lines.push(format!(
                        "  %ptr_b_{line_no} = call i8* @.get_mem_ptr(i64 u{addr})"
                    ));
                } else {
                    lines.extend(vec![
                        format!("  %ptr_val_{line_no} = ptrtoint {dest_ty} {dest_arg} to i64"),
                        format!(
                            "  %ptr_b_{line_no} = call i8* @.get_mem_ptr(i64 %ptr_val_{line_no})"
                        ),
                    ]);
                }
                match src_ty {
                    Type::Int(8) => {
                        lines.push(format!("  store i8 {src_arg}, i8* %ptr_b_{line_no}"))
                    }
                    _ => lines.extend(vec![
                        format!("  %ptr_{line_no} = bitcast i8* %ptr_b_{line_no} to {dest_ty}"),
                        format!("  store {src_ty} {src_arg}, {dest_ty} %ptr_{line_no}"),
                    ]),
                }
            } else {
                lines.push(line.to_string());
            }
        } else {
            lines.push(line.to_string());
        }
    }
    if !extern_func_addrs.is_empty() {
        lines.push(String::new());
        lines.extend(
            extern_func_addrs
                .iter()
                .map(|addr| format!("declare i64 @.u{addr}(i64)")),
        );
    }
    lines.push(
        "
declare i8* @.get_mem_ptr(i64)
declare i64 @.dispatch_func(i64)

@.ra = external global i64
@.a0 = external global i64
@.a1 = external global i64
@.a2 = external global i64
@.a3 = external global i64
@.a4 = external global i64
@.a5 = external global i64
@.a6 = external global i64
@.a7 = external global i64

@.fa0 = external global double
@.fa1 = external global double
@.fa2 = external global double
@.fa3 = external global double
@.fa4 = external global double
@.fa5 = external global double
@.fa6 = external global double
@.fa7 = external global double
"
        .to_string(),
    );
    lines.join("\n")
}
