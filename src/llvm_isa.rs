//! LLVM-related definitions.

use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;
use indoc::{formatdoc, writedoc};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prog {
    pub entry: Value,
    pub image: Vec<u8>,
    pub sp: Value,
    pub phdr: Value,
    pub tdata: (Value, usize),
    pub funcs: Vec<Func>,
    pub ir_funcs: HashSet<String>,
    pub func_syms: HashSet<Value>,
    pub native_mem_utils: &'static str,
    pub sys_call: String,
    pub mem: Option<(String, String)>,
    pub module_size: usize,
}

impl Prog {
    /// Splits translated functions into multiple modules for faster recompilation.
    pub fn to_modules(&self) -> (String, String, Vec<String>) {
        let (disp_len, disps) = self.build_dispatchers(false);
        let main = format!(
            include_str!("templates/main.ll"),
            func_decls = self
                .funcs
                .iter()
                .map(|f| f.to_declaration())
                .collect::<Vec<_>>()
                .join("\n"),
            sp = self.sp,
            phdr = self.phdr,
            tdata_addr = self.tdata.0,
            tdata_len = self.tdata.1,
            entry = self.entry,
            disp_len = disp_len,
            native_mem_utils = self.native_mem_utils,
            mem = self.build_memory(false),
            disps = disps,
            rounding_funcs = Self::build_rounding_funcs(),
            defs = include_str!("templates/defs.ll"),
            sys_call = self.sys_call,
        );
        let module_size = if self.module_size == 0 {
            self.funcs.len()
        } else {
            self.module_size
        };
        let mods = self.funcs[..]
            .chunks(module_size)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|funcs| {
                let funcs_set = HashSet::<_>::from_iter(funcs);
                let funcs = self
                    .funcs
                    .iter()
                    .map(|func| {
                        if self.ir_funcs.contains(&func.symbol) && !func.is_fallback
                            || !funcs_set.contains(func)
                        {
                            func.to_declaration()
                        } else {
                            func.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");
                let mem = self.build_memory(true);
                let disps = self.build_dispatchers(true).1;
                let decls = include_str!("templates/decls.ll");
                [&funcs, &mem, &disps, decls, self.native_mem_utils].join("\n\n")
            })
            .collect::<Vec<_>>();
        let objs = (0..mods.len())
            .map(|i| format!("{i}.o"))
            .collect::<Vec<_>>()
            .join(" ");
        let mk = format!(include_str!("templates/Makefile"), objs = &objs);
        (mk, main, mods)
    }

    pub fn build_memory(&self, external: bool) -> String {
        if self.mem.is_some() {
            formatdoc!(
                "
                define internal ptr @.get_mem_ptr(i64 %addr) alwaysinline {{
                  %ptr = inttoptr i64 %addr to ptr
                  ret ptr %ptr
                }}",
            )
        } else {
            let get_mem_ptr = formatdoc!(
                "
                define internal ptr @.get_mem_ptr(i64 %addr) alwaysinline {{
                  %is_static = icmp ugt i64 {size}, %addr
                  br i1 %is_static, label %static, label %dynamic
                static:
                  %static_ptr = getelementptr [{size} x i8], ptr @.image, i64 0, i64 %addr
                  ret ptr %static_ptr
                dynamic:
                  %dynamic_ptr = inttoptr i64 %addr to ptr
                  ret ptr %dynamic_ptr
                }}",
                size = self.image.len(),
            );
            if external {
                formatdoc!(
                    "
                    @.image = external global [{size} x i8]

                    {get_mem_ptr}",
                    size = self.image.len(),
                )
            } else {
                formatdoc!(
                    "
                    @.image = global [{size} x i8] [{image}]

                    {get_mem_ptr}",
                    size = self.image.len(),
                    image = self
                        .image
                        .iter()
                        .map(|b| format!("i8 {b}"))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }
        }
    }

    pub fn build_dispatchers(&self, external: bool) -> (usize, String) {
        let mut disp = Vec::new();
        let mut func_disp = Vec::new();
        for func in &self.funcs {
            if func.is_fallback {
                continue;
            }
            let last_rv_inst = &func.inst_blocks.last().unwrap().rv_inst;
            let rv::Addr(mut end) = last_rv_inst.address();
            end += if last_rv_inst.is_compressed() { 2 } else { 4 };
            disp.resize(end as usize, String::from("i64 0"));
            func_disp.resize(end as usize, String::from("i64 0"));
            let ptr = format!("i64 ptrtoint (ptr @.{} to i64)", func.address);
            let fallback_ptr = format!(
                "i64 ptrtoint (ptr @.{}{} to i64)",
                func.address,
                if !func.is_opaque { "_fallback" } else { "" }
            );
            for inst_block in &func.inst_blocks {
                let rv::Addr(addr) = inst_block.rv_inst.address();
                disp[addr as usize] = fallback_ptr.clone();
            }
            let rv::Addr(addr) = func.inst_blocks[0].rv_inst.address();
            func_disp[addr as usize] = ptr;
        }
        let size = disp.len();
        let find_func = formatdoc!(
            "
            define internal i64 @.find_func(i64 %addr) alwaysinline {{
              %func_addr_ptr = getelementptr [{size} x i64], ptr @.func_disp, i64 0, i64 %addr
              %func_addr = load i64, ptr %func_addr_ptr
              %is_func = icmp ne i64 %func_addr, 0
              br i1 %is_func, label %call, label %ret
            call:
              %func = inttoptr i64 %func_addr to ptr
              %rslt = call i64 %func(i64 %addr)
              ret i64 %rslt
            ret:
              ret i64 0
            }}"
        );
        if external {
            (
                size,
                formatdoc!(
                    "
                    @.disp = external global [{size} x i64]
                    @.func_disp = external global [{size} x i64]

                    {find_func}"
                ),
            )
        } else {
            (
                size,
                formatdoc!(
                    "
                    @.disp = global [{size} x i64] [{disp}]
                    @.func_disp = global [{size} x i64] [{func_disp}]

                    {find_func}",
                    disp = disp.join(", "),
                    func_disp = func_disp.join(", "),
                ),
            )
        }
    }

    fn build_rounding_funcs() -> String {
        [
            ("float", "i32", "fptosi", "sitofp"),
            ("float", "i32", "fptoui", "uitofp"),
            ("float", "i64", "fptosi", "sitofp"),
            ("float", "i64", "fptoui", "uitofp"),
            ("double", "i32", "fptosi", "sitofp"),
            ("double", "i32", "fptoui", "uitofp"),
            ("double", "i64", "fptosi", "sitofp"),
            ("double", "i64", "fptoui", "uitofp"),
        ]
        .iter()
        .map(|(fp, int, fptoint, inttofp)| {
            format!(
                include_str!("templates/rounding.ll"),
                fp = fp,
                int = int,
                fptoint = fptoint,
                inttofp = inttofp,
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Func {
    pub section: String,
    pub symbol: String,
    pub address: Value,
    pub inst_blocks: Vec<InstBlock>,
    pub is_opaque: bool,
    pub is_fallback: bool,
    pub used_regs: Vec<rv::Reg>,
    pub used_fregs: Vec<rv::FReg>,
    pub synced_regs: Vec<rv::Reg>,
    pub synced_fregs: Vec<rv::FReg>,
}

impl Func {
    pub fn to_declaration(&self) -> String {
        format!(
            "declare i64 @.{addr}{fallback}(i64)",
            addr = self.address,
            fallback = if self.is_fallback { "_fallback" } else { "" },
        )
    }

    pub fn build_stack_loading(regs: &[rv::Reg], fregs: &[rv::FReg], prefix: &str) -> String {
        let regs = regs
            .iter()
            .map(|reg| {
                format!(
                    "  %{prefix}_{reg}_val = load i64, ptr {global}
  store i64 %{prefix}_{reg}_val, ptr {stack}",
                    global = Value::Reg(*reg),
                    stack = Value::StkReg(*reg),
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let fregs = fregs
            .iter()
            .map(|freg| {
                format!(
                    "  %{prefix}_{freg}_val = load double, ptr {global}
  store double %{prefix}_{freg}_val, ptr {stack}",
                    global = Value::FReg(*freg),
                    stack = Value::StkFReg(*freg),
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        match (regs.is_empty(), fregs.is_empty()) {
            (true, true) => String::new(),
            (true, false) => fregs[2..].to_string(),
            (false, true) => regs[2..].to_string(),
            (false, false) => format!("{regs}\n{fregs}", regs = &regs[2..]),
        }
    }

    pub fn build_stack_storing(regs: &[rv::Reg], fregs: &[rv::FReg], prefix: &str) -> String {
        let regs = regs
            .iter()
            .map(|reg| {
                format!(
                    "  %{prefix}_{reg}_val = load i64, ptr {stack}
  store i64 %{prefix}_{reg}_val, ptr {global}",
                    stack = Value::StkReg(*reg),
                    global = Value::Reg(*reg),
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let fregs = fregs
            .iter()
            .map(|freg| {
                format!(
                    "  %{prefix}_{freg}_val = load double, ptr {stack}
  store double %{prefix}_{freg}_val, ptr {global}",
                    stack = Value::StkFReg(*freg),
                    global = Value::FReg(*freg),
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        match (regs.is_empty(), fregs.is_empty()) {
            (true, true) => String::new(),
            (true, false) => fregs[2..].to_string(),
            (false, true) => regs[2..].to_string(),
            (false, false) => format!("{regs}\n{fregs}", regs = &regs[2..]),
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let stack_regs = self
            .used_regs
            .iter()
            .map(|reg| format!("  {reg} = alloca i64", reg = Value::StkReg(*reg)))
            .collect::<Vec<_>>()
            .join("\n");
        let stack_fregs = self
            .used_fregs
            .iter()
            .map(|freg| format!("  {freg} = alloca double", freg = Value::StkFReg(*freg)))
            .collect::<Vec<_>>()
            .join("\n");
        let stack_loading =
            Self::build_stack_loading(&self.synced_regs, &self.synced_fregs, "entry");
        let local_disp = if self.is_opaque {
            let disp = self
                .inst_blocks
                .iter()
                .map(|inst_block| {
                    let addr = Value::Addr(inst_block.rv_inst.address());
                    format!("i64 {addr}, label %{addr}")
                })
                .collect::<Vec<_>>()
                .join(" ");
            let stack_storing =
                Self::build_stack_storing(&self.used_regs, &self.used_fregs, "disp_s");
            let stack_loading =
                Self::build_stack_loading(&self.used_regs, &self.used_fregs, "disp_l");
            formatdoc!(
                "
                  store i64 %entry, ptr %entry_ptr
                  %local_jalr_ptr = alloca i1, i1 0
                  br label %disp
                disp:
                  %addr = load i64, ptr %entry_ptr
                  switch i64 %addr, label %func_disp [{disp}]
                func_disp:
                  {stack_storing}
                  %ra = call i64 @.find_func(i64 %addr)
                  {stack_loading}
                  %fail = icmp eq i64 %ra, 0
                  br i1 %fail, label %ret, label %cont
                cont:
                  store i64 %ra, ptr %entry_ptr
                  br label %disp"
            )
        } else {
            let addr = Value::Addr(self.inst_blocks[0].rv_inst.address());
            format!("  br label %{addr}")
        };
        let inst_blocks = self
            .inst_blocks
            .iter()
            .map(|block| block.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let last_rv_inst = &self.inst_blocks.last().unwrap().rv_inst;
        let next_pc = next_pc!(
            next_pc,
            last_rv_inst.address(),
            last_rv_inst.is_compressed()
        );
        let stack_storing = Self::build_stack_storing(&self.used_regs, &self.used_fregs, "ret");
        writedoc!(
            f,
            "
            ; {addr} {sec} <{sym}>
            define i64 @.{addr}{fallback}(i64 %entry) {{
              %entry_ptr = alloca i64
            {stack_regs}
            {stack_fregs}
              {stack_loading}
            {local_disp}

            {inst_blocks}

            {next_pc}:
              store i64 {next_pc}, ptr %entry_ptr
              br label %ret
            ret:
              {stack_storing}
              %ret_addr = load i64, ptr %entry_ptr
              ret i64 %ret_addr
            }}",
            addr = self.address,
            sec = self.section,
            sym = self.symbol,
            fallback = if self.is_fallback { "_fallback" } else { "" },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct InstBlock {
    pub rv_inst: rv::Inst,
    pub insts: Vec<Inst>,
}

impl Display for InstBlock {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let insts = self
            .insts
            .iter()
            .map(|inst| format!("  {inst}"))
            .collect::<Vec<_>>()
            .join("\n");
        let cont = if matches!(
            self.insts.last(),
            Some(Inst::Ret { .. })
                | Some(Inst::Br { .. })
                | Some(Inst::Call { .. })
                | Some(Inst::Conbr { .. })
                | Some(Inst::Checkret { .. })
                | Some(Inst::Dispfunc { .. })
        ) {
            String::new()
        } else {
            let br = Inst::Br {
                addr: next_pc!(
                    next_pc,
                    self.rv_inst.address(),
                    self.rv_inst.is_compressed()
                ),
            };
            format!("  {br}")
        };
        writedoc!(
            f,
            "
            ; {rv_inst:?}
            {addr}:
            {insts}
            {cont}",
            rv_inst = self.rv_inst,
            addr = Value::Addr(self.rv_inst.address()),
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Inst {
    // Terminator Instructions
    Ret {
        val: Value,
    },
    Br {
        addr: Value,
    },
    Conbr {
        cond: Value,
        iftrue: Value,
        iffalse: Value,
    },
    Unreachable,

    // Unary Operations
    Fneg {
        rslt: Value,
        ty: Type,
        op: Value,
    },

    // Binary Operations
    Add {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fadd {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Sub {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fsub {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Mul {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fmul {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Udiv {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Sdiv {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fdiv {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Urem {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Srem {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },

    // Bitwise Binary Operations
    Shl {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Lshr {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Ashr {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    And {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Or {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Xor {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },

    // Aggregate Operations
    Extractvalue {
        rslt: Value,
        ty: Type,
        val: Value,
        idx: Value,
    },

    // Memory Access and Addressing Operations
    Load {
        rslt: Value,
        ty: Type,
        ptr: Value,
    },
    Store {
        ty: Type,
        val: Value,
        ptr: Value,
    },
    Fence,
    Cmpxchg {
        rslt: Value,
        ty: Type,
        ptr: Value,
        cmp: Value,
        new: Value,
        mo: Mo,
    },
    Atomicrmw {
        rslt: Value,
        op: Op,
        ty: Type,
        ptr: Value,
        val: Value,
        mo: Mo,
    },
    Getelementptr {
        rslt: Value,
        ptr: Value,
        offset: Value,
    },

    // Conversion Operations
    Trunc {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Zext {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Sext {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Fptrunc {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Fpext {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Fptoui {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
        rm: Rm,
    },
    Fptosi {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
        rm: Rm,
    },
    Uitofp {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Sitofp {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Bitcast {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },

    // Other Operations
    Icmp {
        rslt: Value,
        cond: Cond,
        op1: Value,
        op2: Value,
    },
    Fcmp {
        rslt: Value,
        fcond: FCond,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Select {
        rslt: Value,
        cond: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Call {
        rslt: Value,
        target: Value,
        next_pc: Value,
        used_regs: Vec<rv::Reg>,
        used_fregs: Vec<rv::FReg>,
    },

    // Standard C/C++ Library Intrinsics
    Sqrt {
        rslt: Value,
        ty: Type,
        arg: Value,
    },
    Fma {
        rslt: Value,
        ty: Type,
        arg1: Value,
        arg2: Value,
        arg3: Value,
    },
    Fabs {
        rslt: Value,
        ty: Type,
        arg: Value,
    },
    Copysign {
        rslt: Value,
        ty: Type,
        mag: Value,
        sgn: Value,
    },

    // Misc
    Getmemptr {
        rslt: Value,
        addr: Value,
    },
    Syscall {
        rslt: Value,
        nr: Value,
        arg1: Value,
        arg2: Value,
        arg3: Value,
        arg4: Value,
        arg5: Value,
        arg6: Value,
    },
    Checkret {
        addr: Value,
        stk: bool,
    },
    Dispfunc {
        addr: Value,
        target: Value,
        used_regs: Vec<rv::Reg>,
        used_fregs: Vec<rv::FReg>,
    },
    Memcmp {
        addr: Value,
        stk: bool,
    },
    Memset {
        addr: Value,
        stk: bool,
    },
    Memcpy {
        addr: Value,
        stk: bool,
    },
    Memmove {
        addr: Value,
        stk: bool,
    },
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Inst::*;

        match self {
            // Terminator Instructions
            Ret { val } => writedoc!(f, "
                store i64 {val}, ptr %entry_ptr
                  br label %ret"),
            Br { addr } => write!(f, "br label %{addr}"),
            Conbr { cond, iftrue, iffalse } => write!(f, "br i1 {cond}, label %{iftrue}, label %{iffalse}"),
            Unreachable => write!(f, "unreachable"),

            // Unary Operations
            Fneg { rslt, ty, op } => write!(f, "{rslt} = fneg {ty} {op}"),

            // Binary Operations
            Add { rslt, ty, op1, op2 } => write!(f, "{rslt} = add {ty} {op1}, {op2}"),
            Fadd { rslt, ty, op1, op2 } => write!(f, "{rslt} = fadd {ty} {op1}, {op2}"),
            Sub { rslt, ty, op1, op2 } => write!(f, "{rslt} = sub {ty} {op1}, {op2}"),
            Fsub { rslt, ty, op1, op2 } => write!(f, "{rslt} = fsub {ty} {op1}, {op2}"),
            Mul { rslt, ty, op1, op2 } => write!(f, "{rslt} = mul {ty} {op1}, {op2}"),
            Fmul { rslt, ty, op1, op2 } => write!(f, "{rslt} = fmul {ty} {op1}, {op2}"),
            Udiv { rslt, ty, op1, op2 } => write!(f, "{rslt} = udiv {ty} {op1}, {op2}"),
            Sdiv { rslt, ty, op1, op2 } => write!(f, "{rslt} = sdiv {ty} {op1}, {op2}"),
            Fdiv { rslt, ty, op1, op2 } => write!(f, "{rslt} = fdiv {ty} {op1}, {op2}"),
            Urem { rslt, ty, op1, op2 } => write!(f, "{rslt} = urem {ty} {op1}, {op2}"),
            Srem { rslt, ty, op1, op2 } => write!(f, "{rslt} = srem {ty} {op1}, {op2}"),

            // Bitwise Binary Operations
            Shl { rslt, ty, op1, op2 } => write!(f, "{rslt} = shl {ty} {op1}, {op2}"),
            Lshr { rslt, ty, op1, op2 } => write!(f, "{rslt} = lshr {ty} {op1}, {op2}"),
            Ashr { rslt, ty, op1, op2 } => write!(f, "{rslt} = ashr {ty} {op1}, {op2}"),
            And { rslt, ty, op1, op2 } => write!(f, "{rslt} = and {ty} {op1}, {op2}"),
            Or { rslt, ty, op1, op2 } => write!(f, "{rslt} = or {ty} {op1}, {op2}"),
            Xor { rslt, ty, op1, op2 } => write!(f, "{rslt} = xor {ty} {op1}, {op2}"),

            // Aggregate Operations
            Extractvalue { rslt, ty, val, idx } => write!(f, "{rslt} = extractvalue {{{ty}, i1}} {val}, {idx}"),

            // Memory Access and Addressing Operations
            Load { rslt, ty, ptr } => write!(f, "{rslt} = load {ty}, ptr {ptr}"),
            Store { ty, val, ptr } => write!(f, "store {ty} {val}, ptr {ptr}"),
            Fence => write!(f, "fence seq_cst"),
            Cmpxchg { rslt, ty, ptr, cmp, new, mo } => write!(f, "{rslt} = cmpxchg ptr {ptr}, {ty} {cmp}, {ty} {new} {mo} monotonic"),
            Atomicrmw { rslt, op, ty, ptr, val, mo } => write!(f, "{rslt} = atomicrmw {op} ptr {ptr}, {ty} {val} {mo}"),
            Getelementptr { rslt, ptr, offset } => write!(f, "{rslt} = getelementptr i8, ptr {ptr}, i64 {offset}"),

            // Conversion Operations
            Trunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = trunc {ty1} {val} to {ty2}"),
            Zext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = zext {ty1} {val} to {ty2}"),
            Sext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sext {ty1} {val} to {ty2}"),
            Fptrunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fptrunc {ty1} {val} to {ty2}"),
            Fpext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fpext {ty1} {val} to {ty2}"),
            Fptoui { rslt, ty1, val, ty2, rm } => match rm {
                Rm::Downward => write!(f, "{rslt} = call {ty2} @.round_{ty1}_{ty2}_fptoui({ty1} {val}, i1 1)"),
                Rm::Upward => write!(f, "{rslt} = call {ty2} @.round_{ty1}_{ty2}_fptoui({ty1} {val}, i1 0)"),
                _ => write!(f, "{rslt} = fptoui {ty1} {val} to {ty2}"),
            }
            Fptosi { rslt, ty1, val, ty2, rm } => match rm {
                Rm::Downward => write!(f, "{rslt} = call {ty2} @.round_{ty1}_{ty2}_fptosi({ty1} {val}, i1 1)"),
                Rm::Upward => write!(f, "{rslt} = call {ty2} @.round_{ty1}_{ty2}_fptosi({ty1} {val}, i1 0)"),
                _ => write!(f, "{rslt} = fptosi {ty1} {val} to {ty2}"),
            }
            Uitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = uitofp {ty1} {val} to {ty2}"),
            Sitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sitofp {ty1} {val} to {ty2}"),
            Bitcast { rslt, ty1, val, ty2 } => write!(f, "{rslt} = bitcast {ty1} {val} to {ty2}"),

            // Other Operations
            Icmp { rslt, cond, op1, op2 } => write!(f, "{rslt} = icmp {cond} i64 {op1}, {op2}"),
            Fcmp {rslt,fcond, ty,op1,op2} => write!(f, "{rslt} = fcmp {fcond} {ty} {op1}, {op2}"),
            Select {rslt,cond, ty, op1,op2} => write!(f, "{rslt} = select i1 {cond}, {ty} {op1}, {ty} {op2}"),
            Call { rslt, target, next_pc, used_regs, used_fregs } => {
                let stack_storing = Func::build_stack_storing(used_regs, used_fregs, &format!("{}_s", &rslt.to_string()[1..]));
                let stack_loading = Func::build_stack_loading(used_regs, used_fregs, &format!("{}_l", &rslt.to_string()[1..]));
                writedoc!(f, "
                    {stack_storing}
                      {rslt} = call i64 @.{target}(i64 {target})
                      {stack_loading}
                      store i64 {rslt}, ptr %entry_ptr
                      {rslt}_is_next_pc = icmp eq i64 {rslt}, {next_pc}
                      br i1 {rslt}_is_next_pc, label %{next_pc}, label %ret")
            }

            // Standard C/C++ Library Intrinsics
            Sqrt { rslt, ty, arg } => write!(f,"{rslt} = call {ty} @llvm.sqrt.{ty}({ty} {arg})"),
            Fma { rslt, ty, arg1, arg2, arg3 } => write!(f, "{rslt} = call {ty} @llvm.fma.{ty}({ty} {arg1}, {ty} {arg2}, {ty} {arg3})"),
            Fabs { rslt, ty, arg } => write!(f, "{rslt} = call {ty} @llvm.fabs.{ty}({ty} {arg})"),
            Copysign { rslt, ty, mag, sgn } => write!(f, "{rslt} = call {ty} @llvm.copysign.{ty}({ty} {mag}, {ty} {sgn})"),

            // Misc
            Getmemptr { rslt, addr } => write!(f, "{rslt} = call ptr @.get_mem_ptr(i64 {addr})"),
            Syscall { rslt, nr, arg1, arg2, arg3, arg4, arg5, arg6 } =>
                write!(f, "{rslt} = call i64 (i64, i64, i64, i64, i64, i64, i64) @.sys_call(i64 {nr}, i64 {arg1}, i64 {arg2}, i64 {arg3}, i64 {arg4}, i64 {arg5}, i64 {arg6})"),
            Checkret { addr , stk} => writedoc!(f, "
                %{addr}_0 = load i64, ptr {ra}
                  store i64 %{addr}_0, ptr %entry_ptr
                  %{addr}_1 = load i1, ptr %local_jalr_ptr
                  %{addr}_2 = icmp eq i1 %{addr}_1, 1
                  br i1 %{addr}_2, label %{addr}_local, label %ret
                {addr}_local:
                  store i1 0, ptr %local_jalr_ptr
                  br label %disp",
                ra = if *stk {Value::StkReg(rv::Reg::Ra)} else {Value::Reg(rv::Reg::Ra)},
            ),
            Dispfunc { addr, target , used_regs, used_fregs} => {
                let stack_storing = Func::build_stack_storing(used_regs, used_fregs, &format!("{addr}_s"));
                let stack_loading = Func::build_stack_loading(used_regs, used_fregs, &format!("{addr}_l"));
                writedoc!(f, "
                    {stack_storing}
                      %{addr}_ra = call i64 @.find_func(i64 {target})
                      {stack_loading}
                      %{addr}_fail = icmp eq i64 %{addr}_ra, 0
                      br i1 %{addr}_fail, label %{addr}_disp, label %{addr}_cont
                    {addr}_disp:
                      store i64 {target}, ptr %entry_ptr
                      store i1 1, ptr %local_jalr_ptr
                      br label %disp
                    {addr}_cont:
                      store i64 %{addr}_ra, ptr %entry_ptr
                      br label %disp")
            }
            Memcmp { addr ,stk} => writedoc!(f, "
                %{addr}_0 = load i64, ptr {a0}
                  %{addr}_1 = call ptr @.get_mem_ptr(i64 %{addr}_0)
                  %{addr}_2 = load i64, ptr {a1}
                  %{addr}_3 = call ptr @.get_mem_ptr(i64 %{addr}_2)
                  %{addr}_4 = load i64, ptr {a2}
                  %{addr}_5 = call i32 @memcmp(ptr %{addr}_1, ptr %{addr}_3, i64 %{addr}_4)
                  %{addr}_6 = sext i32 %{addr}_5 to i64
                  store i64 %{addr}_6, ptr {a0}",
                a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
                a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
                a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
            ),
            Memset { addr ,stk} => writedoc!(f, "
                %{addr}_0 = load i64, ptr {a0}
                  %{addr}_1 = call ptr @.get_mem_ptr(i64 %{addr}_0)
                  %{addr}_2 = load i64, ptr {a1}
                  %{addr}_3 = trunc i64 %{addr}_2 to i8
                  %{addr}_4 = load i64, ptr {a2}
                  call void @llvm.memset.p0.i64(ptr %{addr}_1, i8 %{addr}_3, i64 %{addr}_4, i1 false)",
                a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
                a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
                a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
            ),
            Memcpy { addr ,stk} => writedoc!(f, "
                %{addr}_0 = load i64, ptr {a0}
                  %{addr}_1 = call ptr @.get_mem_ptr(i64 %{addr}_0)
                  %{addr}_2 = load i64, ptr {a1}
                  %{addr}_3 = call ptr @.get_mem_ptr(i64 %{addr}_2)
                  %{addr}_4 = load i64, ptr {a2}
                  call void @llvm.memcpy.p0.p0.i64(ptr %{addr}_1, ptr %{addr}_3, i64 %{addr}_4, i1 false)",
                a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
                a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
                a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
            ),
            Memmove { addr ,stk} => writedoc!(f, "
                %{addr}_0 = load i64, ptr {a0}
                  %{addr}_1 = call ptr @.get_mem_ptr(i64 %{addr}_0)
                  %{addr}_2 = load i64, ptr {a1}
                  %{addr}_3 = call ptr @.get_mem_ptr(i64 %{addr}_2)
                  %{addr}_4 = load i64, ptr {a2}
                  call void @llvm.memmove.p0.p0.i64(ptr %{addr}_1, ptr %{addr}_3, i64 %{addr}_4, i1 false)",
                a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
                a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
                a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Type {
    I1,
    I8,
    I16,
    I32,
    I64,
    I128,
    Float,
    Double,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Type::*;

        match self {
            I1 => write!(f, "i1"),
            I8 => write!(f, "i8"),
            I16 => write!(f, "i16"),
            I32 => write!(f, "i32"),
            I64 => write!(f, "i64"),
            I128 => write!(f, "i128"),
            Float => write!(f, "float"),
            Double => write!(f, "double"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Value {
    Reg(rv::Reg),
    FReg(rv::FReg),
    Imm(rv::Imm),
    Addr(rv::Addr),
    Temp(rv::Addr, usize),
    RS,
    StkReg(rv::Reg),
    StkFReg(rv::FReg),
    EntryPtr,
    Disp,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Value::*;

        match self {
            Reg(reg) => write!(f, "@.{reg}"),
            FReg(freg) => write!(f, "@.{freg}"),
            Imm(imm) => write!(f, "{imm}"),
            Addr(addr) => write!(f, "u{addr}"),
            Temp(addr, i) => write!(f, "%u{addr}_{i}"),
            RS => write!(f, "@.rs"),
            StkReg(reg) => write!(f, "%{reg}"),
            StkFReg(freg) => write!(f, "%{freg}"),
            EntryPtr => write!(f, "%entry_ptr"),
            Disp => write!(f, "disp"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Cond {
    Eq,
    Ne,
    Uge,
    Ult,
    Sgt,
    Sge,
    Slt,
    Sle,
}

impl Display for Cond {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Cond::*;

        match self {
            Eq => write!(f, "eq"),
            Ne => write!(f, "ne"),
            Uge => write!(f, "uge"),
            Ult => write!(f, "ult"),
            Sgt => write!(f, "sgt"),
            Sge => write!(f, "sge"),
            Slt => write!(f, "slt"),
            Sle => write!(f, "sle"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum FCond {
    Oeq,
    Olt,
    Ole,
}

impl Display for FCond {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use FCond::*;

        match self {
            Oeq => write!(f, "oeq"),
            Olt => write!(f, "olt"),
            Ole => write!(f, "ole"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Mo {
    Monotonic,
    Acquire,
    Release,
    SeqCst,
}

impl Display for Mo {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Mo::*;

        match self {
            Monotonic => write!(f, "monotonic"),
            Acquire => write!(f, "acquire"),
            Release => write!(f, "release"),
            SeqCst => write!(f, "seq_cst"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Op {
    Xchg,
    Add,
    And,
    Or,
    Xor,
    Max,
    Min,
    Umax,
    Umin,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Op::*;

        match self {
            Xchg => write!(f, "xchg"),
            Add => write!(f, "add"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Xor => write!(f, "xor"),
            Max => write!(f, "max"),
            Min => write!(f, "min"),
            Umax => write!(f, "umax"),
            Umin => write!(f, "umin"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Rm {
    Dynamic,
    Tonearest,
    Downward,
    Upward,
    Towardzero,
    Tonearestaway,
}
