use crate::riscv_isa::{RiscvImmediate, RiscvRegister};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

const STACK_SIZE: usize = 1024 * 10;

const GLOBAL: &str = "
; Dump registers:
; declare dso_local i32 @printf(i8*, ...)
; @.str = private unnamed_addr constant [19 x i8] c\"*** Debug ***: %d\\0A\\00\", align 1
; %val = load i64, i64* @reg.zero
; call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.str, i64 0, i64 0), i64 %val)

declare float @llvm.sqrt.f32(float %value)
declare double @llvm.sqrt.f64(double %value)
declare float @llvm.fma.f32(float %a, float %b, float %c)
declare double @llvm.fma.f64(double %a, double %b, double %c)
declare float @llvm.fabs.f32(float %value)
declare double @llvm.fabs.f64(double %value)
declare float @llvm.minimum.f32(float %op1, float %op2)
declare double @llvm.minimum.f64(double %op1, double %op2)
declare float @llvm.maximum.f32(float %op1, float %op2)
declare double @llvm.maximum.f64(double %op1, double %op2)
declare float @llvm.copysign.f32(float %mag, float %sign)
declare double @llvm.copysign.f64(double %mag, double %sign)

declare dso_local i64 @syscall(i64, ...)

@reg.zero = global i64 zeroinitializer
@reg.ra = global i64 zeroinitializer
@reg.sp = global i64 {stack_size}
@reg.gp = global i64 zeroinitializer
@reg.tp = global i64 zeroinitializer
@reg.t0 = global i64 zeroinitializer
@reg.t1 = global i64 zeroinitializer
@reg.t2 = global i64 zeroinitializer
@reg.s0 = global i64 zeroinitializer
@reg.s1 = global i64 zeroinitializer
@reg.a0 = global i64 zeroinitializer
@reg.a1 = global i64 zeroinitializer
@reg.a2 = global i64 zeroinitializer
@reg.a3 = global i64 zeroinitializer
@reg.a4 = global i64 zeroinitializer
@reg.a5 = global i64 zeroinitializer
@reg.a6 = global i64 zeroinitializer
@reg.a7 = global i64 zeroinitializer
@reg.s2 = global i64 zeroinitializer
@reg.s3 = global i64 zeroinitializer
@reg.s4 = global i64 zeroinitializer
@reg.s5 = global i64 zeroinitializer
@reg.s6 = global i64 zeroinitializer
@reg.s7 = global i64 zeroinitializer
@reg.s8 = global i64 zeroinitializer
@reg.s9 = global i64 zeroinitializer
@reg.s10 = global i64 zeroinitializer
@reg.s11 = global i64 zeroinitializer
@reg.t3 = global i64 zeroinitializer
@reg.t4 = global i64 zeroinitializer
@reg.t5 = global i64 zeroinitializer
@reg.t6 = global i64 zeroinitializer

@reg.ft0 = global double zeroinitializer
@reg.ft1 = global double zeroinitializer
@reg.ft2 = global double zeroinitializer
@reg.ft3 = global double zeroinitializer
@reg.ft4 = global double zeroinitializer
@reg.ft5 = global double zeroinitializer
@reg.ft6 = global double zeroinitializer
@reg.ft7 = global double zeroinitializer
@reg.fs0 = global double zeroinitializer
@reg.fs1 = global double zeroinitializer
@reg.fa0 = global double zeroinitializer
@reg.fa1 = global double zeroinitializer
@reg.fa2 = global double zeroinitializer
@reg.fa3 = global double zeroinitializer
@reg.fa4 = global double zeroinitializer
@reg.fa5 = global double zeroinitializer
@reg.fa6 = global double zeroinitializer
@reg.fa7 = global double zeroinitializer
@reg.fs2 = global double zeroinitializer
@reg.fs3 = global double zeroinitializer
@reg.fs4 = global double zeroinitializer
@reg.fs5 = global double zeroinitializer
@reg.fs6 = global double zeroinitializer
@reg.fs7 = global double zeroinitializer
@reg.fs8 = global double zeroinitializer
@reg.fs9 = global double zeroinitializer
@reg.fs10 = global double zeroinitializer
@reg.fs11 = global double zeroinitializer
@reg.ft8 = global double zeroinitializer
@reg.ft9 = global double zeroinitializer
@reg.ft10 = global double zeroinitializer
@reg.ft11 = global double zeroinitializer

@reg.stack = global [{stack_size} x i8] zeroinitializer

";

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statics: HashMap<String, (String, LlvmType)>,
    pub functions: Vec<LlvmFunction>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut s = GLOBAL.replace("{stack_size}", &STACK_SIZE.to_string());
        for func in &self.functions {
            s += &format!("{}\n", func);
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
pub struct LlvmFunction {
    pub name: String,
    pub body: Vec<LlvmInstruction>,
}

impl Display for LlvmFunction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.name == "main" {
            let mut s = format!("define i64 @{}() {{\n", self.name);
            for inst in &self.body[0..self.body.len() - 1] {
                if let LlvmInstruction::Label(label) = inst {
                    s += &format!("{}:\n", label);
                } else {
                    s += &format!("  {}\n", inst);
                }
            }
            s += "  %ret = load i64, i64* @reg.a0";
            s += "  ret i64 %ret";
            s += "}\n";
            write!(f, "{}", s)
        } else {
            let mut s = format!("define void @{}() {{\n", self.name);
            for inst in &self.body {
                if let LlvmInstruction::Label(label) = inst {
                    s += &format!("{}:\n", label);
                } else {
                    s += &format!("  {}\n", inst);
                }
            }
            s += "}\n";
            write!(f, "{}", s)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LlvmValue {
    GlobalVar(String),
    LocalVar(usize),
    Int(i64),
}

impl From<RiscvRegister> for LlvmValue {
    fn from(reg: RiscvRegister) -> Self {
        let mut reg_str = format!("reg.{:?}", reg);
        reg_str.make_ascii_lowercase();
        LlvmValue::GlobalVar(reg_str)
    }
}

impl From<usize> for LlvmValue {
    fn from(temp: usize) -> Self {
        LlvmValue::LocalVar(temp)
    }
}

impl From<RiscvImmediate> for LlvmValue {
    fn from(RiscvImmediate(imm): RiscvImmediate) -> Self {
        LlvmValue::Int(imm)
    }
}

impl From<i64> for LlvmValue {
    fn from(val: i64) -> Self {
        LlvmValue::Int(val)
    }
}

impl Display for LlvmValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use LlvmValue::*;
        let s = match self {
            GlobalVar(name) => format!("@{}", name),
            LocalVar(no) => format!("%temp_{}", no),
            Int(int) => format!("{}", int),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LlvmType {
    I1,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
}

impl Display for LlvmType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use LlvmType::*;
        let s = match self {
            I1 => "i1",
            I8 => "i8",
            I16 => "i16",
            I32 => "i32",
            I64 => "i64",
            I128 => "i128",
            F32 => "float",
            F64 => "double",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
pub enum LlvmOrdering {
    // Monotonic,
    // Acquire,
    // Release,
    AcqRel,
}

impl Display for LlvmOrdering {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use LlvmOrdering::*;
        let s = match self {
            AcqRel => "acq_rel",
        };
        write!(f, "{}", s)
    }
}

// #[derive(Debug, PartialEq)]
// pub enum LlvmOperation {
//     Xchg,
//     Add,
//     And,
//     Or,
//     Xor,
//     Max,
//     Min,
//     Umax,
//     Umin,
// }

#[derive(Debug, PartialEq)]
pub enum LlvmIntCondition {
    Eq,
    Ne,
    Uge,
    Ult,
    Sgt,
    Sge,
    Slt,
    Sle,
}

impl Display for LlvmIntCondition {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use LlvmIntCondition::*;
        let s = match self {
            Eq => "eq",
            Ne => "ne",
            Uge => "uge",
            Ult => "ult",
            Sgt => "sgt",
            Sge => "sge",
            Slt => "slt",
            Sle => "sle",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
pub enum LlvmFpCondition {
    Oeq,
    Olt,
    Ole,
}

impl Display for LlvmFpCondition {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use LlvmFpCondition::*;
        let s = match self {
            Oeq => "oeq",
            Olt => "olt",
            Ole => "ole",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
pub enum LlvmInstruction {
    Label(String),

    // Terminator Instructions
    Ret,
    ConBr {
        cond: LlvmValue,
        iftrue: String,
        iffalse: String,
    },
    UnconBr(String),
    Switch {
        ty: LlvmType,
        value: LlvmValue,
        defaultdest: String,
        targets: Vec<(LlvmType, LlvmValue, String)>,
    },
    Unreachable,

    // Unary Operations
    Fneg {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
    },

    // Binary Operations
    Add {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Fadd {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Sub {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Fsub {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Mul {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Fmul {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Udiv {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Sdiv {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Fdiv {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Urem {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Srem {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },

    // Bitwise Binary Operations
    Shl {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Lshr {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Ashr {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    And {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Or {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Xor {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },

    // Memory Access and Addressing Operations
    Load {
        result: LlvmValue,
        ty: LlvmType,
        pointer: LlvmValue,
    },
    Store {
        ty: LlvmType,
        value: LlvmValue,
        pointer: LlvmValue,
    },
    Fence(LlvmOrdering),
    // Cmpxchg {
    //     ty: LlvmType,
    //     pointer: LlvmValue,
    //     cmp: LlvmValue,
    //     new: LlvmValue,
    //     success_ordering: LlvmOrdering,
    //     failure_ordering: LlvmOrdering,
    // },
    // Atomicrmw {
    //     operation: LlvmOperation,
    //     ty: LlvmType,
    //     pointer: LlvmValue,
    //     value: LlvmValue,
    //     ordering: LlvmOrdering,
    // },
    Getelementptr {
        result: LlvmValue,
        index: LlvmValue,
    },

    // Conversion Operations
    Trunc {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Zext {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Sext {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Fptrunc {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Fpext {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Fptoui {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Fptosi {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Uitofp {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Sitofp {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },
    Bitcast {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
        ty2: LlvmType,
    },

    // Other Operations
    Icmp {
        result: LlvmValue,
        cond: LlvmIntCondition,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Fcmp {
        result: LlvmValue,
        cond: LlvmFpCondition,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Call(String),

    // Standard C/C++ Library Intrinsics
    Sqrt {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
    },
    Fma {
        result: LlvmValue,
        ty: LlvmType,
        a: LlvmValue,
        b: LlvmValue,
        c: LlvmValue,
    },
    Fabs {
        result: LlvmValue,
        ty: LlvmType,
        value: LlvmValue,
    },
    Minimum {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Maximum {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Copysign {
        result: LlvmValue,
        ty: LlvmType,
        mag: LlvmValue,
        sign: LlvmValue,
    },

    // System Calls
    Syscall {
        result: LlvmValue,
        no: LlvmValue,
        arg1: LlvmValue,
        arg2: LlvmValue,
        arg3: LlvmValue,
        arg4: LlvmValue,
        arg5: LlvmValue,
        arg6: LlvmValue,
    },
}

impl Display for LlvmInstruction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use LlvmInstruction::*;
        let s = match self {
            // Terminator Instructions
            Label(_) => unreachable!(),
            Ret => String::from("ret void"),
            ConBr {
                cond,
                iftrue,
                iffalse,
            } => format!("br i1 {}, label %{}, label %{}", cond, iftrue, iffalse),
            UnconBr(label) => format!("br label %{}", label),
            Switch {
                ty,
                value,
                defaultdest,
                targets,
            } => {
                let mut s = format!("switch {} {}, label %{} [ ", ty, value, defaultdest);
                for (ty, value, label) in targets {
                    s += &format!("{} {}, label %{} ", ty, value, label);
                }
                s += "]";
                s
            }
            Unreachable => String::from("unreachable"),

            // Unary Operations
            Fneg { result, ty, op1 } => format!("{} = fneg {} {}", result, ty, op1),

            // Binary Operations
            Add {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = add {} {}, {}", result, ty, op1, op2),
            Fadd {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = fadd {} {}, {}", result, ty, op1, op2),
            Sub {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = sub {} {}, {}", result, ty, op1, op2),
            Fsub {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = fsub {} {}, {}", result, ty, op1, op2),
            Mul {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = mul {} {}, {}", result, ty, op1, op2),
            Fmul {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = fmul {} {}, {}", result, ty, op1, op2),
            Udiv {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = udiv {} {}, {}", result, ty, op1, op2),
            Sdiv {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = sdiv {} {}, {}", result, ty, op1, op2),
            Fdiv {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = fdiv {} {}, {}", result, ty, op1, op2),
            Urem {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = urem {} {}, {}", result, ty, op1, op2),
            Srem {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = srem {} {}, {}", result, ty, op1, op2),

            // Bitwise Binary Operations
            Shl {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = shl {} {}, {}", result, ty, op1, op2),
            Lshr {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = lshr {} {}, {}", result, ty, op1, op2),
            Ashr {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = ashr {} {}, {}", result, ty, op1, op2),
            And {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = and {} {}, {}", result, ty, op1, op2),
            Or {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = or {} {}, {}", result, ty, op1, op2),
            Xor {
                result,
                ty,
                op1,
                op2,
            } => format!("{} = xor {} {}, {}", result, ty, op1, op2),

            // Memory Access and Addressing Operations
            Load {
                result,
                ty,
                pointer,
            } => format!("{} = load {}, {}* {}", result, ty, ty, pointer),
            Store { ty, value, pointer } => format!("store {} {}, {}* {}", ty, value, ty, pointer),
            Fence(ord) => format!("fence {}", ord),
            Getelementptr { result, index } => format!(
                "{} = getelementptr [{} x i8], [{} x i8]* @reg.stack, i8 0, i64 {}",
                result, STACK_SIZE, STACK_SIZE, index
            ),

            // Conversion Operations
            Trunc {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = trunc {} {} to {}", result, ty, value, ty2),
            Zext {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = zext {} {} to {}", result, ty, value, ty2),
            Sext {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = sext {} {} to {}", result, ty, value, ty2),
            Fptrunc {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = fptrunc {} {} to {}", result, ty, value, ty2),
            Fpext {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = fpext {} {} to {}", result, ty, value, ty2),
            Fptoui {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = fptoui {} {} to {}", result, ty, value, ty2),
            Fptosi {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = fptosi {} {} to {}", result, ty, value, ty2),
            Uitofp {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = uitofp {} {} to {}", result, ty, value, ty2),
            Sitofp {
                result,
                ty,
                value,
                ty2,
            } => format!("{} = sitofp {} {} to {}", result, ty, value, ty2),
            Bitcast {
                result,
                ty,
                value,
                ty2,
            } => {
                if let (LlvmType::I8, _) | (_, LlvmType::I8) = (ty, ty2) {
                    format!("{} = bitcast {}* {} to {}*", result, ty, value, ty2)
                } else {
                    format!("{} = bitcast {} {} to {}", result, ty, value, ty2)
                }
            }

            // Other Operations
            Icmp {
                result,
                cond,
                ty,
                op1,
                op2,
            } => format!("{} = icmp {} {} {}, {}", result, cond, ty, op1, op2),
            Fcmp {
                result,
                cond,
                ty,
                op1,
                op2,
            } => format!("{} = fcmp {} {} {}, {}", result, cond, ty, op1, op2),
            Call(name) => format!("call void @{}()", name),

            // Standard C/C++ Library Intrinsics
            Sqrt { result, ty, value } => match ty {
                LlvmType::F32 => {
                    format!("{} = call {} @llvm.sqrt.f32({} {})", result, ty, ty, value)
                }
                LlvmType::F64 => {
                    format!("{} = call {} @llvm.sqrt.f64({} {})", result, ty, ty, value)
                }
                _ => unreachable!(),
            },
            Fma {
                result,
                ty,
                a,
                b,
                c,
            } => match ty {
                LlvmType::F32 => format!(
                    "{} = call {} @llvm.fma.f32({} {}, {} {}, {} {})",
                    result, ty, ty, a, ty, b, ty, c
                ),
                LlvmType::F64 => format!(
                    "{} = call {} @llvm.fma.f64({} {}, {} {}, {} {})",
                    result, ty, ty, a, ty, b, ty, c
                ),
                _ => unreachable!(),
            },
            Fabs { result, ty, value } => match ty {
                LlvmType::F32 => {
                    format!("{} = call {} @llvm.fabs.f32({} {})", result, ty, ty, value)
                }
                LlvmType::F64 => {
                    format!("{} = call {} @llvm.fabs.f64({} {})", result, ty, ty, value)
                }
                _ => unreachable!(),
            },
            Minimum {
                result,
                ty,
                op1,
                op2,
            } => match ty {
                LlvmType::F32 => format!(
                    "{} = call {} @llvm.minimum.f32({} {}, {} {})",
                    result, ty, ty, op1, ty, op2
                ),
                LlvmType::F64 => format!(
                    "{} = call {} @llvm.minimum.f64({} {}, {} {})",
                    result, ty, ty, op1, ty, op2
                ),
                _ => unreachable!(),
            },
            Maximum {
                result,
                ty,
                op1,
                op2,
            } => match ty {
                LlvmType::F32 => format!(
                    "{} = call {} @llvm.maximum.f32({} {}, {} {})",
                    result, ty, ty, op1, ty, op2
                ),
                LlvmType::F64 => format!(
                    "{} = call {} @llvm.maximum.f64({} {}, {} {})",
                    result, ty, ty, op1, ty, op2
                ),
                _ => unreachable!(),
            },
            Copysign {
                result,
                ty,
                mag,
                sign,
            } => match ty {
                LlvmType::F32 => format!(
                    "{} = call {} @llvm.copysign.f32({} {}, {} {})",
                    result, ty, ty, mag, ty, sign
                ),
                LlvmType::F64 => format!(
                    "{} = call {} @llvm.copysign.f64({} {}, {} {})",
                    result, ty, ty, mag, ty, sign
                ),
                _ => unreachable!(),
            },

            // System Calls
            Syscall {
                result,
                no,
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6,
            } => format!(
                "{} = call i64 @syscall(i64 {}, i64 {}, i64 {}, i64 {}, i64 {}, i64 {}, i64 {})",
                result, no, arg1, arg2, arg3, arg4, arg5, arg6
            ),
        };
        write!(f, "{}", s)
    }
}
