#![recursion_limit = "256"]

#[macro_use]
extern crate lazy_static;

mod cfg_builder;
mod cfg_translator;
mod llvm_serializer;
mod riscv_parser;

mod cfg;
mod llvm_isa;
mod riscv_isa;
mod riscv_macro;

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
        let potential_targets = riscv_parser::parse_rodata(&source);
        let riscv_insts = riscv_parser::parse_text(source);
        let cfg = self.cfg_builder.run(riscv_insts, potential_targets);
        let llvm_insts = self.cfg_translator.run(cfg);
        llvm_serializer::serialize(llvm_insts)
    }
}
