mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod opts;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;
mod syscalls;

pub fn run(
    rv_src: &str,
    tdata_src: Option<&str>,
    arch: &str,
    enable_all_opts: bool,
    enable_opts: &Vec<String>,
    disable_opts: &Vec<String>,
    disable_all_opts: bool,
    src_funcs: &Vec<String>,
    parts: usize,
    verbose: bool,
) -> Vec<String> {
    let rv_prog = riscv_parser::run(rv_src, tdata_src, verbose);
    let syscalls = syscalls::build(arch);
    let ll_prog = llvm_translator::run(rv_prog, syscalls, src_funcs, verbose);
    let opted_prog = opts::optimize(
        ll_prog,
        enable_all_opts,
        enable_opts,
        disable_opts,
        disable_all_opts,
        verbose,
    );
    opted_prog.in_parts(parts)
}
