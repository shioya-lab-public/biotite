mod direct_branches;
mod direct_jalr;
mod direct_jumps;
mod global2stack;
mod longjmp_except;
mod native_mem_func;
mod native_stack;
mod offset_mem_op;
mod split_functions;
mod static_data;
mod trans_static_dyn_funcs;

use crate::llvm_isa::Program;

pub fn optimize(
    prog: Program,
    enable_all_opts: bool,
    enable_opts: &Vec<String>,
    disable_opts: &Vec<String>,
    disable_all_opts: bool,
    verbose: bool,
) -> Program {
    let opts = vec!["all"];
    let available_opts = vec![
        "split_functions",
        "direct_jumps",
        "direct_branches",
        "direct_jalr",
        "native_stack",
        "static_data",
        "trans_static_dyn_funcs",
        "longjmp_except",
        "global2stack",
        "native_mem_func",
        "offset_mem_op",
    ];
    match opts[..] {
        [] | ["all"] => available_opts
            .iter()
            .fold(prog, |prog, opt| call_opt(prog, opt)),
        ["none"] => prog,
        _ => opts.iter().fold(prog, |prog, opt| call_opt(prog, opt)),
    }
}

fn call_opt(prog: Program, opt: &str) -> Program {
    match opt {
        "direct_jumps" => direct_jumps::direct_jumps(prog),
        "direct_branches" => direct_branches::direct_branches(prog),
        "native_stack" => native_stack::native_stack(prog),
        "static_data" => static_data::static_data(prog),
        "split_functions" => split_functions::split_functions(prog),
        "trans_static_dyn_funcs" => trans_static_dyn_funcs::trans_static_dyn_funcs(prog),
        "longjmp_except" => longjmp_except::longjmp_except(prog),
        "global2stack" => global2stack::global2stack(prog),
        "direct_jalr" => direct_jalr::direct_jalr(prog),
        "native_mem_func" => native_mem_func::native_mem_func(prog),
        "offset_mem_op" => offset_mem_op::offset_mem_op(prog),
        _ => panic!("Unknown optimization `{opt}`"),
    }
}
