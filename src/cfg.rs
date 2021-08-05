use crate::riscv_isa::{RiscvAddress, RiscvInstruction};

pub type Cfg = Vec<Function>;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub address: RiscvAddress,
    pub basic_blocks: Vec<BasicBlock>,
    pub potential_targets: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub struct BasicBlock {
    pub instructions: Vec<RiscvInstruction>,
    pub continue_target: usize,
    pub jump_target: usize,
}
