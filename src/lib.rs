mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;

use std::path::PathBuf;

use llvm_translator::Translator;
use riscv_parser::Parser;

pub fn run(rv_source: &str, auto_split_functions: bool, elf: &Option<String>, irs: &Vec<PathBuf>) -> String {
    let rv_program = Parser::new().run(rv_source);
    let ll_program = Translator::new().run(rv_program);
    format!("{}", ll_program)
}
