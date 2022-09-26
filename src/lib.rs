mod ir_rewriter;
mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;
mod syscall;
mod syscall_x86_64;

pub fn run(rv_src: &str, irs: &Vec<Vec<u8>>, arch: &str) -> String {
    let syscall = syscall::build(arch);
    let rv_prog = riscv_parser::run(rv_src);
    let src_funcs = ir_rewriter::run(&rv_prog, irs);
    let ll_prog = llvm_translator::run(rv_prog, src_funcs, syscall);
    ll_prog.to_string()
}
