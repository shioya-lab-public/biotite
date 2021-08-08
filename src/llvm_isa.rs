use crate::riscv_isa::{RiscvImmediate, RiscvRegister};
// use std::fmt::{Display, Formatter, Result};

pub type Program = Vec<LlvmFunction>;

pub struct LlvmFunction {
    pub name: String,
    pub body: Vec<LlvmInstruction>,
}

// impl Display for LlvmFunction {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let mut body_str = self
//             .body
//             .iter()
//             .map(|inst| format!("    {}\n", inst))
//             .reduce(|insts, inst| insts + &inst)
//             .unwrap();
//         write!(
//             f,
//             "define {} @{}({}) {{\n{}}}\n",
//             self.return_type, self.name, param_str, body_str
//         )
//     }
// }

pub enum LlvmCondition {
    Eq,
    Ne,
    Sge,
    Uge,
    Slt,
    Ult,
}

pub enum LlvmType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
}

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
    IndirectBr {
        register: RiscvRegister,
        labels: Vec<String>,
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

// impl Display for LlvmInstruction {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self {
//             LlvmInstruction::Ret(ty, val) => write!(f, "ret {} {}", ty, val),
//         }
//     }
// }
