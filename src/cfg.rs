use crate::riscv_isa::{RiscvAddress, RiscvInstruction};
use std::collections::HashMap;

pub type Cfg = Vec<RiscvFunction>;

#[derive(Debug, PartialEq)]
pub struct RiscvFunction {
    pub name: String,
    pub basic_blocks: Vec<BasicBlock>,
    pub indirect_targets: HashMap<RiscvAddress, usize>,
}

#[derive(Debug, PartialEq)]
pub struct BasicBlock {
    pub instructions: Vec<RiscvInstruction>,
    pub continue_target: Option<usize>,
    pub jump_target: Option<usize>,
}
