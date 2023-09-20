mod ir_translator;
mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod opt;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;
mod sys_call;

use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn run(
    rv_src: String,
    tdata_src: Option<String>,
    arch: Option<String>,
    enable_all_opts: bool,
    disable_all_opts: bool,
    enable_opts: Vec<String>,
    disable_opts: Vec<String>,
    srcs: Vec<PathBuf>,
    ir_dir: PathBuf,
) -> String {
    let (rv_prog, syms) = riscv_parser::run(rv_src, tdata_src);
    let sys_call = sys_call::build(arch);
    let ir_funcs = ir_translator::run(srcs, &syms, ir_dir);
    let ll_prog = llvm_translator::run(rv_prog, sys_call, ir_funcs);
    let opted_prog = opt::optimize(
        ll_prog,
        enable_all_opts,
        disable_all_opts,
        enable_opts,
        disable_opts,
    );
    opted_prog.to_string()
}
