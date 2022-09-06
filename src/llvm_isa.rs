use crate::riscv_isa as RV;
use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub entry: RV::Addr,
    pub data_blocks: Vec<RV::DataBlock>,
    pub funcs: Vec<Func>,
    pub src_funcs: HashMap<RV::Addr, String>,
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
            let addr = inst_block.rv_inst.address();
            dispatcher += &format!("i64 {addr}, label %{addr} ");
            inst_blocks += &inst_block.to_string();
        }
        dispatcher.pop();
        dispatcher += "]";
        write!(f, "; {} {} <{}>
define i64 @{}(i64 %entry) {{
  {dispatcher}
unreachable:
  unreachable
  {inst_blocks}
}}", self.address, self.section, self.symbol, self.address)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstBlock {
    pub rv_inst: RV::Inst,
    pub insts: Vec<Inst>,
}

impl Display for InstBlock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut inst_block = format!("  ; {:?}\n", self.rv_inst);
        for inst in self.insts {
            inst_block += &format!("  {inst}\n");
        }
        write!(f, "{}", inst_block)
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
            Reg(reg) => write!(f, "@{reg}"),
            FReg(freg) => write!(f, "@{freg}"),
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
    Getdataptr {
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
            Getdataptr { rslt, addr } => write!(f, "{rslt} = call i8* @get_data_ptr(i64 {addr})"),
            Syscall { rslt, nr, arg1, arg2, arg3, arg4, arg5, arg6 } => write!(f, "{rslt} = call i64 (i64, ...) @syscall(i64 {nr}, i64 {arg1}, i64 {arg2}, i64 {arg3}, i64 {arg4}, i64 {arg5}, i64 {arg6})"),
        }
    }
}



const SYSCALL: &str = "declare i64 @syscall(i64, ...)

%struct.tms = type { i64, i64, i64, i64 }

%struct.stat = type { i64, i64, i64, i32, i32, i32, i32, i64, i64, i64, i64, %struct.timespec, %struct.timespec, %struct.timespec, [3 x i64] }
%struct.timespec = type { i64, i64 }

define i64 @sys_call(i64 %nr, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) {
  switch i64 %nr, label %fallback [
    i64 93, label %SYS_exit
    i64 169, label %SYS_gettimeofday
    i64 214, label %SYS_brk
    i64 57, label %SYS_close
    i64 80, label %SYS_fstat
    i64 62, label %SYS_lseek
    i64 63, label %SYS_read
    i64 64, label %SYS_write
  ]

SYS_exit:
  %SYS_exit_rslt = call i64 (i64, ...) @syscall(i64 60, i64 %arg1)
  ret i64 %SYS_exit_rslt

SYS_gettimeofday:
  %tms_ptr = call i8* @get_data_ptr(i64 %arg1)
  %tms = bitcast i8* %tms_ptr to %struct.tms*
  %SYS_gettimeofday_rslt = call i64 (i64, ...) @syscall(i64 96, %struct.tms* %tms, i64 %arg2)
  ret i64 %SYS_gettimeofday_rslt

SYS_brk:
  %SYS_brk_rslt = call i64 (i64, ...) @syscall(i64 12, i64 %arg1)
  ret i64 %SYS_brk_rslt

SYS_close:
  %SYS_close_rslt = call i64 (i64, ...) @syscall(i64 3, i64 %arg1)
  ret i64 %SYS_close_rslt

SYS_fstat:
  %stat_ptr = call i8* @get_data_ptr(i64 %arg2)
  %stat = bitcast i8* %stat_ptr to %struct.stat*
  %SYS_fstat_rslt = call i64 (i64, ...) @syscall(i64 5, i64 %arg1, %struct.stat* %stat)
  ret i64 %SYS_fstat_rslt

SYS_lseek:
  %SYS_lseek_rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)
  ret i64 %SYS_lseek_rslt

SYS_read:
  %read_buf = call i8* @get_data_ptr(i64 %arg2)
  %SYS_read_rslt = call i64 (i64, ...) @syscall(i64 0, i64 %arg1, i8* %read_buf, i64 %arg3)
  ret i64 %SYS_read_rslt

SYS_write:
  %write_buf = call i8* @get_data_ptr(i64 %arg2)
  %SYS_write_rslt = call i64 (i64, ...) @syscall(i64 1, i64 %arg1, i8* %write_buf, i64 %arg3)
  ret i64 %SYS_write_rslt

fallback:
  unreachable
}";

const FPFUNCTIONS: &str = "declare double @llvm.sqrt.f64(double %op1)
declare double @llvm.fma.f64(double %op1, double %op2, double %op3)
declare double @llvm.fabs.f64(double %op1)
declare double @llvm.minimum.f64(double %op1, double %op2)
declare double @llvm.maximum.f64(double %op1, double %op2)
declare double @llvm.copysign.f64(double %mag, double %sgn)";

const INIT_ARGV: &str = "define void @init_argv(i32 %0, i32 %1, i8** %2) {
    %4 = icmp sgt i32 %1, 0
    br i1 %4, label %5, label %16
  
  5:                                                ; preds = %3
    %6 = zext i32 %1 to i64
    br label %7
  
  7:                                                ; preds = %5, %29
    %8 = phi i64 [ 0, %5 ], [ %31, %29 ]
    %9 = phi i32 [ %0, %5 ], [ %30, %29 ]
    %10 = getelementptr inbounds i8*, i8** %2, i64 %8
    %11 = load i8*, i8** %10
    %12 = load i8, i8* %11
    %13 = sext i32 %9 to i64
    %14 = call i8* @get_data_ptr(i64 %13)
    store i8 %12, i8* %14
    %15 = icmp eq i8 %12, 0
    br i1 %15, label %29, label %17
  
  16:                                               ; preds = %29, %3
    ret void
  
  17:                                               ; preds = %7, %17
    %18 = phi i64 [ %20, %17 ], [ %13, %7 ]
    %19 = phi i64 [ %21, %17 ], [ 0, %7 ]
    %20 = add i64 %18, 1
    %21 = add nuw nsw i64 %19, 1
    %22 = load i8*, i8** %10
    %23 = getelementptr inbounds i8, i8* %22, i64 %21
    %24 = load i8, i8* %23
    %25 = call i8* @get_data_ptr(i64 %20)
    store i8 %24, i8* %25
    %26 = icmp eq i8 %24, 0
    br i1 %26, label %27, label %17
  
  27:                                               ; preds = %17
    %28 = trunc i64 %20 to i32
    br label %29
  
  29:                                               ; preds = %27, %7
    %30 = phi i32 [ %9, %7 ], [ %28, %27 ]
    %31 = add nuw nsw i64 %8, 1
    %32 = icmp eq i64 %31, %6
    br i1 %32, label %16, label %7
  }
";

const REGISTERS: &str = "

  %reg = alloca %struct.reg
  %freg = alloca %struct.freg
  %reg_byte = bitcast %struct.reg* %reg to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %reg_byte, i8 0, i64 256, i1 false)
  %freg_byte = bitcast %struct.freg* %freg to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %freg_byte, i8 0, i64 256, i1 false)

  %sp = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 2
  store i64 10240, i64* %sp

  ; riscv64-unknown-elf-gcc
  %argc_i64 = sext i32 %argc to i64
  %argc_ptr = call i8* @get_data_ptr(i64 10240)
  %cast_argc_ptr = bitcast i8* %argc_ptr to i64*
  store i64 %argc_i64, i64* %cast_argc_ptr

  %argv_ptr = call i8* @get_data_ptr(i64 10248)
  %cast_argv = bitcast i8** %argv to i8*
  %num = mul i64 %argc_i64, 8
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %argv_ptr, i8* %cast_argv, i64 %num, i1 0)
        
";

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let data_blocks = self
            .data_blocks
            .iter()
            .fold(String::new(), |s, b| s + &format!("{}\n", b));

        let get_data_ptr = if self.data_blocks.is_empty() {
            String::new()
        } else {
            let mut get_data_ptr = format!("define i8* @get_data_ptr(i64 %addr) {{\n");
            let mut data_blocks_iter = self.data_blocks.iter();
            let mut current = data_blocks_iter.next();
            let mut next = data_blocks_iter.next();
            while let Some(cur) = current {
                if let Some(nxt) = next {
                    let Address(addr) = cur.address;
                    let cur_end = addr as usize + cur.bytes.len();
                    get_data_ptr += &format!(
                        "data_{cur}:
  %data_{cur}_start = icmp sle i64 {cur}, %addr
  br i1 %data_{cur}_start, label %data_{cur}_start_true, label %data_{nxt}
data_{cur}_start_true:
  %data_{cur}_end = icmp sgt i64 {cur_end}, %addr
  br i1 %data_{cur}_end, label %data_{cur}_true, label %data_{nxt}
data_{cur}_true:
  %rel_addr_{cur} = sub i64 %addr, {cur}
  %ptr_{cur} = getelementptr [{len} x i8], [{len} x i8]* @data_{cur}, i64 0, i64 %rel_addr_{cur}
  ret i8* %ptr_{cur}
",
                        cur = cur.address,
                        nxt = nxt.address,
                        len = cur.bytes.len()
                    );
                } else {
                    let Address(addr) = cur.address;
                    let cur_end = addr as usize + cur.bytes.len();
                    get_data_ptr += &format!(
                        "data_{cur}:
  %data_{cur}_start = icmp sle i64 {cur}, %addr
  br i1 %data_{cur}_start, label %data_{cur}_start_true, label %fallback
data_{cur}_start_true:
  %data_{cur}_end = icmp sgt i64 {cur_end}, %addr
  br i1 %data_{cur}_end, label %data_{cur}_true, label %fallback
data_{cur}_true:
  %rel_addr_{cur} = sub i64 %addr, {cur}
  %ptr_{cur} = getelementptr [{len} x i8], [{len} x i8]* @data_{cur}, i64 0, i64 %rel_addr_{cur}
  ret i8* %ptr_{cur}
fallback:
  %ptr = inttoptr i64 %addr to i8*
  ret i8* %ptr
",
                        cur = cur.address,
                        len = cur.bytes.len()
                    );
                }
                current = next;
                next = data_blocks_iter.next();
            }
            get_data_ptr += "}";
            get_data_ptr
        };

        // Build each function
        let mut functions = String::new();
        for func in &self.functions {
            functions += format!("{func}");
        }

        // Main dispatcher
        let inst_cnt = 150000;
        let mut table = vec![String::from("i64 0"); inst_cnt];
        for func in &self.functions {
            let f = func[0].instruction_blocks[0].riscv_instruction.address();
            for block in func {
                for b in &block.instruction_blocks {
                    let Addr(addr) = b.riscv_instruction.address();
                    if let Some(name) = self.parsed_funcs.get(&Addr(addr)) {
                        table[addr as usize] = format!(
                            "i64 ptrtoint (i64 (i64, %struct.reg*, %struct.freg*)* @{name} to i64)"
                        );
                    } else {
                        table[addr as usize] = format!(
                            "i64 ptrtoint (i64 (i64, %struct.reg*, %struct.freg*)* @func_{f} to i64)"
                        );
                    }
                }
            }
        }
        let table = table.join(", ");
        let table = format!("@dispatch_table = dso_local global [{inst_cnt} x i64] [{table}]");

        // Main formatting
        write!(
            f,
            "{}

{}

{}

{}

%struct.reg = type {{ i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }}
%struct.freg = type {{ double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double }}
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg)
declare void @llvm.memcpy.p0i8.p0i8.i64(i8*, i8*, i64, i1)
declare dso_local void @exit(i32)
declare dso_local i32 @printf(i8*, ...)
@.str.d = private unnamed_addr constant [14 x i8] c\"#value: %ld#\\0A\\00\", align 1
@.str.f = private unnamed_addr constant [13 x i8] c\"#value: %f#\\0A\\00\", align 1
@.str.s = private unnamed_addr constant [13 x i8] c\"#value: %s#\\0A\\00\", align 1

{table}

define i64 @main(i32 %argc, i8** %argv) {{
{}

%entry_p= alloca i64
store i64 {}, i64* %entry_p
br label %loop
loop:
%entry = load i64, i64* %entry_p
%func_val_ptr = getelementptr [{inst_cnt} x i64], [{inst_cnt} x i64]* @dispatch_table, i64 0, i64 %entry
%func_val = load i64, i64* %func_val_ptr
%func_ptr = inttoptr i64 %func_val to i64 (i64, %struct.reg*, %struct.freg*)*
%target = call i64 %func_ptr(i64 %entry, %struct.reg* %reg, %struct.freg* %freg)
store i64 %target, i64* %entry_p
br label %loop


label_0:
  unreachable
}}

{}

{}
",
SYSCALL,
FPFUNCTIONS,
            get_data_ptr,
            data_blocks,
            REGISTERS,
            self.entry,
            functions,
            self.parsed_irs.join("\n"),
        )
    }
}
