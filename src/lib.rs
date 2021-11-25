#[macro_use]
extern crate lazy_static;

mod llvm_translator;
mod riscv_parser;
mod riscv_isa;
mod riscv_macro;
mod llvm_isa;
mod llvm_macro;

use riscv_parser::RiscvParser;
use llvm_translator::LlvmTranslator;

pub fn run(rv_source: &str) -> String {
    let rv_program = RiscvParser::new(rv_source).run();
    let ll_program = LlvmTranslator::new(rv_program).run();
    format!("{}", ll_program)
}
