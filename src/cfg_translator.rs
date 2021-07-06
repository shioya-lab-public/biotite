use crate::cfg::Cfg;
use crate::llvm_isa::LlvmProgram;

pub struct CfgTranslator {}

impl CfgTranslator {
    pub fn new() -> Self {
        CfgTranslator {}
    }

    pub fn run(&mut self, cfg: Cfg) -> LlvmProgram {
        todo!()
    }
}
