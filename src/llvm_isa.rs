use crate::riscv_isa as RV;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub entry: RV::Addr,
    pub data_blocks: Vec<RV::DataBlock>,
    pub funcs: Vec<Func>,
    pub src_funcs: HashMap<RV::Addr, String>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Merge data blocks
        let mut memory = Vec::new();
        for data_block in self.data_blocks {
            let RV::Addr(start) = data_block.address;
            let diff = start as usize - memory.len();
            memory.extend(vec![0; diff]);
            memory.extend(data_block.bytes);
        }

        // Append the stack
        let stack_len = 8192 * 1024;
        let sp = memory.len() + 8191 * 1024;
        memory.extend(vec![0; stack_len]);

        // Format the memory array
        let memory_len = memory.len();
        let memory_str = memory
            .iter()
            .map(|b| format!("i8 {b}"))
            .collect::<Vec<_>>()
            .join(", ");
        let memory = format!("@.memory = global [{memory_len} x i8] [{memory_str}]");

        // Build `get_memory_ptr`
        let get_memory_ptr = format!(
            "define i8* @.get_memory_ptr(i64 %addr) {{
  %is_static = icmp ugt i64 {memory_len}, %addr
  br i1 %is_static, label %static, label %dynamic
static:
  %static_ptr = getelementptr [{memory_len} x i8], [{memory_len} x i8]* @memory, i64 0, i64 %addr
  ret i8* %static_ptr
dynamic:
  %dynamic_ptr = inttoptr i64 %addr to i8*
  ret i8* %dynamic_ptr
}}"
        );

        // Build the main dispatcher
        let mut dispatcher = Vec::new();
        for func in self.funcs {
            let last_rv_inst = func.inst_blocks.last().unwrap().rv_inst;
            let RV::Addr(end) = last_rv_inst.address();
            end += if last_rv_inst.is_compressed() { 2 } else { 4 };
            let diff = end as usize - dispatcher.len();
            dispatcher.extend(vec![String::from("i64 0"); diff]);
            for inst_block in func.inst_blocks {
                let RV::Addr(addr) = inst_block.rv_inst.address();
                if let Some(name) = self.src_funcs.get(&RV::Addr(addr)) {
                    dispatcher[addr as usize] = format!("i64 ptrtoint (i64 (i64)* @{name} to i64)");
                } else {
                    dispatcher[addr as usize] =
                        format!("i64 ptrtoint (i64 (i64)* @.{} to i64)", func.address);
                }
            }
        }
        let dispatcher_len = dispatcher.len();
        let dispatcher_str = dispatcher.join(", ");
        let dispatcher =
            format!("@.dispatcher = global [{dispatcher_len} x i64] [{dispatcher_str}]");

        // Format other components
        let entry = self.entry;
        let funcs = self
            .funcs
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n\n");
        let src_funcs = self
            .src_funcs
            .values()
            .map(|f| f.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");
        let system_call = include_str!("system_call.ll");

        // Merge all components
        write!(f, "define i64 @main(i32 %argc, i8** %argv) {{
  ; Initialize the stack pointer
  store i64 {sp}, i64* @sp

  ; Initialize `argc`
  %argc_addr = load i64, i64* @sp
  %argc_ptr_b = call i8* @.get_memory_ptr(i64 argc_addr)
  %argc_ptr_w = bitcast i8* %argc_ptr_b to i32*
  store i32 %argc, i32* %argc_ptr_w

  ; Initialize `argv`
  %argv_addr = add i64 %argc_addr, 8
  %argv_dest = call i8* @.get_memory_ptr(i64 %argv_addr)
  %argv_src = bitcast i8** %argv to i8*
  %num = mul i32 %argc, 8
  call void @.memory_copy(i8* %argv_dest, i8* %argv_src, i32 %num)

  ; Load the entry address
  %entry_p= alloca i64
  store i64 {entry}, i64* %entry_p
  br label %loop

loop:
  %entry = load i64, i64* %entry_p
  %func_addr_ptr = getelementptr [{dispatcher_len} x i64], [{dispatcher_len} x i64]* @dispatcher, i64 0, i64 %entry
  %func_addr = load i64, i64* %func_addr_ptr
  %func = inttoptr i64 %func_addr to i64 (i64)*
  %next = call i64 %func(i64 %entry)
  store i64 %next, i64* %entry_p
  br label %loop
}}

{funcs}

{src_funcs}

{memory}

{dispatcher}

@.zero = global i64 zeroinitializer
@.ra = global i64 zeroinitializer
@.sp = global i64 zeroinitializer
@.gp = global i64 zeroinitializer
@.tp = global i64 zeroinitializer
@.t0 = global i64 zeroinitializer
@.t1 = global i64 zeroinitializer
@.t2 = global i64 zeroinitializer
@.s0 = global i64 zeroinitializer
@.s1 = global i64 zeroinitializer
@.a0 = global i64 zeroinitializer
@.a1 = global i64 zeroinitializer
@.a2 = global i64 zeroinitializer
@.a3 = global i64 zeroinitializer
@.a4 = global i64 zeroinitializer
@.a5 = global i64 zeroinitializer
@.a6 = global i64 zeroinitializer
@.a7 = global i64 zeroinitializer
@.s2 = global i64 zeroinitializer
@.s3 = global i64 zeroinitializer
@.s4 = global i64 zeroinitializer
@.s5 = global i64 zeroinitializer
@.s6 = global i64 zeroinitializer
@.s7 = global i64 zeroinitializer
@.s8 = global i64 zeroinitializer
@.s9 = global i64 zeroinitializer
@.s10 = global i64 zeroinitializer
@.s11 = global i64 zeroinitializer
@.t3 = global i64 zeroinitializer
@.t4 = global i64 zeroinitializer
@.t5 = global i64 zeroinitializer
@.t6 = global i64 zeroinitializer

@.ft0 = global double zeroinitializer
@.ft1 = global double zeroinitializer
@.ft2 = global double zeroinitializer
@.ft3 = global double zeroinitializer
@.ft4 = global double zeroinitializer
@.ft5 = global double zeroinitializer
@.ft6 = global double zeroinitializer
@.ft7 = global double zeroinitializer
@.fs0 = global double zeroinitializer
@.fs1 = global double zeroinitializer
@.fa0 = global double zeroinitializer
@.fa1 = global double zeroinitializer
@.fa2 = global double zeroinitializer
@.fa3 = global double zeroinitializer
@.fa4 = global double zeroinitializer
@.fa5 = global double zeroinitializer
@.fa6 = global double zeroinitializer
@.fa7 = global double zeroinitializer
@.fs2 = global double zeroinitializer
@.fs3 = global double zeroinitializer
@.fs4 = global double zeroinitializer
@.fs5 = global double zeroinitializer
@.fs6 = global double zeroinitializer
@.fs7 = global double zeroinitializer
@.fs8 = global double zeroinitializer
@.fs9 = global double zeroinitializer
@.fs10 = global double zeroinitializer
@.fs11 = global double zeroinitializer
@.ft8 = global double zeroinitializer
@.ft9 = global double zeroinitializer
@.ft10 = global double zeroinitializer
@.ft11 = global double zeroinitializer

{get_memory_ptr}

define void @.memory_copy(i8* %0, i8* %1, i32 %2) {{
  %4 = icmp sgt i32 %2, 0
  br i1 %4, label %5, label %7

5:                                                ; preds = %3
  %6 = zext i32 %2 to i64
  br label %8

7:                                                ; preds = %8, %3
  ret void

8:                                                ; preds = %5, %8
  %9 = phi i64 [ 0, %5 ], [ %13, %8 ]
  %10 = getelementptr i8, i8* %1, i64 %9
  %11 = load i8, i8* %10
  %12 = getelementptr i8, i8* %0, i64 %9
  store i8 %11, i8* %12
  %13 = add i64 %9, 1
  %14 = icmp eq i64 %13, %6
  br i1 %14, label %7, label %8
}}

declare float @llvm.sqrt.float(float %arg)
declare double @llvm.sqrt.double(double %arg)
declare float @llvm.fma.float(float %arg1, float %arg2, float %arg3)
declare double @llvm.fma.double(double %arg1, double %arg2, double %arg3)
declare float @llvm.fabs.float(float %arg)
declare double @llvm.fabs.double(double %arg)
declare float @llvm.minimum.float(float %arg1, float %arg2)
declare double @llvm.minimum.double(double %arg1, double %arg2)
declare float @llvm.maximum.float(float %arg1, float %arg2)
declare double @llvm.maximum.double(double %arg1, double %arg2)
declare float @llvm.copysign.float(float %mag, float %sgn)
declare double @llvm.copysign.double(double %mag, double %sgn)

{system_call}
")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Func {
    pub section: String,
    pub symbol: String,
    pub address: RV::Addr,
    pub inst_blocks: Vec<InstBlock>,
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut dispatcher = String::from("switch i64 %entry, label %unreachable [");
        let mut inst_blocks = String::new();
        for inst_block in self.inst_blocks {
            let addr = Value::Addr(inst_block.rv_inst.address());
            dispatcher += &format!("i64 {addr}, label %{addr} ");
            inst_blocks += &inst_block.to_string();
        }
        dispatcher.pop();
        dispatcher += "]";
        write!(
            f,
            "; {} {} <{}>
define i64 @.{}(i64 %entry) {{
  {dispatcher}
unreachable:
  unreachable
{inst_blocks}
}}",
            self.address, self.section, self.symbol, self.address
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstBlock {
    pub rv_inst: RV::Inst,
    pub insts: Vec<Inst>,
}

impl Display for InstBlock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let insts_str = self
            .insts
            .iter()
            .map(|i| format!("  {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "  ; {:?}\n{insts_str}", self.rv_inst)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Value {
    Reg(RV::Reg),
    FReg(RV::FReg),
    Imm(RV::Imm),
    Addr(RV::Addr),
    Temp(RV::Addr, usize),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Value::*;

        match self {
            Reg(reg) => write!(f, "@.{reg}"),
            FReg(freg) => write!(f, "@.{freg}"),
            Imm(imm) => write!(f, "{imm}"),
            Addr(addr) => write!(f, "u{addr}"),
            Temp(addr, i) => write!(f, "%{addr}_{i}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MO {
    Mono,
    Aq,
    Rl,
    AqRl,
}

impl Display for MO {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use MO::*;

        match self {
            Mono => write!(f, "monotonic"),
            Aq => write!(f, "acquire"),
            Rl => write!(f, "release"),
            AqRl => write!(f, "acq_rel"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Inst {
    // Terminator Instructions
    Ret {
        val: Value,
    },
    Br {
        addr: Value,
    },

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
    Fence {
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
    },
    Fptosi {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
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
        op1: Value,
        op2: Value,
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
    Minimum {
        rslt: Value,
        ty: Type,
        arg1: Value,
        arg2: Value,
    },
    Maximum {
        rslt: Value,
        ty: Type,
        arg1: Value,
        arg2: Value,
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
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Inst::*;

        match self {
            // Terminator Instructions
            Ret { val } => write!(f, "ret i64 {val}"),
            Br { addr } => write!(f, "br label %{addr}"),

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

            // Memory Access and Addressing Operations
            Load { rslt, ty, ptr } => write!(f, "{rslt} = load {ty}, {ty}* {ptr}"),
            Store { ty, val, ptr } => write!(f, "store {ty} {val}, {ty}* {ptr}"),
            Fence { mo } => write!(f, "fence {mo}"),
            Atomicrmw { rslt, op, ty, ptr, val, mo } => write!(f, "{rslt} = atomicrmw {op} {ty}* {ptr}, {ty} {val} {mo}"),

            // Conversion Operations
            Trunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = trunc {ty1} {val} to {ty2}"),
            Zext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = zext {ty1} {val} to {ty2}"),
            Sext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sext {ty1} {val} to {ty2}"),
            Fptrunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fptrunc {ty1} {val} to {ty2}"),
            Fpext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fpext {ty1} {val} to {ty2}"),
            Fptoui { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fptoui {ty1} {val} to {ty2}"),
            Fptosi { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fptosi {ty1} {val} to {ty2}"),
            Uitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = uitofp {ty1} {val} to {ty2}"),
            Sitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sitofp {ty1} {val} to {ty2}"),
            Bitcast { rslt, ty1, val, ty2 } => match (ty1, ty2) {
                (Type::Float, _) | (_, Type::Float) | (Type::Double, _) | (_, Type::Double) => write!(f, "{rslt} = bitcast {ty1} {val} to {ty2}"),
                _ => write!(f, "{rslt} = bitcast {ty1}* {val} to {ty2}*"),
            },

            // Other Operations
            Icmp { rslt, cond, op1, op2 } => write!(f, "{rslt} = icmp {cond} i64 {op1}, {op2}"),
            Fcmp {rslt,fcond,op1,op2} => write!(f, "{rslt} = fcmp {fcond} i64 {op1}, {op2}"),
            Select {rslt,cond,op1,op2} => write!(f, "{rslt} = select i1 {cond}, i64 {op1}, i64 {op2}"),

            // Standard C/C++ Library Intrinsics
            Sqrt { rslt, ty, arg } => write!(f,"{rslt} = call {ty} @llvm.sqrt.{ty}({ty} {arg})"),
            Fma { rslt, ty, arg1, arg2, arg3 } => write!(f, "{rslt} = call {ty} @llvm.fma.{ty}({ty} {arg1}, {ty} {arg2}, {ty} {arg3})"),
            Fabs { rslt, ty, arg } => write!(f, "{rslt} = call {ty} @llvm.fabs.{ty}({ty} {arg})"),
            Minimum { rslt, ty, arg1, arg2 } => write!(f, "{rslt} = call {ty} @llvm.minimum.{ty}({ty} {arg1}, {ty} {arg2})"),
            Maximum { rslt, ty, arg1, arg2 } => write!(f, "{rslt} = call {ty} @llvm.maximum.{ty}({ty} {arg1}, {ty} {arg2})"),
            Copysign { rslt, ty, mag, sgn } => write!(f, "{rslt} = call {ty} @llvm.copysign.{ty}({ty} {mag}, {ty} {sgn})"),

            // Misc
            Getmemptr { rslt, addr } => write!(f, "{rslt} = call i8* @get_data_ptr(i64 {addr})"),
            Syscall { rslt, nr, arg1, arg2, arg3, arg4, arg5, arg6 } =>
                write!(f, "{rslt} = call i64 (i64, ...) @syscall(i64 {nr}, i64 {arg1}, i64 {arg2}, i64 {arg3}, i64 {arg4}, i64 {arg5}, i64 {arg6})"),
        }
    }
}
