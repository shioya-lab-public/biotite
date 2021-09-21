use crate::riscv_isa::{RiscvAddress, RiscvImmediate, RiscvRegister};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub struct Program {
    pub statics: HashMap<String, String>,
    pub functions: Vec<LlvmFunction>,
}

pub struct LlvmFunction {
    pub name: String,
    pub body: Vec<LlvmInstruction>,
}

pub enum LlvmType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

pub enum LlvmValue {
    GlobalVar(RiscvRegister),
    LocalVar(usize),
    Int(i64),
    Fp(f64),
}

pub enum LlvmOrdering {
    Monotonic,
    Acquire,
    Release,
    AcqRel,
}

pub enum LlvmIntCondition {
    Eq,  //
    Ne,  //
    Ugt, //
    Uge, //
    Ult, //
    Ule, //
    Sgt, //
    Sge, //
    Slt, //
    Sle, //
}

pub enum LlvmFpCondition {
    Eq,  //
    Slt, //
    Sle, //
}

pub enum LlvmOperation {
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

pub enum LlvmInstruction {
    // Terminator Instructions
    Ret,
    ConBr {
        cond: LlvmValue,
        iftrue: String,
        iffalse: String,
    },
    UnconBr(String),
    Label(String),
    Switch {
        value: LlvmValue,
        defaultdest: String,
        targets: Vec<(LlvmValue, String)>,
    },

    // Unary Operations
    Fneg {
        result: LlvmValue,
        ty: LlvmType,
        op1: LlvmValue,
    },

    // Binary Operations
    Add {
        result: LlvmValue,
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
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Sdiv {
        result: LlvmValue,
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
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Srem {
        result: LlvmValue,
        op1: LlvmValue,
        op2: LlvmValue,
    },

    // Bitwise Binary Operations
    Shl {
        result: LlvmValue,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Lshr {
        result: LlvmValue,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Ashr {
        result: LlvmValue,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    And {
        result: LlvmValue,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Or {
        result: LlvmValue,
        op1: LlvmValue,
        op2: LlvmValue,
    },
    Xor {
        result: LlvmValue,
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
    Fence,
    Cmpxchg {
        pointer: LlvmValue,
        cmp: LlvmValue,
        new: LlvmValue,
        success_ordering: LlvmOrdering,
        failure_ordering: LlvmOrdering,
    },
    Atomicrmw {
        operation: LlvmOperation,
        pointer: LlvmValue,
        value: LlvmValue,
        ordering: LlvmOrdering,
    },
    Getelementptr(LlvmValue),

    // Conversion Operations
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

    // Intrinsics
    Sqrt {
        ty: LlvmType,
        result: LlvmValue,
        val: LlvmValue,
    },
    Fma {
        ty: LlvmType,
        result: LlvmValue,
        a: LlvmValue,
        b: LlvmValue,
        c: LlvmValue,
    },
    Fabs {
        ty: LlvmType,
        result: LlvmValue,
        val: LlvmValue,
    },
    Copysign {
        ty: LlvmType,
        result: LlvmValue,
        mag: LlvmValue,
        sgn: LlvmValue,
    },
    Minimum {
        ty: LlvmType,
        val0: LlvmValue,
        val1: LlvmValue,
    },
    Maximum {
        ty: LlvmType,
        val0: LlvmValue,
        val1: LlvmValue,
    },
}

// impl Display for LlvmCondition {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         use LlvmCondition::*;

//         match self {
//             Eq => write!(f, "eq",),
//             Ne => write!(f, "ne",),
//             Sge => write!(f, "sge",),
//             Uge => write!(f, "uge",),
//             Slt => write!(f, "slt",),
//             Ult => write!(f, "ult",),
//             Sle => write!(f, "sle",),
//         }
//     }
// }

// impl Display for LlvmType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         use LlvmType::*;

//         match self {
//             I8 => write!(f, "i8",),
//             U8 => write!(f, "i8",), // LLVM does not distinguish signed and unsigned integers.
//             I16 => write!(f, "i16",),
//             U16 => write!(f, "i16",),
//             I32 => write!(f, "i32",),
//             U32 => write!(f, "i32",),
//             I64 => write!(f, "i64",),
//         }
//     }
// }
