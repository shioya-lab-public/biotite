#[macro_use]
extern crate lazy_static;

mod cfg;
mod cfg_builder;
mod llvm_isa;
mod llvm_serializer;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;

use cfg_builder::CfgBuilder;
use llvm_translator::LlvmTranslator;

pub fn run(source: &str) -> String {
    let indirect_targets = riscv_parser::parse_rodata(source);
    let mut statics = riscv_parser::parse_sdata(source);
    statics.extend(riscv_parser::parse_sbss(source));
    let rv_insts = riscv_parser::parse_text(source);
    let cfg = CfgBuilder::new(rv_insts, indirect_targets).run();
    let ll_program = LlvmTranslator::new(cfg, statics).run();
    llvm_serializer::serialize(ll_program)
}
