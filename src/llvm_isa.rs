use crate::riscv_isa::{RiscvAddress, RiscvImmediate, RiscvRegister};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub type Program = Vec<LlvmFunction>;

pub struct LlvmFunction {
    pub name: String,
    pub body: Vec<LlvmInstruction>,
}

#[derive(Debug)]
pub enum LlvmCondition {
    Eq,
    Ne,
    Sge,
    Uge,
    Slt,
    Ult,
    Sle,
}

impl Display for LlvmCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use LlvmCondition::*;

        match self {
            Eq => write!(f, "eq",),
            Ne => write!(f, "ne",),
            Sge => write!(f, "sge",),
            Uge => write!(f, "uge",),
            Slt => write!(f, "slt",),
            Ult => write!(f, "ult",),
            Sle => write!(f, "sle",),
        }
    }
}

#[derive(Debug)]
pub enum LlvmType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
}

impl Display for LlvmType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use LlvmType::*;

        match self {
            I8 => write!(f, "i8",),
            U8 => write!(f, "i8",), // LLVM does not distinguish signed and unsigned integers.
            I16 => write!(f, "i16",),
            U16 => write!(f, "i16",),
            I32 => write!(f, "i32",),
            U32 => write!(f, "i32",),
            I64 => write!(f, "i64",),
        }
    }
}

#[derive(Debug)]
pub enum LlvmInstruction {
    Label(String),
    Add {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Addi {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    And {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Andi {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    Icmp {
        condition: LlvmCondition,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Br {
        iftrue: String,
        iffalse: String,
    },
    DirectBr(String),
    Switch {
        register: RiscvRegister,
        targets: HashMap<RiscvAddress, RiscvAddress>,
    },
    Call(String),
    Load {
        ty: LlvmType,
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    Shli12 {
        result: RiscvRegister,
        op1: RiscvImmediate,
    },
    Or {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Ori {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    Save {
        ty: LlvmType,
        op1: RiscvRegister,
        op2: RiscvImmediate,
        source: RiscvRegister,
    },
    Shl {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Shli {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    Ashr {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Ashri {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    Lshr {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Lshri {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    Sub {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Xor {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvRegister,
    },
    Xori {
        result: RiscvRegister,
        op1: RiscvRegister,
        op2: RiscvImmediate,
    },
    Ret,
}
