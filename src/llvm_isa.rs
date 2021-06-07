use super::riscv_isa::RiscvRegister;
use std::fmt::{Display, Formatter, Result};

pub enum LlvmInstruction {
    Add(LlvmValue, LlvmType, LlvmValue, LlvmValue),
    Ret(LlvmType, LlvmValue),
}

#[derive(Clone, Copy)]
pub enum LlvmType {
    I64,
}

pub enum LlvmValue {
    Literal(i64),
    Register(RiscvRegister, u64),
}

impl Display for LlvmInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LlvmInstruction::Add(rd, ty, rs1, rs2) => {
                write!(f, "{} = add {} {}, {}", rd, ty, rs1, rs2)
            }
            LlvmInstruction::Ret(ty, rs1) => write!(f, "ret {} {}", ty, rs1),
        }
    }
}

impl Display for LlvmType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LlvmType::I64 => write!(f, "i64"),
        }
    }
}

impl Display for LlvmValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LlvmValue::Literal(value) => write!(f, "{}", value),
            LlvmValue::Register(reg, sub) => write!(f, "%{:?}_{}", reg, sub),
        }
    }
}
