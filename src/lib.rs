mod llvm_isa;
mod llvm_macro;
mod llvm_parser;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;

use std::collections::HashMap;
use std::path::PathBuf;

use llvm_translator::Translator;
use riscv_parser::Parser;

pub fn run(
    rv_source: &str,
    auto_split_functions: bool,
    elf: &Option<String>,
    irs: &Vec<Vec<u8>>,
) -> String {
    let rv_program = Parser::new().run(rv_source);
    let (mut parsed_funcs, mut parsed_irs) = (HashMap::new(), Vec::new());
    for ir in irs {
        let (funcs, ir) = llvm_parser::parse(ir, &rv_program);
        parsed_funcs.extend(funcs);
        parsed_irs.push(ir);
    }
    let ll_program = Translator::new().run(rv_program, parsed_funcs, parsed_irs);
    format!("{}", ll_program)
}
