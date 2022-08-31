// mod ir_rewriter;
// mod llvm_isa;
// mod llvm_macro;
// mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;

// use ir_rewriter::Rewriter;
// use llvm_translator::Translator;
// use riscv_parser::Parser;

pub fn run(rv_src: &str, irs: &Vec<Vec<u8>>, jobs: usize) -> String {
    // let rv_prog = Parser::new(jobs).run(rv_src);
    // let src_funcs = Rewriter::new(jobs).run(&rv_prog, irs);
    // let ll_prog = Translator::new(jobs).run(rv_prog, &src_funcs);
    // ll_prog.to_string()
    String::new()
}
