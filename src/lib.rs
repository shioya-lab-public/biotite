mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;

use llvm_translator::Translator;
use riscv_parser::Parser;

pub fn run(rv_source: &str, abi: &Option<String>) -> String {
    let rv_program = Parser::new().run(rv_source, abi);
    let ll_program = Translator::new().run(rv_program);
    format!("{}", ll_program)
}
