mod direct_branches;
mod direct_jumps;

use crate::llvm_isa::Program;

pub fn optimize(prog: Program, opts: &Vec<String>) -> Program {
    let opts: Vec<_> = opts.iter().map(|opt| opt.as_str()).collect();
    let available_opts = vec!["direct_jumps", "direct_branches"];
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
        _ => panic!("Unknown optimization `{opt}`"),
    }
}
