#[macro_use]
extern crate lazy_static;

mod cfg;
mod cfg_builder;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;
// mod llvm_isa;
// mod llvm_translator;
// mod llvm_serializer;

use cfg_builder::CfgBuilder;

pub fn run(source: &str) -> String {
    let indirect_targets = riscv_parser::parse_rodata(source);
    let rv_insts = riscv_parser::parse_text(source);
    let cfg = CfgBuilder::new(rv_insts, indirect_targets).run();
    // let ll_program = llvm_translator::translate(cfg);
    // llvm_serializer::serialize(ll_program)
    String::new()
}
