mod ir_translator;
mod llvm_isa;
mod llvm_macro;
mod llvm_translator;
mod mem;
mod opt;
mod riscv_isa;
mod riscv_macro;
mod riscv_parser;
mod sys_call;

use crate::llvm_isa::Prog;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn run(
    rv_src: String,
    tdata_src: String,
    arch: Option<String>,
    enable_all_opts: bool,
    disable_all_opts: bool,
    enable_opts: Vec<String>,
    disable_opts: Vec<String>,
    srcs: Vec<PathBuf>,
    ir_dir: PathBuf,
    module_size: usize,
) -> Prog {
    let (rv_prog, syms) = riscv_parser::run(rv_src, tdata_src);
    let mut ll_prog = llvm_translator::run(rv_prog);
    ll_prog.mem = arch.as_ref().map(|arch| mem::run(arch, &ll_prog));
    ll_prog.ir_funcs = ir_translator::run(srcs, ir_dir, &syms, &ll_prog)
        .into_iter()
        .collect();
    ll_prog.sys_call = arch.as_ref().map(|arch| sys_call::run(arch));
    ll_prog.module_size = module_size;
    opt::run(
        ll_prog,
        enable_all_opts,
        disable_all_opts,
        enable_opts,
        disable_opts,
    )
}
