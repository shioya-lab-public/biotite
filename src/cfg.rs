use crate::riscv_isa::{RiscvInstruction, RiscvAddress};

pub type Cfg = Vec<Function>;

pub struct Function {
    pub name: String,
    pub address: RiscvAddress,
    pub basic_blocks: Vec<BasicBlock>,
}

pub struct BasicBlock {
    pub index: usize,
    pub instructions: Vec<RiscvInstruction>,
    pub continue_target: Option<usize>,
    pub alternative_target: Option<usize>,
}
