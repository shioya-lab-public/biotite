mod direct_branches;
mod direct_jumps;
mod native_stack;
mod static_data;

use crate::llvm_isa::Program;

pub fn optimize(prog: Program, opts: &Vec<String>) -> Program {
    let opts: Vec<_> = opts.iter().map(|opt| opt.as_str()).collect();
    let available_opts = vec!["direct_jumps", "direct_branches"/*, "native_stack"*/, "static_data"];
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
        _ => panic!("Unknown optimization `{opt}`"),
    }
}
