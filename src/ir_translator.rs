use crate::llvm_isa::Prog;
use crate::riscv_isa::{Addr, FReg, Reg};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::mem;
use std::path::PathBuf;
use std::process::Command;

macro_rules! regex {
    ( $re:literal ) => {{
        static RE: OnceCell<Regex> = OnceCell::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
    }};
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Value {
    Const(String),
    ConstExp(ConstExp),
    Ident(Ident),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Value::*;

        match self {
            Const(cnst) => write!(f, "{cnst}"),
            ConstExp(cnst_exp) => write!(f, "{cnst_exp}"),
            Ident(ident) => write!(f, "{ident}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ConstExp {
    Bitcast {
        from_ty: Type,
        from: Box<Value>,
        to_ty: Type,
    },
    Getelementptr {
        ty: Type,
        ptr: Box<Value>,
        idxes: Vec<(Value, Type)>,
    },
}

impl Display for ConstExp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use ConstExp::*;

        match self {
            Bitcast {
                from_ty,
                from,
                to_ty,
            } => write!(f, "bitcast ({from_ty} {from} to {to_ty})"),
            Getelementptr { ty, ptr, idxes } => write!(
                f,
                "getelementptr ({ty}, {ty}* {ptr}, {})",
                idxes
                    .iter()
                    .map(|(idx, ty)| format!("{ty} {idx}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Ident {
    Global(String),
    Local(String),
}

impl Ident {
    fn name(&self) -> &str {
        use Ident::*;

        match self {
            Global(ident) | Local(ident) => ident,
        }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Ident::*;

        match self {
            Global(ident) => write!(f, "@{ident}"),
            Local(ident) => write!(f, "%{ident}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Type {
    Void,
    Func(Box<Type>, Vec<Type>),
    Int(usize, bool),
    Float,
    Double,
    Ptr(Box<Type>),
    Array(usize, Box<Type>),
    Struct(String),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Type::*;

        match self {
            Void => write!(f, "void"),
            Func(rslt_ty, param_tys) => write!(
                f,
                "{rslt_ty} ({})",
                param_tys
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Int(sz, _) => write!(f, "i{sz}"),
            Float => write!(f, "float"),
            Double => write!(f, "double"),
            Ptr(ty) => write!(f, "{ty}*"),
            Array(sz, ty) => write!(f, "[{sz} x {ty}]"),
            Struct(name) => write!(f, "%struct.{name}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Proto {
    rslt_ty: Type,
    func: Ident,
    params: Vec<(Value, Type)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Call {
    rslt: Option<Ident>,
    rslt_ty: Type,
    func: Ident,
    args: Vec<(Value, Type)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Load {
    dest: Ident,
    dest_ty: Type,
    src: Value,
    src_ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Store {
    dest: Value,
    dest_ty: Type,
    src: Value,
    src_ty: Type,
}

struct LineParser<'a> {
    line: &'a str,
    index: usize,
}

impl<'a> LineParser<'a> {
    pub fn new(line: &'a str) -> Self {
        Self { line, index: 0 }
    }

    pub fn parse_proto(&mut self) -> Result<Proto, ()> {
        self.assert_word("define")?;
        let _ = self.assert_word("dso_local");
        let mut zeroext = false;
        let rslt_ty = loop {
            if let Ok(ty) = self.parse_type() {
                if let Type::Int(sz, _) = ty {
                    break Type::Int(sz, mem::take(&mut zeroext));
                } else {
                    break ty;
                }
            } else {
                if self.assert_word("zeroext").is_ok() {
                    zeroext = true;
                } else if self.assert_word("align").is_ok() && self.assert_word("(").is_err() {
                    self.parse_value()?;
                } else {
                    self.index += self.line[self.index..]
                        .chars()
                        .position(|c| c == ' ')
                        .unwrap();
                    self.skip_whitespace();
                }
                if self.assert_word("asm").is_ok() || self.assert_word("@").is_ok() {
                    return Err(());
                }
            }
        };
        if self.assert_word("asm").is_ok() {
            return Err(());
        }
        let func = self.parse_ident()?;
        self.assert_word("(")?;
        let mut params = Vec::new();
        while self.assert_word(")").is_err() {
            let mut param_ty = self.parse_type()?;
            let param = loop {
                if let Ok(val) = self.parse_value() {
                    break val;
                } else if self.assert_word("zeroext").is_ok() {
                    zeroext = true;
                } else if self.assert_word("align").is_ok() && self.assert_word("(").is_err() {
                    self.parse_value()?;
                } else {
                    self.index += self.line[self.index..]
                        .chars()
                        .position(|c| c == ' ')
                        .unwrap();
                    self.skip_whitespace();
                }
            };
            if let Type::Int(sz, _) = param_ty {
                param_ty = Type::Int(sz, mem::take(&mut zeroext));
            }
            params.push((param, param_ty));
            let _ = self.assert_word(",");
        }
        Ok(Proto {
            rslt_ty,
            func,
            params,
        })
    }

    pub fn parse_call(&mut self) -> Result<Call, ()> {
        self.skip_whitespace();
        let _ = self.assert_word("tail");
        let rslt = if self.assert_word("call").is_ok() {
            None
        } else {
            let rslt = Some(self.parse_ident()?);
            self.assert_word("=")?;
            let _ = self.assert_word("tail");
            self.assert_word("call")?;
            rslt
        };
        let line = format!("define {}", &self.line[self.index..]);
        let proto = LineParser::new(&line).parse_proto()?;
        Ok(Call {
            rslt,
            rslt_ty: proto.rslt_ty,
            func: proto.func,
            args: proto.params,
        })
    }

    pub fn parse_load(&mut self) -> Result<Load, ()> {
        self.skip_whitespace();
        let dest = self.parse_ident()?;
        self.assert_word("= load")?;
        let dest_ty = self.parse_type()?;
        self.assert_word(",")?;
        let src_ty = self.parse_type()?;
        let src = self.parse_value()?;
        Ok(Load {
            dest,
            dest_ty,
            src,
            src_ty,
        })
    }

    pub fn parse_store(&mut self) -> Result<Store, ()> {
        self.skip_whitespace();
        self.assert_word("store")?;
        let src_ty = self.parse_type()?;
        let src = self.parse_value()?;
        self.assert_word(",")?;
        let dest_ty = self.parse_type()?;
        let dest = self.parse_value()?;
        Ok(Store {
            dest,
            dest_ty,
            src,
            src_ty,
        })
    }

    fn assert_word(&mut self, word: &str) -> Result<(), ()> {
        if self.index + word.len() <= self.line.len()
            && &self.line[self.index..self.index + word.len()] == word
        {
            self.index += word.len();
            self.skip_whitespace();
            Ok(())
        } else {
            Err(())
        }
    }

    fn skip_whitespace(&mut self) {
        self.index = self.line.len() - self.line[self.index..].trim_start().len();
    }

    fn parse_value(&mut self) -> Result<Value, ()> {
        self.parse_const()
            .or_else(|_| self.parse_const_exp())
            .or_else(|_| self.parse_ident().map(Value::Ident))
    }

    fn parse_const(&mut self) -> Result<Value, ()> {
        let caps = regex!(
            r"^(-?\d+)|^(\d+\.\d+e\+\d+)|^(0x[[:xdigit:]]+)|^(null)|^(true)|^(false)|^(poison)"
        )
        .captures(&self.line[self.index..])
        .ok_or(())?;
        let cnst = &caps[0];
        self.index += cnst.len();
        self.skip_whitespace();
        Ok(Value::Const(cnst.to_string()))
    }

    fn parse_const_exp(&mut self) -> Result<Value, ()> {
        if self.assert_word("bitcast").is_ok() {
            self.assert_word("(")?;
            let from_ty = self.parse_type()?;
            let from = self.parse_value()?;
            self.assert_word("to")?;
            let to_ty = self.parse_type()?;
            self.assert_word(")")?;
            Ok(Value::ConstExp(ConstExp::Bitcast {
                from_ty,
                from: Box::new(from),
                to_ty,
            }))
        } else if self.assert_word("getelementptr").is_ok() {
            let _ = self.assert_word("inbounds");
            self.assert_word("(")?;
            let ty = self.parse_type()?;
            self.assert_word(",")?;
            self.parse_type()?;
            let ptr = self.parse_value()?;
            self.assert_word(",")?;
            let mut idxes = Vec::new();
            while self.assert_word(")").is_err() {
                let idx_ty = self.parse_type()?;
                let idx = self.parse_value()?;
                idxes.push((idx, idx_ty));
                let _ = self.assert_word(",");
            }
            Ok(Value::ConstExp(ConstExp::Getelementptr {
                ty,
                ptr: Box::new(ptr),
                idxes,
            }))
        } else {
            Err(())
        }
    }

    fn parse_ident(&mut self) -> Result<Ident, ()> {
        let caps = regex!(r"^[@%][-a-zA-Z$._0-9]+")
            .captures(&self.line[self.index..])
            .ok_or(())?;
        let ident = &caps[0];
        self.index += ident.len();
        self.skip_whitespace();
        match &ident[0..1] {
            "@" => Ok(Ident::Global(ident[1..].to_string())),
            "%" => Ok(Ident::Local(ident[1..].to_string())),
            _ => unreachable!(),
        }
    }

    fn parse_type(&mut self) -> Result<Type, ()> {
        let mut ty = if self.assert_word("void").is_ok() {
            Type::Void
        } else if let Some(caps) = regex!(r"^i(\d+)").captures(&self.line[self.index..]) {
            let sz = &caps[1];
            self.index += sz.len() + 1;
            self.skip_whitespace();
            Type::Int(sz.parse().unwrap(), false)
        } else if self.assert_word("float").is_ok() {
            Type::Float
        } else if self.assert_word("double").is_ok() {
            Type::Double
        } else if self.assert_word("[").is_ok() {
            let caps = regex!(r"\d+")
                .captures(&self.line[self.index..])
                .ok_or(())?;
            let sz = &caps[0];
            self.index += sz.len();
            self.skip_whitespace();
            self.assert_word("x")?;
            let ty = self.parse_type()?;
            self.assert_word("]")?;
            Type::Array(sz.parse().unwrap(), Box::new(ty))
        } else if let Ok(Ident::Local(ident)) = self.parse_ident() {
            Type::Struct(String::from(ident.strip_prefix("struct.").ok_or(())?))
        } else {
            return Err(());
        };
        while self.assert_word("*").is_ok() {
            ty = Type::Ptr(Box::new(ty));
        }
        self.skip_whitespace();
        while self.assert_word("(").is_ok() {
            let mut param_tys = Vec::new();
            while self.assert_word(")").is_err() {
                param_tys.push(self.parse_type()?);
                let _ = self.assert_word(",");
            }
            ty = Type::Func(Box::new(ty), param_tys);
            while self.assert_word("*").is_ok() {
                ty = Type::Ptr(Box::new(ty));
            }
            self.skip_whitespace();
        }
        Ok(ty)
    }
}

pub fn run(
    srcs: Vec<PathBuf>,
    symbols: &HashMap<String, Addr>,
    ir_dir: PathBuf,
    prog: &Prog,
) -> Vec<String> {
    let mut ir_funcs = Vec::new();
    for src in srcs {
        let mut ir_dir = ir_dir.clone();
        if src.is_dir() {
            if let Some(file_name) = src.file_name() {
                ir_dir.push(file_name);
                fs::create_dir(&ir_dir).expect("Unable to create the IR directory");
            }
            if let Ok(dir) = fs::read_dir(&src) {
                let paths = dir
                    .into_iter()
                    .flatten()
                    .map(|entry| entry.path())
                    .collect();
                ir_funcs.extend(run(paths, symbols, ir_dir.clone(), prog));
            }
        } else {
            let ext = src.extension().and_then(|ext| ext.to_str());
            match ext {
                Some("ll") => ir_dir.push(src.file_name().unwrap()),
                Some("c") | Some("cpp") | Some("cxx") | Some("cc") => {
                    ir_dir.push(src.with_extension("ll").file_name().unwrap())
                }
                _ => continue,
            }
            ir_funcs.extend(
                trans_file(&src, symbols, &ir_dir, prog)
                    .iter()
                    .map(|ident| ident.name().to_string()),
            );
        }
    }
    ir_funcs
}

fn trans_file(
    path: &PathBuf,
    symbols: &HashMap<String, Addr>,
    output: &PathBuf,
    prog: &Prog,
) -> Vec<Ident> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("ll") => {
            if Command::new("cp").args([path, output]).status().is_err() {
                return Vec::new();
            }
        }
        Some(ext) => {
            let clang = if ext == "c" {
                env::var("CLANG").expect("The environment variable `CLANG` is not set")
            } else {
                env::var("CLANGXX").expect("The environment variable `CLANGXX` is not set")
            };
            if Command::new(&clang)
                .args([
                    "-Xclang",
                    "-no-opaque-pointers",
                    "-O1",
                    "-S",
                    "-emit-llvm",
                    "-o",
                ])
                .args([output, path])
                .status()
                .is_err()
            {
                return Vec::new();
            }
        }
        _ => unreachable!(),
    }
    let mut ir_funcs = Vec::new();
    let mut lines = Vec::new();
    let mut cache: Vec<String> = Vec::new();
    let mut extern_func_addrs = HashSet::new();
    let mut proto_idx = 0;
    for line in fs::read_to_string(output).unwrap().lines() {
        if line.starts_with("define") {
            proto_idx = 0;
            if let Some(line) = cache.pop() {
                if line.starts_with("; Function Attrs") {
                    proto_idx = 1;
                    lines.extend(mem::replace(&mut cache, vec![line]));
                } else {
                    cache.push(line);
                    lines.extend(mem::take(&mut cache));
                }
            }
            cache.push(line.to_string());
        } else if line == "}" {
            cache.push(line.to_string());
            let idx = cache[proto_idx].chars().position(|c| c == '@').unwrap();
            if &cache[proto_idx][idx..idx + 5] == "@main" {
                cache[proto_idx].insert(idx + 1, '.');
            }
            if let Ok((f, ls, exts)) = trans_func(proto_idx, mem::take(&mut cache), symbols) {
                ir_funcs.push(f);
                lines.extend(ls);
                extern_func_addrs.extend(exts);
            } else if let Some(true) = lines.last().map(|line| line.is_empty()) {
                lines.pop();
            }
        } else {
            cache.push(line.to_string());
        }
    }
    lines.extend(cache);
    lines.push(
        "
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
@.fa7 = external global double"
            .to_string(),
    );
    let ir_funcs_set = ir_funcs.iter().map(|func| symbols[func.name()]).collect();
    let mut extern_func_addrs: Vec<_> = extern_func_addrs.difference(&ir_funcs_set).collect();
    if !extern_func_addrs.is_empty() {
        extern_func_addrs.sort_unstable();
        lines.push(String::new());
        lines.extend(
            extern_func_addrs
                .into_iter()
                .map(|addr| format!("declare i64 @.u{addr}(i64)")),
        );
    }
    lines.extend([
        String::new(),
        prog.build_mem(true),
        String::new(),
        prog.build_dispatchers(true).1,
    ]);
    if ir_funcs.is_empty() {
        fs::remove_file(output).unwrap();
    } else {
        lines.push(String::new());
        fs::write(output, lines.join("\n")).unwrap();
    }
    ir_funcs
}

fn trans_func(
    proto_idx: usize,
    lines: Vec<String>,
    symbols: &HashMap<String, Addr>,
) -> Result<(Ident, Vec<String>, Vec<Addr>), ()> {
    let mut extern_func_addrs = HashSet::new();
    let proto = LineParser::new(&lines[proto_idx]).parse_proto()?;
    let addr = symbols.get(proto.func.name()).ok_or(())?;
    let mut adaptor = trans_proto(addr, &proto)?;
    let lines = lines
        .into_iter()
        .enumerate()
        .map(|(line_no, line)| {
            if regex!(r"^(  (tail )?call)|(= (tail )?call)").is_match(&line) {
                let call = LineParser::new(&line).parse_call()?;
                let (ls, exts) = trans_call(line_no, &call, symbols)?;
                extern_func_addrs.extend(exts);
                Ok(ls)
            } else if line.contains("= load") {
                let load = LineParser::new(&line).parse_load()?;
                trans_load(line_no, &load, symbols)
            } else if line.starts_with("  store") {
                let store = LineParser::new(&line).parse_store()?;
                trans_store(line_no, &store, symbols)
            } else {
                Ok(vec![line])
            }
        })
        .collect::<Result<Vec<_>, ()>>()?;
    adaptor.push(String::new());
    adaptor.extend(lines.into_iter().flatten());
    Ok((proto.func, adaptor, extern_func_addrs.into_iter().collect()))
}

fn trans_proto(addr: &Addr, proto: &Proto) -> Result<Vec<String>, ()> {
    let mut lines = vec![format!("define i64 @.u{addr}(i64) {{")];
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
    let mut args = Vec::new();
    for (no, (_, ty)) in proto.params.iter().enumerate() {
        match ty {
            Type::Int(64, _) => lines.push(format!(
                "  %arg_{no} = load i64, i64* @.{}",
                regs.pop().ok_or(())?,
            )),
            Type::Int(sz, _) => lines.extend([
                format!(
                    "  %arg_{no}_val = load i64, i64* @.{}",
                    regs.pop().ok_or(())?,
                ),
                format!("  %arg_{no} = trunc i64 %arg_{no}_val to i{sz}"),
            ]),
            Type::Float => lines.extend([
                format!(
                    "  %arg_{no}_val = load double, double* @.{}",
                    fregs.pop().ok_or(())?,
                ),
                format!("  %arg_{no} = fptrunc double %arg_{no}_val to float"),
            ]),
            Type::Double => lines.push(format!(
                "  %arg_{no} = load double, double* @.{}",
                fregs.pop().ok_or(())?,
            )),
            Type::Ptr(ty) => lines.extend([
                format!(
                    "  %arg_{no}_val = load i64, i64* @.{}",
                    regs.pop().ok_or(())?,
                ),
                format!("  %arg_{no} = inttoptr i64 %arg_{no}_val to {ty}*"),
            ]),
            _ => Err(())?,
        }
        args.push(format!("{ty} %arg_{no}"));
    }
    match &proto.rslt_ty {
        Type::Void => lines.push(format!("  call void {}({})", proto.func, args.join(", "))),
        Type::Int(64, _) => lines.extend([
            format!("  %rslt = call i64 {}({})", proto.func, args.join(", ")),
            "  store i64 %rslt, i64* @.a0".to_string(),
        ]),
        Type::Int(sz, zeroext) => lines.extend([
            format!(
                "  %rslt_val = call i{sz} {}({})",
                proto.func,
                args.join(", ")
            ),
            format!(
                "  %rslt = {} i{sz} %rslt_val to i64",
                if *zeroext { "zext" } else { "sext" }
            ),
            "  store i64 %rslt, i64* @.a0".to_string(),
        ]),
        Type::Float => lines.extend([
            format!(
                "  %rslt_val = call float {}({})",
                proto.func,
                args.join(", ")
            ),
            "  %rslt = fpext float %rslt_val to double".to_string(),
            "  store double %rslt, double* @.fa0".to_string(),
        ]),
        Type::Double => lines.extend([
            format!("  %rslt = call double {}({})", proto.func, args.join(", ")),
            "  store double %rslt, double* @.fa0".to_string(),
        ]),
        Type::Ptr(ty) => lines.extend([
            format!(
                "  %rslt_val = call {ty}* {}({})",
                proto.func,
                args.join(", ")
            ),
            format!("  %rslt = ptrtoint {ty}* %rslt_val to i64"),
            "  store i64 %rslt, i64* @.a0".to_string(),
        ]),
        _ => Err(())?,
    }
    lines.push(String::from(
        "  %ra = load i64, i64* @.ra
  ret i64 %ra
}",
    ));
    Ok(lines)
}

fn trans_call(
    line_no: usize,
    call: &Call,
    symbols: &HashMap<String, Addr>,
) -> Result<(Vec<String>, HashSet<Addr>), ()> {
    let mut lines = Vec::new();
    let mut extern_func_addrs = HashSet::new();
    if call.func.name().starts_with("llvm.") {
        let mut args = Vec::new();
        for (no, (arg, ty)) in call.args.iter().enumerate() {
            let arg = trans_arg(no, line_no, arg, &mut lines, symbols);
            if let Type::Ptr(ty) = ty {
                lines.push(format!(
                    "  %arg_{no}_val_{line_no} = ptrtoint {ty}* {arg} to i64"
                ));
                if let Type::Int(8, _) = **ty {
                    lines.push(format!("  %arg_{no}_{line_no} = call i8* @.get_mem_ptr(i64 %arg_{no}_val_{line_no})"));
                } else {
                    lines.extend([
                        format!("  %arg_{no}_b_{line_no} = call i8* @.get_mem_ptr(i64 %arg_{no}_val_{line_no})"),
                        format!("  %arg_{no}_{line_no} = bitcast i8* %arg_{no}_b_{line_no} to {ty}*"),
                    ]);
                }
                args.push(format!("{ty}* %arg_{no}_{line_no}"));
            } else {
                args.push(format!("{ty} {arg}"));
            }
        }
        if let Some(rslt) = &call.rslt {
            lines.push(format!(
                "  {rslt} = call {} {}({})",
                call.rslt_ty,
                call.func,
                args.join(", ")
            ));
        } else {
            lines.push(format!(
                "  call {} {}({})",
                call.rslt_ty,
                call.func,
                args.join(", ")
            ));
        }
    } else {
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
        let mut params = Vec::new();
        for (no, (arg, ty)) in call.args.iter().enumerate() {
            let arg = trans_arg(no, line_no, arg, &mut lines, symbols);
            match ty {
                Type::Int(64, _) => lines.push(format!(
                    "  store i64 {arg}, i64* @.{}",
                    regs.pop().ok_or(())?,
                )),
                Type::Int(sz, zeroext) => lines.extend([
                    format!(
                        "  %arg_{no}_{line_no} = {} i{sz} {arg} to i64",
                        if *zeroext { "zext" } else { "sext" }
                    ),
                    format!(
                        "  store i64 %arg_{no}_{line_no}, i64* @.{}",
                        regs.pop().ok_or(())?,
                    ),
                ]),
                Type::Float => lines.extend([
                    format!("  %arg_{no}_{line_no} = fpext float {arg} to double"),
                    format!(
                        "  store double %arg_{no}_{line_no}, double* @.{}",
                        fregs.pop().ok_or(())?,
                    ),
                ]),
                Type::Double => lines.push(format!(
                    "  store double {arg}, double* @.{}",
                    fregs.pop().ok_or(())?
                )),
                Type::Ptr(ty) => lines.extend([
                    format!("  %arg_{no}_{line_no} = ptrtoint {ty}* {arg} to i64"),
                    format!(
                        "  store i64 %arg_{no}_{line_no}, i64* @.{}",
                        regs.pop().ok_or(())?,
                    ),
                ]),
                _ => Err(())?,
            }
            params.push(ty.to_string());
        }
        if let Ident::Global(func) = &call.func {
            let addr = symbols.get(func).ok_or(())?;
            extern_func_addrs.insert(*addr);
            lines.push(format!("  %ra_{line_no} = call i64 @.u{addr}(i64 u{addr})"));
        } else {
            lines.extend([
                format!(
                    "  %func_{line_no} = ptrtoint {} ({})* {} to i64",
                    call.rslt_ty,
                    params.join(", "),
                    call.func
                ),
                format!("  %ra_{line_no} = call i64 @.dispatch_func(i64 %func_{line_no})"),
            ]);
        }
        match &call.rslt_ty {
            Type::Void => (),
            Type::Int(64, _) => lines.push(format!(
                "  {} = load i64, i64* @.a0",
                call.rslt.as_ref().unwrap()
            )),
            Type::Int(sz, _) => lines.extend([
                format!("  %rslt_{line_no} = load i64, i64* @.a0"),
                format!(
                    "  {} = trunc i64 %rslt_{line_no} to i{sz}",
                    call.rslt.as_ref().unwrap()
                ),
            ]),
            Type::Float => lines.extend([
                format!("  %rslt_{line_no} = load double, double* @.fa0"),
                format!(
                    "  {} = fptrunc double %rslt_{line_no} to float",
                    call.rslt.as_ref().unwrap()
                ),
            ]),
            Type::Double => lines.push(format!(
                "  {} = load double, double* @.fa0",
                call.rslt.as_ref().unwrap()
            )),
            Type::Ptr(ty) => lines.extend([
                format!("  %rslt_{line_no} = load i64, i64* @.a0"),
                format!(
                    "  {} = inttoptr i64 %rslt_{line_no} to {ty}*",
                    call.rslt.as_ref().unwrap()
                ),
            ]),
            _ => Err(())?,
        }
    }
    Ok((lines, extern_func_addrs))
}

fn trans_arg(
    no: usize,
    line_no: usize,
    arg: &Value,
    lines: &mut Vec<String>,
    symbols: &HashMap<String, Addr>,
) -> Value {
    if let Value::ConstExp(arg) = arg {
        match arg {
            ConstExp::Bitcast {
                from,
                to_ty: Type::Ptr(to_ty),
                ..
            } => {
                if let Value::Ident(Ident::Global(from)) = &**from {
                    if let Some(addr) = symbols.get(from) {
                        if let Type::Int(8, _) = **to_ty {
                            lines.push(format!(
                                "  %_arg_{no}_{line_no} = call i8* @.get_mem_ptr(i64 u{addr})"
                            ));
                        } else {
                            lines.extend([
                            format!("  %_arg_{no}_b_{line_no} = call i8* @.get_mem_ptr(i64 u{addr}"),
                            format!("  %_arg_{no}_{line_no} = bitcast i8* %_arg_{no}_b_{line_no} to {to_ty}*"),
                        ]);
                        }
                        Value::Ident(Ident::Local(format!("_arg_{no}_{line_no}")))
                    } else {
                        Value::ConstExp(arg.clone())
                    }
                } else {
                    Value::ConstExp(arg.clone())
                }
            }
            _ => Value::ConstExp(arg.clone()),
        }
    } else {
        arg.clone()
    }
}

fn trans_load(
    line_no: usize,
    load: &Load,
    symbols: &HashMap<String, Addr>,
) -> Result<Vec<String>, ()> {
    let mut lines = Vec::new();
    let src = match &load.src {
        Value::Ident(ident) => ident,
        Value::ConstExp(ConstExp::Getelementptr { ptr, .. }) => {
            if let Value::Ident(ident) = &**ptr {
                ident
            } else {
                return Err(());
            }
        }
        _ => return Err(()),
    };
    match src {
        Ident::Global(src) => {
            let addr = symbols.get(src).ok_or(())?;
            lines.push(format!(
                "  %src_b_{line_no} = call i8* @.get_mem_ptr(i64 u{addr})"
            ));
        }
        Ident::Local(src) => {
            lines.extend([
                format!(
                    "  %src_val_{line_no} = ptrtoint {} %{src} to i64",
                    load.src_ty
                ),
                format!("  %src_b_{line_no} = call i8* @.get_mem_ptr(i64 %src_val_{line_no})"),
            ]);
        }
    }
    match (&load.dest_ty, &load.src) {
        (Type::Int(8, _), Value::Ident(_)) => {
            lines.push(format!("  {} = load i8, i8* %src_b_{line_no}", load.dest))
        }
        (_, Value::Ident(_)) => lines.extend([
            format!(
                "  %src_{line_no} = bitcast i8* %src_b_{line_no} to {}",
                load.src_ty
            ),
            format!(
                "  {} = load {}, {} %src_{line_no}",
                load.dest, load.dest_ty, load.src_ty
            ),
        ]),
        (_, Value::ConstExp(ConstExp::Getelementptr { ty, idxes, .. })) => {
            let idxes = idxes
                .iter()
                .map(|(idx, ty)| format!("{ty} {idx}"))
                .collect::<Vec<_>>()
                .join(", ");
            lines.extend([
                format!("  %src_{line_no} = bitcast i8* %src_b_{line_no} to {ty}*"),
                format!("  %gep_{line_no} = getelementptr {ty}, {ty}* %src_{line_no}, {idxes}"),
                format!(
                    "  {} = load {}, {} %gep_{line_no}",
                    load.dest, load.dest_ty, load.src_ty
                ),
            ]);
        }
        _ => unreachable!(),
    }
    Ok(lines)
}

fn trans_store(
    line_no: usize,
    store: &Store,
    symbols: &HashMap<String, Addr>,
) -> Result<Vec<String>, ()> {
    let mut lines = Vec::new();
    let dest = match &store.dest {
        Value::Ident(ident) => ident,
        Value::ConstExp(ConstExp::Getelementptr { ptr, .. }) => {
            if let Value::Ident(ident) = &**ptr {
                ident
            } else {
                return Err(());
            }
        }
        _ => return Err(()),
    };
    match dest {
        Ident::Global(dest) => {
            let addr = symbols.get(dest).ok_or(())?;
            lines.push(format!(
                "  %dest_b_{line_no} = call i8* @.get_mem_ptr(i64 u{addr})"
            ));
        }
        Ident::Local(dest) => {
            lines.extend([
                format!(
                    "  %dest_val_{line_no} = ptrtoint {} %{dest} to i64",
                    store.dest_ty
                ),
                format!("  %dest_b_{line_no} = call i8* @.get_mem_ptr(i64 %dest_val_{line_no})"),
            ]);
        }
    }
    match (&store.src_ty, &store.dest) {
        (Type::Int(8, _), Value::Ident(_)) => {
            lines.push(format!("  store i8 {}, i8* %dest_b_{line_no}", store.src))
        }
        (_, Value::Ident(_)) => lines.extend([
            format!(
                "  %dest_{line_no} = bitcast i8* %dest_b_{line_no} to {}",
                store.dest_ty
            ),
            format!(
                "  store {} {}, {} %dest_{line_no}",
                store.src_ty, store.src, store.dest_ty
            ),
        ]),
        (_, Value::ConstExp(ConstExp::Getelementptr { ty, idxes, .. })) => {
            let idxes = idxes
                .iter()
                .map(|(idx, ty)| format!("{ty} {idx}"))
                .collect::<Vec<_>>()
                .join(", ");
            lines.extend([
                format!("  %dest_{line_no} = bitcast i8* %dest_b_{line_no} to {ty}*"),
                format!("  %gep_{line_no} = getelementptr {ty}, {ty}* %dest_{line_no}, {idxes}"),
                format!(
                    "  store {} {}, {} %gep_{line_no}",
                    store.src_ty, store.src, store.dest_ty
                ),
            ]);
        }
        _ => unreachable!(),
    }
    Ok(lines)
}
