mod block_cloning;
mod cached_base_ptrs;
mod immut_gp;
mod jump_localization;
mod native_branches;
mod native_direct_jumps;
mod native_mem_utils;
mod stk_regs;

use crate::llvm_isa::Prog;

macro_rules! opts {
    ( $( $opt:ident, )* ) => {
        pub fn run(
            mut prog: Prog,
            enable_all_opts: bool,
            disable_all_opts: bool,
            enable_opts: Vec<String>,
            disable_opts: Vec<String>,
        ) -> Prog {
            let opts = if enable_all_opts {
                vec![$( stringify!($opt) ),*]
            } else if disable_all_opts {
                Vec::new()
            } else if !enable_opts.is_empty() {
                enable_opts.iter().map(|opt| opt.as_str()).collect()
            } else {
                [$( stringify!($opt) ),*]
                    .into_iter()
                    .filter(|opt| !disable_opts.contains(&opt.to_string()))
                    .collect()
            };
            for opt in opts {
                match opt {
                    $(
                        stringify!($opt) => prog = $opt::run(prog),
                    )*
                    _ => panic!("Unknown optimization `{opt}`"),
                }
            }
            prog
        }
    };
}

opts! {
    immut_gp,
    native_mem_utils,
    block_cloning,
    native_branches,
    native_direct_jumps,
    jump_localization,
    cached_base_ptrs,
    stk_regs,
}
