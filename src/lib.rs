#[macro_use]
extern crate lazy_static;

mod riscv_isa;
mod riscv_macro;
mod riscv_parser;
// mod cfg;
// mod cfg_builder;
// mod llvm_isa;
// mod llvm_translator;
// mod llvm_serializer;

// use cfg_builder::CfgBuilder;

pub fn run(source: &str) -> String {
    let jump_targets = riscv_parser::parse_rodata(source);
    let rv_insts = riscv_parser::parse_text(source);
    String::new()
    // let cfg = CfgBuilder::new(rv_insts, jump_targets).run();
    // let ll_program = llvm_translator::translate(cfg);
    // llvm_serializer::serialize(ll_program)
}
