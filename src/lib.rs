#[macro_use]
extern crate lazy_static;

mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;

use llvm_translator::LlvmTranslator;
use riscv_parser::RiscvParser;

pub fn run(rv_source: &str) -> String {
    let rv_program = RiscvParser::new(rv_source).run();
    let ll_program = LlvmTranslator::new(rv_program).run();
    format!("{}", ll_program)
}
