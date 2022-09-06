mod ir_rewriter;
mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;

pub fn run(rv_src: &str, irs: &Vec<Vec<u8>>) -> String {
    let rv_prog = riscv_parser::run(rv_src);
    let src_funcs = ir_rewriter::run(&rv_prog, irs);
    let ll_prog = llvm_translator::run(rv_prog, &src_funcs);
    ll_prog.to_string()
}
