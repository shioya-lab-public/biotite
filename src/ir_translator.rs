use crate::llvm_isa as ll;
use crate::llvm_isa::Prog;
use crate::riscv_isa::{Addr, FReg, Reg};
use crate::riscv_macro::regex;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::mem;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum ConstExp {
    Getelementptr {
        ty: Type,
        ptr: Ident,
        idxes: Vec<(Value, Type)>,
    },
}

impl Display for ConstExp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use ConstExp::*;

        match self {
            Getelementptr { ty, ptr, idxes } => write!(
                f,
                "getelementptr ({ty}, ptr {ptr}, {})",
                idxes
                    .iter()
                    .map(|(idx, ty)| format!("{ty} {idx}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Type {
    Void,
    Int(usize, bool),
    Float,
    Double,
    Ptr,
    Vector(usize, Box<Type>),
    Array(usize, Box<Type>),
    Struct(String),
    VarArgs(Box<Type>, Vec<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Type::*;

        match self {
            Void => write!(f, "void"),
            Int(sz, _) => write!(f, "i{sz}"),
            Float => write!(f, "float"),
            Double => write!(f, "double"),
            Ptr => write!(f, "ptr"),
            Vector(sz, ty) => write!(f, "<{sz} x {ty}>"),
            Array(sz, ty) => write!(f, "[{sz} x {ty}]"),
            Struct(name) => write!(f, "%struct.{name}"),
            VarArgs(rslt_ty, arg_tys) => {
                let mut arg_tys = arg_tys
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                if !arg_tys.is_empty() {
                    arg_tys += ", ";
                }
                write!(f, "{rslt_ty} ({arg_tys}...)")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Proto {
    fastcc: bool,
    rslt_ty: Type,
    func: Ident,
    params: Vec<(Value, Type)>,
    var_args: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Call {
    rslt: Option<Ident>,
    rslt_ty: Type,
    func: Ident,
    args: Vec<(Value, Type)>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Load {
    ty: Type,
    dest: Ident,
    src: Value,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Store {
    ty: Type,
    dest: Value,
    src: Value,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Gep {
    rslt: Ident,
    ty: Type,
    ptr: Ident,
    idxes: Vec<(Value, Type)>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Phi {
    rslt: Ident,
    ty: Type,
    vals: Vec<(Value, Ident)>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Select {
    rslt: Ident,
    cond: Value,
    ty: Type,
    op1: Value,
    op2: Value,
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
        let mut fastcc = false;
        let mut zeroext = false;
        let rslt_ty = loop {
            if let Ok(ty) = self.parse_type() {
                if let Type::Int(sz, _) = ty {
                    break Type::Int(sz, zeroext);
                } else {
                    break ty;
                }
            } else if self.assert_word("fastcc").is_ok() {
                fastcc = true;
            } else if self.assert_word("zeroext").is_ok() {
                zeroext = true;
            } else if self.assert_word("align").is_ok() && self.assert_word("(").is_err() {
                self.parse_value()?;
            } else if self.assert_word("@").is_ok() {
                return Err(());
            } else {
                self.index += self.line[self.index..]
                    .chars()
                    .position(|c| c == ' ')
                    .unwrap();
                self.skip_whitespace();
            }
        };
        let func = self.parse_ident()?;
        self.assert_word("(")?;
        let mut params = Vec::new();
        while self.assert_word("...").is_err() && self.assert_word(")").is_err() {
            let mut param_ty = self.parse_type()?;
            zeroext = false;
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
                param_ty = Type::Int(sz, zeroext);
            }
            params.push((param, param_ty));
            let _ = self.assert_word(",");
        }
        let var_args = self.assert_word(")").is_ok();
        Ok(Proto {
            fastcc,
            rslt_ty,
            func,
            params,
            var_args,
        })
    }

    pub fn parse_call(&mut self) -> Result<Call, ()> {
        self.skip_whitespace();
        let _ = self.assert_word("tail");
        let _ = self.assert_word("musttail");
        let _ = self.assert_word("notail");
        let rslt = if self.assert_word("call").is_ok() {
            None
        } else {
            let rslt = self.parse_ident()?;
            self.assert_word("=")?;
            let _ = self.assert_word("tail");
            let _ = self.assert_word("musttail");
            let _ = self.assert_word("notail");
            self.assert_word("call")?;
            Some(rslt)
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
        let ty = self.parse_type()?;
        self.assert_word(", ptr")?;
        let src = self.parse_value()?;
        Ok(Load { ty, dest, src })
    }

    pub fn parse_store(&mut self) -> Result<Store, ()> {
        self.skip_whitespace();
        self.assert_word("store")?;
        let ty = self.parse_type()?;
        let src = self.parse_value()?;
        self.assert_word(", ptr")?;
        let dest = self.parse_value()?;
        Ok(Store { ty, dest, src })
    }

    pub fn parse_gep(&mut self) -> Result<Gep, ()> {
        self.skip_whitespace();
        let rslt = self.parse_ident()?;
        self.assert_word("= getelementptr")?;
        let _ = self.assert_word("inbounds");
        let ty = self.parse_type()?;
        self.assert_word(", ptr")?;
        let ptr = self.parse_ident()?;
        self.assert_word(",")?;
        let mut idxes = Vec::new();
        while !self.is_end() {
            let idx_ty = self.parse_type()?;
            let idx = self.parse_value()?;
            idxes.push((idx, idx_ty));
            let _ = self.assert_word(",");
        }
        Ok(Gep {
            rslt,
            ty,
            ptr,
            idxes,
        })
    }

    pub fn parse_phi(&mut self) -> Result<Phi, ()> {
        self.skip_whitespace();
        let rslt = self.parse_ident()?;
        self.assert_word("= phi")?;
        let ty = self.parse_type()?;
        let mut vals = Vec::new();
        while self.assert_word("[").is_ok() {
            let val = self.parse_value()?;
            self.assert_word(",")?;
            let lb = self.parse_ident()?;
            vals.push((val, lb));
            self.assert_word("]")?;
            let _ = self.assert_word(",");
        }
        Ok(Phi { rslt, ty, vals })
    }

    pub fn parse_select(&mut self) -> Result<Select, ()> {
        self.skip_whitespace();
        let rslt = self.parse_ident()?;
        self.assert_word("= select i1")?;
        let cond = self.parse_value()?;
        self.assert_word(",")?;
        let ty = self.parse_type()?;
        let op1 = self.parse_value()?;
        self.assert_word(",")?;
        self.parse_type()?;
        let op2 = self.parse_value()?;
        Ok(Select {
            rslt,
            cond,
            ty,
            op1,
            op2,
        })
    }

    fn is_end(&self) -> bool {
        self.index >= self.line.len()
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
            r"^(-?\d+)|^(\d+\.\d+e\+\d+)|^(0x[[:xdigit:]]+)|^(null)|^(true)|^(false)|^(poison)|^(<.+?>)"
        )
        .captures(&self.line[self.index..])
        .ok_or(())?;
        let cnst = &caps[0];
        self.index += cnst.len();
        self.skip_whitespace();
        Ok(Value::Const(cnst.to_string()))
    }

    fn parse_const_exp(&mut self) -> Result<Value, ()> {
        if self.assert_word("getelementptr").is_ok() {
            let _ = self.assert_word("inbounds");
            self.assert_word("(")?;
            let ty = self.parse_type()?;
            self.assert_word(", ptr")?;
            let ptr = self.parse_ident()?;
            self.assert_word(",")?;
            let mut idxes = Vec::new();
            while self.assert_word(")").is_err() {
                let idx_ty = self.parse_type()?;
                let idx = self.parse_value()?;
                idxes.push((idx, idx_ty));
                let _ = self.assert_word(",");
            }
            Ok(Value::ConstExp(ConstExp::Getelementptr { ty, ptr, idxes }))
        } else {
            Err(())
        }
    }

    fn parse_ident(&mut self) -> Result<Ident, ()> {
        let caps = regex!(r"^[%@][-a-zA-Z$._0-9]+")
            .captures(&self.line[self.index..])
            .ok_or(())?;
        let ident = &caps[0];
        self.index += ident.len();
        self.skip_whitespace();
        if ident == "@main" {
            Ok(Ident::Global(String::from(".main")))
        } else if let Some(ident) = ident.strip_prefix('@') {
            Ok(Ident::Global(ident.to_string()))
        } else {
            Ok(Ident::Local(ident[1..].to_string()))
        }
    }

    fn parse_type(&mut self) -> Result<Type, ()> {
        let mut ty = self.parse_simple_type()?;
        if self.assert_word("(").is_ok() {
            let mut arg_tys = Vec::new();
            while self.assert_word("...").is_err() {
                arg_tys.push(self.parse_type()?);
                self.assert_word(",")?;
            }
            self.assert_word(")")?;
            ty = Type::VarArgs(Box::new(ty), arg_tys);
        }
        Ok(ty)
    }

    fn parse_simple_type(&mut self) -> Result<Type, ()> {
        if self.assert_word("void").is_ok() {
            Ok(Type::Void)
        } else if let Some(caps) = regex!(r"^i(\d+)").captures(&self.line[self.index..]) {
            let sz = &caps[1];
            self.index += sz.len() + 1;
            self.skip_whitespace();
            Ok(Type::Int(sz.parse().unwrap(), false))
        } else if self.assert_word("float").is_ok() {
            Ok(Type::Float)
        } else if self.assert_word("double").is_ok() {
            Ok(Type::Double)
        } else if self.assert_word("ptr").is_ok() {
            Ok(Type::Ptr)
        } else if self.assert_word("<").is_ok() || self.assert_word("[").is_ok() {
            let caps = regex!(r"\d+")
                .captures(&self.line[self.index..])
                .ok_or(())?;
            let sz = &caps[0];
            self.index += sz.len();
            self.assert_word(" x ")?;
            let ty = self.parse_type()?;
            if self.assert_word(">").is_ok() {
                Ok(Type::Vector(sz.parse().unwrap(), Box::new(ty)))
            } else if self.assert_word("]").is_ok() {
                Ok(Type::Array(sz.parse().unwrap(), Box::new(ty)))
            } else {
                Err(())
            }
        } else if let Ok(Ident::Local(ident)) = self.parse_ident() {
            Ok(Type::Struct(String::from(
                ident.strip_prefix("struct.").ok_or(())?,
            )))
        } else {
            Err(())
        }
    }
}

pub fn run(
    srcs: Vec<PathBuf>,
    ir_dir: PathBuf,
    symbols: &HashMap<String, Vec<Addr>>,
    prog: &Prog,
) -> Vec<String> {
    find_files(srcs, ir_dir)
        .par_iter()
        .map(|(path, output)| trans_file(path, output, symbols, prog))
        .flatten()
        .map(|ident| ident.name().to_string())
        .collect()
}

fn find_files(srcs: Vec<PathBuf>, ir_dir: PathBuf) -> Vec<(PathBuf, PathBuf)> {
    let mut files = Vec::new();
    for src in srcs {
        let mut ir_dir = ir_dir.clone();
        if src.is_dir() {
            let dir = src
                .file_name()
                .unwrap_or_else(|| panic!("Invalid src path `{src:?}`"));
            ir_dir.push(dir);
            fs::create_dir(&ir_dir).expect("Unable to create the IR directory");
            let paths = fs::read_dir(&src)
                .unwrap_or_else(|_| panic!("Cannot read `{src:?}`"))
                .map(|entry| {
                    entry
                        .unwrap_or_else(|_| panic!("Cannot read `{src:?}"))
                        .path()
                })
                .collect();
            files.extend(find_files(paths, ir_dir));
        } else if let Some("ll") = src.extension().and_then(|ext| ext.to_str()) {
            ir_dir.push(src.file_name().unwrap());
            files.push((src, ir_dir));
        }
    }
    files
}

fn get_sym_addrs<'a>(sym: &str, symbols: &'a HashMap<String, Vec<Addr>>) -> Option<&'a Vec<Addr>> {
    if sym == ".main" {
        symbols.get("main")
    } else {
        symbols.get(sym)
    }
}

fn get_first_sym_addr<'a>(sym: &str, symbols: &'a HashMap<String, Vec<Addr>>) -> Option<&'a Addr> {
    get_sym_addrs(sym, symbols).map(|addrs| &addrs[0])
}

fn trans_file(
    path: &PathBuf,
    output: &PathBuf,
    symbols: &HashMap<String, Vec<Addr>>,
    prog: &Prog,
) -> Vec<Ident> {
    Command::new("cp")
        .args([path, output])
        .status()
        .unwrap_or_else(|_| panic!("Cannot copy `{path:?}`"));
    let mut ir_funcs = Vec::new();
    let mut lines = Vec::new();
    let mut cache: Vec<String> = Vec::new();
    let mut extern_func_addrs = HashSet::new();
    let mut proto_idx = 0;
    let src = fs::read_to_string(output).unwrap();
    let fastcc_funcs = src
        .lines()
        .filter_map(|l| LineParser::new(l).parse_proto().ok())
        .filter_map(|proto| proto.fastcc.then_some(proto.func))
        .collect();
    for line in src.lines() {
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
            if let Ok((f, ls)) = trans_func(
                proto_idx,
                mem::take(&mut cache),
                symbols,
                &fastcc_funcs,
                &mut extern_func_addrs,
            ) {
                if f.name() == ".main" {
                    ir_funcs.push(Ident::Global(String::from("main")));
                } else {
                    ir_funcs.push(f);
                }
                lines.extend(ls);
            } else if let Some(true) = lines.last().map(|l| l.is_empty()) {
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
@.sp = external global i64
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
    let ir_funcs_set = ir_funcs
        .iter()
        .filter_map(|f| symbols.get(f.name()))
        .flatten()
        .cloned()
        .collect();
    let mut extern_func_addrs: Vec<_> = extern_func_addrs.difference(&ir_funcs_set).collect();
    extern_func_addrs.sort_unstable();
    if !extern_func_addrs.is_empty() {
        lines.push(String::new());
        lines.extend(
            extern_func_addrs
                .iter()
                .map(|addr| format!("declare i64 @.u{addr}(i64)")),
        );
    }
    lines.extend([
        String::new(),
        prog.build_memory(true),
        String::new(),
        prog.build_dispatchers(true).1,
        String::new(),
    ]);
    fs::write(output, lines.join("\n")).unwrap();
    ir_funcs
}

fn trans_func(
    proto_idx: usize,
    mut lines: Vec<String>,
    symbols: &HashMap<String, Vec<Addr>>,
    fastcc_funcs: &HashSet<Ident>,
    extern_func_addrs: &mut HashSet<Addr>,
) -> Result<(Ident, Vec<String>), ()> {
    let proto = LineParser::new(&lines[proto_idx]).parse_proto()?;
    lines.insert(proto_idx + 1, format!("{}:", proto.params.len()));
    let mut adaptor = Vec::new();
    if let Some(addrs) = get_sym_addrs(proto.func.name(), symbols) {
        for addr in addrs {
            adaptor.extend(trans_proto(addr, &proto)?);
            adaptor.push(String::new());
        }
    }
    let mut trans = HashMap::new();
    let mut lines = lines
        .into_iter()
        .enumerate()
        .map(|(line_no, line)| {
            if let Ok(mut call) = LineParser::new(&line).parse_call() {
                if call.func.name() == "printf" && get_first_sym_addr("printf", symbols).is_none() {
                    call.func = Ident::Global(String::from("__printf_chk"));
                    call.args
                        .insert(0, (Value::Const(String::from("1")), Type::Int(32, false)));
                }
                trans_call(line_no, &call, symbols, fastcc_funcs, extern_func_addrs)
            } else if let Ok(load) = LineParser::new(&line).parse_load() {
                trans_load(line_no, &load, symbols)
            } else if let Ok(store) = LineParser::new(&line).parse_store() {
                trans_store(line_no, &store, symbols)
            } else if let Ok(gep) = LineParser::new(&line).parse_gep() {
                trans_gep(line_no, &gep, symbols)
            } else if let Ok(phi) = LineParser::new(&line).parse_phi() {
                trans_phi(line_no, &phi, symbols, &mut trans)
            } else if let Ok(select) = LineParser::new(&line).parse_select() {
                trans_select(line_no, &select, symbols)
            } else {
                Ok(vec![line])
            }
        })
        .collect::<Result<Vec<_>, ()>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    for (lb, trans) in trans {
        let lb_line = &format!("{lb}:")[1..];
        let mut idx = lines.iter().position(|l| l.starts_with(lb_line)).unwrap();
        idx += 1;
        while LineParser::new(&lines[idx]).parse_phi().is_ok() {
            idx += 1;
        }
        lines.splice(idx..idx, trans);
    }
    adaptor.extend(lines);
    Ok((proto.func, adaptor))
}

fn trans_proto(addr: &Addr, proto: &Proto) -> Result<Vec<String>, ()> {
    let mut lines = vec![format!("define dso_local i64 @.u{addr}(i64) {{")];
    let mut regs = vec![
        ll::Value::Reg(Reg::A7),
        ll::Value::Reg(Reg::A6),
        ll::Value::Reg(Reg::A5),
        ll::Value::Reg(Reg::A4),
        ll::Value::Reg(Reg::A3),
        ll::Value::Reg(Reg::A2),
        ll::Value::Reg(Reg::A1),
        ll::Value::Reg(Reg::A0),
    ];
    let mut fregs = vec![
        ll::Value::FReg(FReg::Fa7),
        ll::Value::FReg(FReg::Fa6),
        ll::Value::FReg(FReg::Fa5),
        ll::Value::FReg(FReg::Fa4),
        ll::Value::FReg(FReg::Fa3),
        ll::Value::FReg(FReg::Fa2),
        ll::Value::FReg(FReg::Fa1),
        ll::Value::FReg(FReg::Fa0),
    ];
    let mut args = Vec::new();
    for (no, (_, ty)) in proto.params.iter().enumerate() {
        match ty {
            Type::Int(64, _) => lines.push(format!(
                "  %arg_{no} = load i64, ptr {}",
                regs.pop().ok_or(())?,
            )),
            Type::Int(sz, _) => lines.extend([
                format!("  %arg_{no}_i64 = load i64, ptr {}", regs.pop().ok_or(())?,),
                format!("  %arg_{no} = trunc i64 %arg_{no}_i64 to i{sz}"),
            ]),
            Type::Float => lines.extend([
                format!(
                    "  %arg_{no}_d = load double, ptr {}",
                    fregs.pop().ok_or(())?,
                ),
                format!("  %arg_{no} = fptrunc double %arg_{no}_d to float"),
            ]),
            Type::Double => lines.push(format!(
                "  %arg_{no} = load double, ptr {}",
                fregs.pop().ok_or(())?,
            )),
            Type::Ptr => lines.extend([
                format!("  %arg_{no}_i64 = load i64, ptr {}", regs.pop().ok_or(())?,),
                format!("  %arg_{no} = inttoptr i64 %arg_{no}_i64 to ptr"),
            ]),
            _ => Err(())?,
        }
        args.push(format!("{ty} %arg_{no}"));
    }
    if proto.var_args {
        while let Some(reg) = regs.pop() {
            lines.push(format!("  %var_arg_{reg} = load i64, ptr @.{reg}"));
            args.push(format!("i64 %var_arg_{reg}"));
        }
    }
    let call = if proto.fastcc { "call fastcc" } else { "call" };
    let f = &proto.func;
    let arg = args.join(", ");
    match &proto.rslt_ty {
        Type::Void => lines.push(format!("  {call} void {f}({arg})")),
        Type::Int(64, _) => lines.extend([
            format!("  %rslt = {call} i64 {f}({arg})"),
            String::from("  store i64 %rslt, ptr @.a0"),
        ]),
        Type::Int(sz, zeroext) => lines.extend([
            format!("  %rslt_i{sz} = {call} i{sz} {f}({arg})"),
            format!(
                "  %rslt = {} i{sz} %rslt_i{sz} to i64",
                if *zeroext { "zext" } else { "sext" }
            ),
            String::from("  store i64 %rslt, ptr @.a0"),
        ]),
        Type::Float => lines.extend([
            format!("  %rslt_f = {call} float {f}({arg})"),
            String::from("  %rslt = fpext float %rslt_f to double"),
            String::from("  store double %rslt, ptr @.fa0"),
        ]),
        Type::Double => lines.extend([
            format!("  %rslt = {call} double {f}({arg})"),
            String::from("  store double %rslt, ptr @.fa0"),
        ]),
        Type::Ptr => lines.extend([
            format!("  %rslt_ptr = {call} ptr {f}({arg})"),
            String::from("  %rslt = ptrtoint ptr %rslt_ptr to i64"),
            String::from("  store i64 %rslt, ptr @.a0"),
        ]),
        _ => Err(())?,
    }
    lines.extend([
        String::from("  %ra = load i64, ptr @.ra"),
        String::from("  ret i64 %ra"),
        String::from("}"),
    ]);
    Ok(lines)
}

fn trans_call(
    line_no: usize,
    call: &Call,
    symbols: &HashMap<String, Vec<Addr>>,
    fastcc_funcs: &HashSet<Ident>,
    extern_func_addrs: &mut HashSet<Addr>,
) -> Result<Vec<String>, ()> {
    let mut lines = Vec::new();
    if let Ident::Global(func) = &call.func {
        if get_first_sym_addr(func, symbols).is_none() {
            let mut args = Vec::new();
            for (no, (arg, ty)) in call.args.iter().enumerate() {
                if let Type::Ptr = ty {
                    lines.extend([
                        format!("  %l{line_no}_arg_{no}_i64 = ptrtoint ptr {arg} to i64"),
                        format!("  %l{line_no}_arg_{no} = call ptr @.get_mem_ptr(i64 %l{line_no}_arg_{no}_i64)"),
                    ]);
                    args.push(format!("ptr %l{line_no}_arg_{no}"));
                } else {
                    args.push(format!("{ty} {arg}"));
                }
            }
            if let Some(rslt) = &call.rslt {
                lines.push(format!(
                    "  {rslt} = call{} {} {}({})",
                    if fastcc_funcs.contains(&call.func) {
                        " fastcc"
                    } else {
                        ""
                    },
                    call.rslt_ty,
                    call.func,
                    args.join(", ")
                ));
            } else {
                lines.push(format!(
                    "  call{} {} {}({})",
                    if fastcc_funcs.contains(&call.func) {
                        " fastcc"
                    } else {
                        ""
                    },
                    call.rslt_ty,
                    call.func,
                    args.join(", ")
                ));
            }
            return Ok(lines);
        }
    }
    let mut regs = vec![
        ll::Value::Reg(Reg::A7),
        ll::Value::Reg(Reg::A6),
        ll::Value::Reg(Reg::A5),
        ll::Value::Reg(Reg::A4),
        ll::Value::Reg(Reg::A3),
        ll::Value::Reg(Reg::A2),
        ll::Value::Reg(Reg::A1),
        ll::Value::Reg(Reg::A0),
    ];
    let mut fregs = vec![
        ll::Value::FReg(FReg::Fa7),
        ll::Value::FReg(FReg::Fa6),
        ll::Value::FReg(FReg::Fa5),
        ll::Value::FReg(FReg::Fa4),
        ll::Value::FReg(FReg::Fa3),
        ll::Value::FReg(FReg::Fa2),
        ll::Value::FReg(FReg::Fa1),
        ll::Value::FReg(FReg::Fa0),
    ];
    lines.push(format!("  %l{line_no}_sp = load i64, ptr @.sp"));
    if call.args.len() > 8 {
        for i in 0..call.args.len() - 8 {
            lines.extend([
                format!("  %l{line_no}_sp_{i} = add i64 %l{line_no}_sp, {}", i * 8),
                format!("  %l{line_no}_stk_{i} = call ptr @.get_mem_ptr(i64 %l{line_no}_sp_{i})"),
            ]);
        }
    }
    let mut stk = -1;
    let mut get_loc = |is_fp| {
        if is_fp && fregs.is_empty() || !is_fp && regs.is_empty() {
            stk += 1;
            format!("%l{line_no}_stk_{stk}")
        } else if is_fp {
            fregs.pop().unwrap().to_string()
        } else {
            regs.pop().unwrap().to_string()
        }
    };
    for (no, (arg, ty)) in call.args.iter().enumerate() {
        match ty {
            Type::Int(64, _) => lines.push(format!("  store i64 {arg}, ptr {}", get_loc(false))),
            Type::Int(sz, zeroext) => lines.extend([
                format!(
                    "  %l{line_no}_arg_{no} = {} i{sz} {arg} to i64",
                    if *zeroext { "zext" } else { "sext" }
                ),
                format!("  store i64 %l{line_no}_arg_{no}, ptr {}", get_loc(false)),
            ]),
            Type::Float => {
                if let Type::VarArgs(_, arg_tys) = &call.rslt_ty {
                    if arg_tys.len() <= no {
                        lines.extend([
                            format!("  %l{line_no}_arg_{no}_d = fpext float {arg} to double"),
                            format!("  %l{line_no}_arg_{no} = bitcast double %l{line_no}_arg_{no}_d to i64"),
                            format!("  store i64 %l{line_no}_arg_{no}, ptr {}", get_loc(false)),
                        ]);
                        continue;
                    }
                }
                lines.extend([
                    format!("  %l{line_no}_arg_{no} = fpext float {arg} to double"),
                    format!("  store double %l{line_no}_arg_{no}, ptr {}", get_loc(true)),
                ]);
            }
            Type::Double => {
                if let Type::VarArgs(_, arg_tys) = &call.rslt_ty {
                    if arg_tys.len() <= no {
                        lines.extend([
                            format!("  %l{line_no}_arg_{no} = bitcast double {arg} to i64"),
                            format!("  store i64 %l{line_no}_arg_{no}, ptr {}", get_loc(false)),
                        ]);
                        continue;
                    }
                }
                lines.push(format!("  store double {arg}, ptr {}", get_loc(true)));
            }
            Type::Ptr => {
                if let Value::Ident(Ident::Global(arg)) = arg {
                    if let Some(addr) = get_first_sym_addr(arg, symbols) {
                        lines.extend([
                            format!("  %l{line_no}_arg_{no}_ptr = call ptr @.get_mem_ptr(i64 u{addr})"),
                            format!("  %l{line_no}_arg_{no} = ptrtoint ptr %l{line_no}_arg_{no}_ptr to i64"),
                            format!("  store i64 %l{line_no}_arg_{no}, ptr {}", get_loc(false),),
                        ]);
                        continue;
                    }
                }
                lines.extend([
                    format!("  %l{line_no}_arg_{no} = ptrtoint ptr {arg} to i64"),
                    format!("  store i64 %l{line_no}_arg_{no}, ptr {}", get_loc(false)),
                ]);
            }
            _ => Err(())?,
        }
    }
    match &call.func {
        Ident::Global(func) => {
            let addr = get_first_sym_addr(func, symbols).unwrap();
            extern_func_addrs.insert(*addr);
            lines.push(format!(
                "  %l{line_no}_ra = call i64 @.u{addr}(i64 u{addr})"
            ));
        }
        Ident::Local(func) => lines.extend([
            format!("  %l{line_no}_func = ptrtoint ptr %{func} to i64"),
            format!("  %l{line_no}_ra = call i64 @.disp_func(i64 %l{line_no}_func)"),
        ]),
    }
    let rslt_ty = match &call.rslt_ty {
        Type::VarArgs(rslt_ty, _) => rslt_ty,
        rslt_ty => rslt_ty,
    };
    match rslt_ty {
        Type::Void => (),
        Type::Int(64, _) => lines.push(format!(
            "  {} = load i64, ptr @.a0",
            call.rslt.as_ref().unwrap()
        )),
        Type::Int(sz, _) => lines.extend([
            format!("  %l{line_no}_rslt = load i64, ptr @.a0"),
            format!(
                "  {} = trunc i64 %l{line_no}_rslt to i{sz}",
                call.rslt.as_ref().unwrap()
            ),
        ]),
        Type::Float => lines.extend([
            format!("  %l{line_no}_rslt = load double, ptr @.fa0"),
            format!(
                "  {} = fptrunc double %l{line_no}_rslt to float",
                call.rslt.as_ref().unwrap()
            ),
        ]),
        Type::Double => lines.push(format!(
            "  {} = load double, ptr @.fa0",
            call.rslt.as_ref().unwrap()
        )),
        Type::Ptr => lines.extend([
            format!("  %l{line_no}_rslt = load i64, ptr @.a0"),
            format!(
                "  {} = inttoptr i64 %l{line_no}_rslt to ptr",
                call.rslt.as_ref().unwrap()
            ),
        ]),
        _ => Err(())?,
    }
    Ok(lines)
}

fn trans_load(
    line_no: usize,
    load: &Load,
    symbols: &HashMap<String, Vec<Addr>>,
) -> Result<Vec<String>, ()> {
    let mut lines = Vec::new();
    let src = match &load.src {
        Value::Ident(ident) => ident,
        Value::ConstExp(ConstExp::Getelementptr { ptr, .. }) => ptr,
        _ => Err(())?,
    };
    match src {
        Ident::Global(src) => lines.push(format!(
            "  %l{line_no}_src = call ptr @.get_mem_ptr(i64 u{})",
            get_first_sym_addr(src, symbols).ok_or(())?,
        )),
        Ident::Local(src) => lines.extend([
            format!("  %l{line_no}_src_i64 = ptrtoint ptr %{src} to i64"),
            format!("  %l{line_no}_src = call ptr @.get_mem_ptr(i64 %l{line_no}_src_i64)"),
        ]),
    }
    match &load.src {
        Value::Ident(_) => lines.push(format!(
            "  {} = load {}, ptr %l{line_no}_src, align 1",
            load.dest, load.ty
        )),
        Value::ConstExp(ConstExp::Getelementptr { ty, idxes, .. }) => {
            let idxes = idxes
                .iter()
                .map(|(idx, ty)| format!("{ty} {idx}"))
                .collect::<Vec<_>>()
                .join(", ");
            lines.extend([
                format!("  %l{line_no}_gep = getelementptr {ty}, ptr %l{line_no}_src, {idxes}"),
                format!(
                    "  {} = load {}, ptr %l{line_no}_gep, align 1",
                    load.dest, load.ty
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
    symbols: &HashMap<String, Vec<Addr>>,
) -> Result<Vec<String>, ()> {
    let mut lines = Vec::new();
    let src = (|| {
        if let Value::Ident(Ident::Global(src)) = &store.src {
            if let Some(addr) = get_first_sym_addr(src, symbols) {
                lines.push(format!(
                    "  %l{line_no}_{src} = call ptr @.get_mem_ptr(i64 u{addr})"
                ));
                return format!("%l{line_no}_{src}");
            }
        }
        store.src.to_string()
    })();
    let dest = match &store.dest {
        Value::Ident(ident) => ident,
        Value::ConstExp(ConstExp::Getelementptr { ptr, .. }) => ptr,
        _ => Err(())?,
    };
    match dest {
        Ident::Global(dest) => lines.push(format!(
            "  %l{line_no}_dest = call ptr @.get_mem_ptr(i64 u{})",
            get_first_sym_addr(dest, symbols).ok_or(())?,
        )),
        Ident::Local(dest) => lines.extend([
            format!("  %l{line_no}_dest_i64 = ptrtoint ptr %{dest} to i64"),
            format!("  %l{line_no}_dest = call ptr @.get_mem_ptr(i64 %l{line_no}_dest_i64)"),
        ]),
    }
    match &store.dest {
        Value::Ident(_) => lines.push(format!(
            "  store {} {src}, ptr %l{line_no}_dest, align 1",
            store.ty
        )),
        Value::ConstExp(ConstExp::Getelementptr { ty, idxes, .. }) => {
            let idxes = idxes
                .iter()
                .map(|(idx, ty)| format!("{ty} {idx}"))
                .collect::<Vec<_>>()
                .join(", ");
            lines.extend([
                format!("  %l{line_no}_gep = getelementptr {ty}, ptr %l{line_no}_dest, {idxes}"),
                format!("  store {} {src}, ptr %l{line_no}_gep, align 1", store.ty),
            ]);
        }
        _ => unreachable!(),
    }
    Ok(lines)
}

fn trans_gep(
    line_no: usize,
    gep: &Gep,
    symbols: &HashMap<String, Vec<Addr>>,
) -> Result<Vec<String>, ()> {
    let idxes = gep
        .idxes
        .iter()
        .map(|(idx, ty)| format!("{ty} {idx}"))
        .collect::<Vec<_>>()
        .join(", ");
    if let Ident::Global(ptr) = &gep.ptr {
        if let Some(addr) = get_first_sym_addr(ptr, symbols) {
            return Ok(vec![
                format!("  %l{line_no}_ptr = call ptr @.get_mem_ptr(i64 u{addr})"),
                format!(
                    "  {} = getelementptr {}, ptr %l{line_no}_ptr, {idxes}",
                    gep.rslt, gep.ty
                ),
            ]);
        }
    }
    Ok(vec![format!(
        "  {} = getelementptr {}, ptr {}, {}",
        gep.rslt, gep.ty, gep.ptr, idxes
    )])
}

fn trans_phi(
    line_no: usize,
    phi: &Phi,
    symbols: &HashMap<String, Vec<Addr>>,
    trans: &mut HashMap<Ident, Vec<String>>,
) -> Result<Vec<String>, ()> {
    let mut lines = Vec::new();
    let vals = phi
        .vals
        .iter()
        .enumerate()
        .map(|(i, (val, lb))| {
            if let Value::Ident(Ident::Global(val)) = val {
                if let Some(addr) = get_first_sym_addr(val, symbols) {
                    trans.entry(lb.clone()).or_default().push(format!("  %l{line_no}_{val}_{i} = call ptr @.get_mem_ptr(i64 u{addr})"));
                    return format!("[ %l{line_no}_{val}_{i}, {lb} ]");
                }
            }
            if let Value::ConstExp(ConstExp::Getelementptr { ty, ptr: Ident::Global(ptr), idxes }) = val {
                if let Some(addr) = get_first_sym_addr(ptr, symbols) {
                    let idxes = idxes
                        .iter()
                        .map(|(idx, ty)| format!("{ty} {idx}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    trans.entry(lb.clone()).or_default().extend([
                        format!("  %l{line_no}_gep_{i}_ptr = call ptr @.get_mem_ptr(i64 u{addr})"),
                        format!("  %l{line_no}_gep_{i} = getelementptr {ty}, ptr %l{line_no}_gep_{i}_ptr, {idxes}"),
                    ]);
                    return format!("[ %l{line_no}_gep_{i}, {lb} ]");
                }
            }
            format!("[ {val}, {lb} ]")
        })
        .collect::<Vec<_>>()
        .join(", ");
    lines.push(format!("  {} = phi {} {vals}", phi.rslt, phi.ty));
    Ok(lines)
}

fn trans_select(
    line_no: usize,
    select: &Select,
    symbols: &HashMap<String, Vec<Addr>>,
) -> Result<Vec<String>, ()> {
    let mut lines = Vec::new();
    let ops = [&select.op1, &select.op2]
        .iter()
        .enumerate()
        .map(|(i, op)| {
            if let Value::Ident(Ident::Global(op)) = op {
                if let Some(addr) = get_first_sym_addr(op, symbols) {
                    lines.push(format!(
                        "  %l{line_no}_{op} = call ptr @.get_mem_ptr(i64 u{addr})"
                    ));
                    return format!("%l{line_no}_{op}");
                }
            }
            if let Value::ConstExp(ConstExp::Getelementptr { ty, ptr: Ident::Global(ptr), idxes }) = op {
                if let Some(addr) = get_first_sym_addr(ptr, symbols) {
                    let idxes = idxes
                        .iter()
                        .map(|(idx, ty)| format!("{ty} {idx}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    lines.extend([
                        format!("  %l{line_no}_gep_{i}_ptr = call ptr @.get_mem_ptr(i64 u{addr})"),
                        format!("  %l{line_no}_gep_{i} = getelementptr {ty}, ptr %l{line_no}_gep_{i}_ptr, {idxes}"),
                    ]);
                    return format!("%l{line_no}_gep_{i}");
                }
            }
            op.to_string()
        })
        .collect::<Vec<_>>();
    lines.push(format!(
        "  {} = select i1 {}, {} {}, {} {}",
        select.rslt, select.cond, select.ty, ops[0], select.ty, ops[1]
    ));
    Ok(lines)
}
