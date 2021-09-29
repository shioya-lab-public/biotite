use crate::riscv_isa::{RiscvImmediate, RiscvRegister};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statics: HashMap<String, (String, LlvmType)>,
    pub functions: Vec<LlvmFunction>,
}

#[derive(Debug, PartialEq)]
pub struct LlvmFunction {
    pub name: String,
    pub body: Vec<LlvmInstruction>,
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

#[derive(Debug, PartialEq, Clone)]
pub enum LlvmType {
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
}

#[derive(Debug, PartialEq)]
pub enum LlvmOrdering {
    Monotonic,
    Acquire,
    Release,
    AcqRel,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum LlvmIntCondition {
    Eq,
    Ne,
    Ugt,
    Uge,
    Ult,
    Ule,
    Sgt,
    Sge,
    Slt,
    Sle,
}

#[derive(Debug, PartialEq)]
pub enum LlvmFpCondition {
    Oeq,
    Olt,
    Ole,
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
    Cmpxchg {
        ty: LlvmType,
        pointer: LlvmValue,
        cmp: LlvmValue,
        new: LlvmValue,
        success_ordering: LlvmOrdering,
        failure_ordering: LlvmOrdering,
    },
    Atomicrmw {
        operation: LlvmOperation,
        ty: LlvmType,
        pointer: LlvmValue,
        value: LlvmValue,
        ordering: LlvmOrdering,
    },
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
