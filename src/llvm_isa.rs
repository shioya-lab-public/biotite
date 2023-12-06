use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Prog {
    pub entry: Value,
    pub tdata: Option<(Value, usize)>,
    pub mem: Vec<u8>,
    pub mem_s: Option<String>,
    pub mem_ld: Option<String>,
    pub sp: Value,
    pub phdr: Value,
    pub funcs: Vec<Func>,
    pub sys_call: Option<String>,
    pub ir_funcs: HashSet<String>,
    pub func_syms: HashSet<(String, Value)>,
    pub native_mem_utils: bool,
}

impl Display for Prog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let funcs = self
            .funcs
            .iter()
            .map(|func| {
                if self.ir_funcs.contains(&func.symbol) && !func.is_fallback {
                    format!("declare i64 @.{}(i64)", func.address)
                } else {
                    func.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        let (dispatcher_len, dispatchers) = self.build_dispatchers(false);
        let mut prog = format!("define i32 @main(i32 %argc, i8** %argv, i8** %envp) {{
{init}

loop:
  %entry = load i64, i64* %entry_p
  %func_addr_ptr = getelementptr [{dispatcher_len} x i64], [{dispatcher_len} x i64]* @.dispatcher, i64 0, i64 %entry
  %func_addr = load i64, i64* %func_addr_ptr
  %func = inttoptr i64 %func_addr to i64 (i64)*
  %next = call i64 %func(i64 %entry)
  store i64 %next, i64* %entry_p
  br label %loop
}}

{funcs}

{mem}

{dispatchers}

{rounding_funcs}

{static_decls_defs}",
            init = self.build_init(),
            mem = self.build_mem(false),
            rounding_funcs = Self::build_rounding_funcs(),
            static_decls_defs = include_str!("static_decls_defs.ll"),
        );
        if self.native_mem_utils {
            prog += "
declare void @llvm.memcpy.p8.p8.i64(i8*, i8*, i64, i1)
declare void @llvm.memmove.p8.p8.i64(i8*, i8*, i64, i1)
declare void @llvm.memset.p8.i64(i8*, i8, i64, i1)
declare i32 @memcmp(i8*, i8*, i64)
";
        };
        if let Some(sys_call) = &self.sys_call {
            prog += &format!("\n{sys_call}\n");
        }
        write!(f, "{prog}")
    }
}

impl Prog {
    fn build_init(&self) -> String {
        let mut init = format!(
            "  ; Initialize the stack pointer
  store i64 {sp}, i64* @.sp

  ; Initialize `argc`
  %argc_dest_b = call i8* @.get_mem_ptr(i64 {sp})
  %argc_dest = bitcast i8* %argc_dest_b to i32*
  store i32 %argc, i32* %argc_dest

  ; Initialize `argv`
  %argv_addr = add i64 {sp}, 8
  %argv_dest = call i8* @.get_mem_ptr(i64 %argv_addr)
  %argv_src = bitcast i8** %argv to i8*
  %argc_i64 = sext i32 %argc to i64
  %argv_byte_cnt = mul i64 %argc_i64, 8
  call void @.mem_copy(i8* %argv_dest, i8* %argv_src, i64 %argv_byte_cnt)

  ; Initialize `envp`
  %argv_offset = add i64 %argv_byte_cnt, 8
  %envp_addr = add i64 %argv_addr, %argv_offset
  %envp_dest_b = call i8* @.get_mem_ptr(i64 %envp_addr)
  %envp_dest = bitcast i8* %envp_dest_b to i8**
  %auxv_b = call i8* @.copy_envp(i8** %envp, i8** %envp_dest)",
            sp = self.sp,
        );
        if let Some((tdata_addr, tdata_len)) = self.tdata {
            init += &format!(
                "

  ; Initialize `auxv`
  %auxv = bitcast i8* %auxv_b to i64*
  %phdr = call i8* @.get_mem_ptr(i64 {phdr})
  call void @.init_auxv(i64* %auxv, i8* %phdr, i64 {phdr}, i64 {tdata_addr}, i64 {tdata_len})",
                phdr = self.phdr,
            );
        }
        init += &format!(
            "

  ; Load the entry address
  %entry_p= alloca i64
  store i64 {entry}, i64* %entry_p
  br label %loop",
            entry = self.entry,
        );
        init
    }

    pub fn build_mem(&self, external: bool) -> String {
        if self.mem_s.is_some() {
            String::from(
                "define internal i8* @.get_mem_ptr(i64 %addr) alwaysinline {
  %ptr = inttoptr i64 %addr to i8*
  ret i8* %ptr
}",
            )
        } else {
            let mem = format!(
                "@.mem = global [{len} x i8] [{mem}]",
                len = self.mem.len(),
                mem = self
                    .mem
                    .iter()
                    .map(|byte| format!("i8 {byte}"))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            let get_mem_ptr = format!(
                "define internal i8* @.get_mem_ptr(i64 %addr) alwaysinline {{
  %is_static = icmp ugt i64 {len}, %addr
  br i1 %is_static, label %static, label %dynamic
static:
  %static_ptr = getelementptr [{len} x i8], [{len} x i8]* @.mem, i64 0, i64 %addr
  ret i8* %static_ptr
dynamic:
  %dynamic_ptr = inttoptr i64 %addr to i8*
  ret i8* %dynamic_ptr
}}",
                len = self.mem.len()
            );
            if external {
                format!(
                    "@.mem = external global [{len} x i8]

{get_mem_ptr}",
                    len = self.mem.len()
                )
            } else {
                format!(
                    "{mem}

{get_mem_ptr}"
                )
            }
        }
    }

    pub fn build_dispatchers(&self, external: bool) -> (usize, String) {
        let mut dispatcher = Vec::new();
        let mut func_dispatcher = Vec::new();
        for func in &self.funcs {
            if func.is_fallback {
                continue;
            }
            let last_rv_inst = &func.inst_blocks.last().unwrap().rv_inst;
            let rv::Addr(mut end) = last_rv_inst.address();
            end += if last_rv_inst.is_compressed() { 2 } else { 4 };
            dispatcher.resize(end as usize, String::from("i64 0"));
            func_dispatcher.resize(end as usize, String::from("i64 0"));
            let ptr = format!(
                "i64 ptrtoint (i64 (i64)* @.{}{} to i64)",
                func.address,
                if !func.is_opaque { "_fallback" } else { "" }
            );
            for inst_block in &func.inst_blocks {
                let rv::Addr(addr) = inst_block.rv_inst.address();
                dispatcher[addr as usize] = ptr.clone();
            }
            let rv::Addr(addr) = func.inst_blocks[0].rv_inst.address();
            func_dispatcher[addr as usize] = ptr;
        }
        let dispatcher_len = dispatcher.len();
        let func_dispatcher_len = func_dispatcher.len();
        let dispatcher = format!(
            "@.dispatcher = global [{dispatcher_len} x i64] [{disp}]",
            disp = dispatcher.join(", ")
        );
        let func_dispatcher = format!(
            "@.func_dispatcher = global [{dispatcher_len} x i64] [{disp}]",
            disp = func_dispatcher.join(", ")
        );
        let dispatch_func = format!("define internal i64 @.dispatch_func(i64 %func) alwaysinline {{
  %func_addr_ptr = getelementptr [{func_dispatcher_len} x i64], [{func_dispatcher_len} x i64]* @.func_dispatcher, i64 0, i64 %func
  %func_addr = load i64, i64* %func_addr_ptr
  %is_func = icmp ne i64 %func_addr, 0
  br i1 %is_func, label %call, label %ret
call:
  %func_ptr = inttoptr i64 %func_addr to i64 (i64)*
  %rslt = call i64 %func_ptr(i64 %func)
  ret i64 %rslt
ret:
  ret i64 0
}}");
        if external {
            (
                dispatcher_len,
                format!(
                    "@.dispatcher = external global [{dispatcher_len} x i64]
@.func_dispatcher = external global [{func_dispatcher_len} x i64] 

{dispatch_func}"
                ),
            )
        } else {
            (
                dispatcher_len,
                format!(
                    "{dispatcher}
{func_dispatcher}

{dispatch_func}"
                ),
            )
        }
    }

    fn build_rounding_funcs() -> String {
        let variants = vec![
            ("float", "i32", "fptosi", "sitofp"),
            ("float", "i32", "fptoui", "uitofp"),
            ("float", "i64", "fptosi", "sitofp"),
            ("float", "i64", "fptoui", "uitofp"),
            ("double", "i32", "fptosi", "sitofp"),
            ("double", "i32", "fptoui", "uitofp"),
            ("double", "i64", "fptosi", "sitofp"),
            ("double", "i64", "fptoui", "uitofp"),
        ];
        let mut rounding_funcs = String::new();
        for (fp, int, fp_int, int_fp) in variants {
            rounding_funcs += &format!(
                "define {int} @.rounding_{fp}_{int}_{fp_int}_{int_fp}({fp} %0, i1 %1) alwaysinline {{
  %3 = {fp_int} {fp} %0 to {int}
  %4 = {int_fp} {int} %3 to {fp}
  %5 = fcmp une {fp} %4, %0
  %6 = fcmp ogt {fp} %0, 0.000000e+00
  %7 = and i1 %6, %5
  %8 = xor i1 %7, true
  %9 = or i1 %8, %1
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = add {int} %3, 1
  br label %18

12:                                               ; preds = %2
  %13 = fcmp olt {fp} %0, 0.000000e+00
  %14 = and i1 %13, %5
  %15 = and i1 %14, %1
  %16 = sext i1 %15 to {int}
  %17 = add {int} %16, %3
  br label %18

18:                                               ; preds = %12, %10
  %19 = phi {int} [ %11, %10 ], [ %17, %12 ]
  ret {int} %19
}}

"
            );
        }
        rounding_funcs.pop();
        rounding_funcs.pop();
        rounding_funcs
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Func {
    pub section: String,
    pub symbol: String,
    pub address: Value,
    pub inst_blocks: Vec<InstBlock>,
    pub is_opaque: bool,
    pub is_fallback: bool,
    pub synced_regs: Vec<rv::Reg>,
    pub synced_fregs: Vec<rv::FReg>,
    pub used_regs: Vec<rv::Reg>,
    pub used_fregs: Vec<rv::FReg>,
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut func = format!(
            "; {addr} {sec} <{sym}>
define i64 @.{addr}{ver}(i64 %entry) {{
  %entry_ptr = alloca i64
  store i64 %entry, i64* %entry_ptr
  %local_jalr_ptr = alloca i1, i1 0
",
            addr = self.address,
            sec = self.section,
            sym = self.symbol,
            ver = if self.is_fallback { "_fallback" } else { "" },
        );
        if !self.used_regs.is_empty() {
            let stack_regs = self
                .used_regs
                .iter()
                .map(|reg| format!("  {reg} = alloca i64", reg = Value::StkReg(*reg)))
                .collect::<Vec<_>>()
                .join("\n");
            func += &format!("\n{stack_regs}\n");
        }
        if !self.used_fregs.is_empty() {
            let stack_fregs = self
                .used_fregs
                .iter()
                .map(|freg| format!("  {freg} = alloca double", freg = Value::StkFReg(*freg)))
                .collect::<Vec<_>>()
                .join("\n");
            func += &format!("\n{stack_fregs}\n");
        }
        if !self.synced_regs.is_empty() || !self.synced_fregs.is_empty() {
            let stack_loading =
                Self::build_stack_loading(&self.synced_regs, &self.synced_fregs, "entry");
            func += &format!("\n  {stack_loading}\n");
        }
        if self.is_opaque {
            let mut dispatcher = String::from("switch i64 %addr, label %func_dispatcher [");
            for inst_block in &self.inst_blocks {
                let addr = Value::Addr(inst_block.rv_inst.address());
                dispatcher += &format!("i64 {addr}, label %{addr} ");
            }
            dispatcher.pop();
            dispatcher += "]";
            let func_dispatcher = if !self.used_regs.is_empty() || !self.used_fregs.is_empty() {
                let stack_storing =
                    Self::build_stack_storing(&self.used_regs, &self.used_fregs, "disp_s");
                let mut func_dispatcher = format!(
                    "{stack_storing}
  %ra_val = call i64 @.dispatch_func(i64 %addr)"
                );
                if !self.used_regs.is_empty() || !self.used_fregs.is_empty() {
                    let stack_loading =
                        Self::build_stack_loading(&self.used_regs, &self.used_fregs, "disp_l");
                    func_dispatcher += &format!(
                        "
  {stack_loading}"
                    )
                }
                func_dispatcher
            } else {
                "%ra_val = call i64 @.dispatch_func(i64 %addr)".to_string()
            };
            func += &format!(
                "
  br label %dispatcher

dispatcher:
  %addr = load i64, i64* %entry_ptr
  {dispatcher}
func_dispatcher:
  {func_dispatcher}
  %fail = icmp eq i64 %ra_val, 0
  br i1 %fail, label %ret, label %cont
cont:
  store i64 %ra_val, i64* %entry_ptr
  br label %dispatcher
"
            );
        } else {
            let addr = Value::Addr(self.inst_blocks[0].rv_inst.address());
            func += &format!(
                "
  br label %{addr}
"
            );
        };
        let inst_blocks = self
            .inst_blocks
            .iter()
            .map(|block| block.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        func += &format!("\n{inst_blocks}\n");
        let last_rv_inst = &self.inst_blocks.last().unwrap().rv_inst;
        let next_pc = next_pc!(
            next_pc,
            last_rv_inst.address(),
            last_rv_inst.is_compressed()
        );
        func += &format!(
            "
{next_pc}:
  store i64 {next_pc}, i64* %entry_ptr
  br label %ret
"
        );
        if !self.used_regs.is_empty() || !self.used_fregs.is_empty() {
            let stack_storing = Self::build_stack_storing(&self.used_regs, &self.used_fregs, "ret");
            func += &format!(
                "
ret:
  {stack_storing}

  %target = load i64, i64* %entry_ptr
  store i64 %target, i64* @.ra
  ret i64 %target
}}"
            );
        } else {
            func += "
ret:
  %target = load i64, i64* %entry_ptr
  store i64 %target, i64* @.ra
  ret i64 %target
}";
        }
        write!(f, "{func}")
    }
}

impl Func {
    pub fn build_stack_loading(regs: &[rv::Reg], fregs: &[rv::FReg], prefix: &str) -> String {
        let regs = regs
            .iter()
            .map(|reg| {
                format!(
                    "  %{prefix}_{reg}_val = load i64, i64* {global}
  store i64 %{prefix}_{reg}_val, i64* {stack}",
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
                    "  %{prefix}_{freg}_val = load double, double* {global}
  store double %{prefix}_{freg}_val, double* {stack}",
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
                    "  %{prefix}_{reg}_val = load i64, i64* {stack}
  store i64 %{prefix}_{reg}_val, i64* {global}",
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
                    "  %{prefix}_{freg}_val = load double, double* {stack}
  store double %{prefix}_{freg}_val, double* {global}",
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstBlock {
    pub rv_inst: rv::Inst,
    pub insts: Vec<Inst>,
}

impl Display for InstBlock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let addr = Value::Addr(self.rv_inst.address());
        let insts = self
            .insts
            .iter()
            .map(|inst| format!("  {inst}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut block = format!(
            "; {rv_inst:?}
{addr}:
{insts}",
            rv_inst = self.rv_inst
        );
        if !matches!(
            self.insts.last(),
            Some(Inst::Ret { .. })
                | Some(Inst::Br { .. })
                | Some(Inst::Conbr { .. })
                | Some(Inst::Checkret { .. })
                | Some(Inst::Contret { .. })
                | Some(Inst::Dispfunc { .. })
                | Some(Inst::Dispret { .. })
        ) {
            let next_pc = next_pc!(
                next_pc,
                self.rv_inst.address(),
                self.rv_inst.is_compressed()
            );
            let br = Inst::Br { addr: next_pc };
            block += &format!(
                "
  {br}"
            );
        };
        write!(f, "{block}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        mo: MO,
    },
    Atomicrmw {
        rslt: Value,
        op: Op,
        ty: Type,
        ptr: Value,
        val: Value,
        mo: MO,
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
        rm: RM,
    },
    Fptosi {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
        rm: RM,
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
    Contret {
        addr: Value,
        next_pc: Value,
        stk: bool,
    },
    Dispfunc {
        addr: Value,
        target: Value,
        used_regs: Vec<rv::Reg>,
        used_fregs: Vec<rv::FReg>,
    },
    Dispret {
        addr: Value,
        next_pc: Value,
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
    Memset {
        addr: Value,
        stk: bool,
    },
    Memcmp {
        addr: Value,
        stk: bool,
    },
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Inst::*;

        match self {
            // Terminator Instructions
            Ret { val } => write!(f, "store i64 {val}, i64* %entry_ptr
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
            Load { rslt, ty, ptr } => write!(f, "{rslt} = load {ty}, {ty}* {ptr}"),
            Store { ty, val, ptr } => write!(f, "store {ty} {val}, {ty}* {ptr}"),
            Fence => write!(f, "fence seq_cst"),
            Cmpxchg { rslt, ty, ptr, cmp, new, mo } => write!(f, "{rslt} = cmpxchg {ty}* {ptr}, {ty} {cmp}, {ty} {new} {mo} monotonic"),
            Atomicrmw { rslt, op, ty, ptr, val, mo } => write!(f, "{rslt} = atomicrmw {op} {ty}* {ptr}, {ty} {val} {mo}"),
            Getelementptr { rslt, ptr, offset } => write!(f, "{rslt} = getelementptr i8, i8* {ptr}, i64 {offset}"),

            // Conversion Operations
            Trunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = trunc {ty1} {val} to {ty2}"),
            Zext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = zext {ty1} {val} to {ty2}"),
            Sext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sext {ty1} {val} to {ty2}"),
            Fptrunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fptrunc {ty1} {val} to {ty2}"),
            Fpext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fpext {ty1} {val} to {ty2}"),
            Fptoui { rslt, ty1, val, ty2, rm } => match rm {
                RM::Downward => write!(f, "{rslt} = call {ty2} @.rounding_{ty1}_{ty2}_fptoui_uitofp({ty1} {val}, i1 1)"),
                RM::Upward => write!(f, "{rslt} = call {ty2} @.rounding_{ty1}_{ty2}_fptoui_uitofp({ty1} {val}, i1 0)"),
                _ => write!(f, "{rslt} = fptoui {ty1} {val} to {ty2}"),
            }
            Fptosi { rslt, ty1, val, ty2, rm } => match rm {
                RM::Downward => write!(f, "{rslt} = call {ty2} @.rounding_{ty1}_{ty2}_fptosi_sitofp({ty1} {val}, i1 1)"),
                RM::Upward => write!(f, "{rslt} = call {ty2} @.rounding_{ty1}_{ty2}_fptosi_sitofp({ty1} {val}, i1 0)"),
                _ => write!(f, "{rslt} = fptosi {ty1} {val} to {ty2}"),
            }
            Uitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = uitofp {ty1} {val} to {ty2}"),
            Sitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sitofp {ty1} {val} to {ty2}"),
            Bitcast { rslt, ty1, val, ty2 } => match (ty1, ty2) {
                (Type::Float, _) | (_, Type::Float) | (Type::Double, _) | (_, Type::Double) => write!(f, "{rslt} = bitcast {ty1} {val} to {ty2}"),
                _ => write!(f, "{rslt} = bitcast {ty1}* {val} to {ty2}*"),
            },

            // Other Operations
            Icmp { rslt, cond, op1, op2 } => write!(f, "{rslt} = icmp {cond} i64 {op1}, {op2}"),
            Fcmp {rslt,fcond,op1,op2} => write!(f, "{rslt} = fcmp {fcond} double {op1}, {op2}"),
            Select {rslt,cond, ty, op1,op2} => write!(f, "{rslt} = select i1 {cond}, {ty} {op1}, {ty} {op2}"),
            Call { rslt, target, used_regs, used_fregs } => if !used_regs.is_empty() || !used_fregs.is_empty() {
                let stack_storing = Func::build_stack_storing(used_regs, used_fregs, &format!("{}_s", &rslt.to_string()[1..]));
                let stack_loading = Func::build_stack_loading(used_regs, used_fregs, &format!("{}_l", &rslt.to_string()[1..]));
                write!(f, "{stack_storing}
  {rslt} = call i64 @.{target}(i64 {target})
  {stack_loading}")
            } else {
                write!(f, "{rslt} = call i64 @.{target}(i64 {target})")
            }

            // Standard C/C++ Library Intrinsics
            Sqrt { rslt, ty, arg } => write!(f,"{rslt} = call {ty} @llvm.sqrt.{ty}({ty} {arg})"),
            Fma { rslt, ty, arg1, arg2, arg3 } => write!(f, "{rslt} = call {ty} @llvm.fma.{ty}({ty} {arg1}, {ty} {arg2}, {ty} {arg3})"),
            Fabs { rslt, ty, arg } => write!(f, "{rslt} = call {ty} @llvm.fabs.{ty}({ty} {arg})"),
            Copysign { rslt, ty, mag, sgn } => write!(f, "{rslt} = call {ty} @llvm.copysign.{ty}({ty} {mag}, {ty} {sgn})"),

            // Misc
            Getmemptr { rslt, addr } => write!(f, "{rslt} = call i8* @.get_mem_ptr(i64 {addr})"),
            Syscall { rslt, nr, arg1, arg2, arg3, arg4, arg5, arg6 } =>
                write!(f, "{rslt} = call i64 (i64, i64, i64, i64, i64, i64, i64) @.sys_call(i64 {nr}, i64 {arg1}, i64 {arg2}, i64 {arg3}, i64 {arg4}, i64 {arg5}, i64 {arg6})"),

            Checkret { addr , stk} => write!(f, "%{addr}_0 = load i64, i64* {ra}
  store i64 %{addr}_0, i64* %entry_ptr
  %{addr}_1 = load i1, i1* %local_jalr_ptr
  %{addr}_2 = icmp eq i1 %{addr}_1, 1
  br i1 %{addr}_2, label %{addr}_local, label %ret
{addr}_local:
  store i1 0, i1* %local_jalr_ptr
  br label %dispatcher",
    ra = if *stk {Value::StkReg(rv::Reg::Ra)} else {Value::Reg(rv::Reg::Ra)},
),
            Contret { addr, next_pc , stk} => write!(f, "%{addr}_ra = load i64, i64* {ra}
  %{addr}_is_next_pc = icmp eq i64 %{addr}_ra, {next_pc}
  br i1 %{addr}_is_next_pc, label %{next_pc}, label %{addr}_cont
{addr}_cont:
  store i64 %{addr}_ra, i64* %entry_ptr
  br label %ret",
    ra = if *stk {Value::StkReg(rv::Reg::Ra)} else {Value::Reg(rv::Reg::Ra)},
),
            Dispfunc { addr, target , used_regs, used_fregs} => {
                let call = if !used_regs.is_empty() || !used_fregs.is_empty() {
                    let stack_storing = Func::build_stack_storing(used_regs, used_fregs, &format!("{addr}_s"));
                    let stack_loading = Func::build_stack_loading(used_regs, used_fregs, &format!("{addr}_l"));
                    format!("{stack_storing}
  %{addr}_ra = call i64 @.dispatch_func(i64 {target})
  {stack_loading}")
                } else {
                    format!("%{addr}_ra = call i64 @.dispatch_func(i64 {target})")
                };
                write!(f, "{call}
  %{addr}_fail = icmp eq i64 %{addr}_ra, 0
  br i1 %{addr}_fail, label %{addr}_disp, label %{addr}_cont
{addr}_disp:
  store i64 {target}, i64* %entry_ptr
  store i1 1, i1* %local_jalr_ptr
  br label %dispatcher
{addr}_cont:
  store i64 %{addr}_ra, i64* %entry_ptr
  br label %dispatcher")
            }
            Dispret { addr, next_pc, stk } => write!(f, "%{addr}_ra = load i64, i64* {ra}
  %{addr}_is_next_pc = icmp eq i64 %{addr}_ra, {next_pc}
  br i1 %{addr}_is_next_pc, label %{next_pc}, label %{addr}_disp
{addr}_disp:
  store i64 %{addr}_ra, i64* %entry_ptr
  br label %dispatcher",
    ra = if *stk {Value::StkReg(rv::Reg::Ra)} else {Value::Reg(rv::Reg::Ra)}
),

            Memcpy { addr ,stk} => write!(f, "%{addr}_0 = load i64, i64* {a0}
  %{addr}_1 = call i8* @.get_mem_ptr(i64 %{addr}_0)
  %{addr}_2 = load i64, i64* {a1}
  %{addr}_3 = call i8* @.get_mem_ptr(i64 %{addr}_2)
  %{addr}_4 = load i64, i64* {a2}
  call void @llvm.memcpy.p8.p8.i64(i8* %{addr}_1, i8* %{addr}_3, i64 %{addr}_4, i1 false)",
    a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
    a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
    a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
),
            Memmove { addr ,stk} => write!(f, "%{addr}_0 = load i64, i64* {a0}
  %{addr}_1 = call i8* @.get_mem_ptr(i64 %{addr}_0)
  %{addr}_2 = load i64, i64* {a1}
  %{addr}_3 = call i8* @.get_mem_ptr(i64 %{addr}_2)
  %{addr}_4 = load i64, i64* {a2}
  call void @llvm.memmove.p8.p8.i64(i8* %{addr}_1, i8* %{addr}_3, i64 %{addr}_4, i1 false)",
    a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
    a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
    a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
),
            Memset { addr ,stk} => write!(f, "%{addr}_0 = load i64, i64* {a0}
  %{addr}_1 = call i8* @.get_mem_ptr(i64 %{addr}_0)
  %{addr}_2 = load i64, i64* {a1}
  %{addr}_3 = trunc i64 %{addr}_2 to i8
  %{addr}_4 = load i64, i64* {a2}
  call void @llvm.memset.p8.i64(i8* %{addr}_1, i8 %{addr}_3, i64 %{addr}_4, i1 false)",
    a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
    a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
    a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
),
            Memcmp { addr ,stk} => write!(f, "%{addr}_0 = load i64, i64* {a0}
  %{addr}_1 = call i8* @.get_mem_ptr(i64 %{addr}_0)
  %{addr}_2 = load i64, i64* {a1}
  %{addr}_3 = call i8* @.get_mem_ptr(i64 %{addr}_2)
  %{addr}_4 = load i64, i64* {a2}
  %{addr}_5 = call i32 @memcmp(i8* %{addr}_1, i8* %{addr}_3, i64 %{addr}_4)
  %{addr}_6 = sext i32 %{addr}_5 to i64
  store i64 %{addr}_6, i64* {a0}",
    a0 = if *stk {Value::StkReg(rv::Reg::A0)} else {Value::Reg(rv::Reg::A0)},
    a1 = if *stk {Value::StkReg(rv::Reg::A1)} else {Value::Reg(rv::Reg::A1)},
    a2 = if *stk {Value::StkReg(rv::Reg::A2)} else {Value::Reg(rv::Reg::A2)},
),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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
    fn fmt(&self, f: &mut Formatter) -> Result {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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
    Dispatcher,
}

impl Default for Value {
    fn default() -> Self {
        Value::Addr(rv::Addr::default())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
            Dispatcher => write!(f, "dispatcher"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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
    fn fmt(&self, f: &mut Formatter) -> Result {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum FCond {
    Oeq,
    Olt,
    Ole,
}

impl Display for FCond {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use FCond::*;

        match self {
            Oeq => write!(f, "oeq"),
            Olt => write!(f, "olt"),
            Ole => write!(f, "ole"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum MO {
    Monotonic,
    Acquire,
    Release,
    SeqCst,
}

impl Display for MO {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use MO::*;

        match self {
            Monotonic => write!(f, "monotonic"),
            Acquire => write!(f, "acquire"),
            Release => write!(f, "release"),
            SeqCst => write!(f, "seq_cst"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
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
    fn fmt(&self, f: &mut Formatter) -> Result {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum RM {
    Dynamic,
    Tonearest,
    Downward,
    Upward,
    Towardzero,
    Tonearestaway,
}
