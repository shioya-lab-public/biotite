mod ir_rewriter;
mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;
mod syscall_builder;
mod syscall_x86_64;

pub fn run(arch: &str, rv_src: &str, tdata_src: &Option<String>, irs: &Vec<Vec<u8>>, parts: usize) -> Vec<String> {
    let syscall = syscall_builder::run(arch);
    let rv_prog = riscv_parser::run(rv_src, tdata_src);
    let src_funcs = ir_rewriter::run(&rv_prog, irs);
    let ll_prog = llvm_translator::run(rv_prog, src_funcs, syscall);
    ll_prog.in_parts(parts)
}
