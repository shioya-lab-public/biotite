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

pub fn run(source: String) -> String {
    let potential_targets = riscv_parser::parse_rodata(&source);
    let riscv_insts = riscv_parser::parse_text(&source);
    let cfg = CfgBuilder::new(riscv_insts, potential_targets).run();
    let program = cfg_translator::run(cfg);
    llvm_serializer::serialize(program)
}
