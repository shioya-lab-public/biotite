use crate::cfg::Cfg;
use crate::riscv_isa::RiscvInstruction;

pub struct CfgBuilder {}

impl CfgBuilder {
    pub fn new() -> Self {
        CfgBuilder {}
    }

    pub fn run(&mut self, riscv_instructions: Vec<RiscvInstruction>) -> Cfg {
        todo!();
    }
}
