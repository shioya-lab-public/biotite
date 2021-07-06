#[macro_use]
extern crate lazy_static;

mod cfg_builder;
mod cfg_translator;
mod llvm_serializer;
mod riscv_parser;

mod cfg;
mod llvm_isa;
mod riscv_isa;

use cfg_builder::CfgBuilder;
use cfg_translator::CfgTranslator;

pub struct Translator {
    cfg_builder: CfgBuilder,
    cfg_translator: CfgTranslator,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            cfg_builder: CfgBuilder::new(),
            cfg_translator: CfgTranslator::new(),
        }
    }

    pub fn run(&mut self, source: String) -> String {
        let riscv_insts = riscv_parser::parse(source);
        let cfg = self.cfg_builder.run(riscv_insts);
        let llvm_insts = self.cfg_translator.run(cfg);
        llvm_serializer::serialize(llvm_insts)
    }
}
