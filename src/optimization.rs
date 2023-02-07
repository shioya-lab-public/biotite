mod block_cloning;
mod cached_mem_access;
mod immut_gp;
mod jump_localization;
mod native_branches;
mod native_direct_jumps;
mod native_mem_utils;
mod native_stack_vars;
mod stack_regs;

use crate::llvm_isa::Program;

macro_rules! opts {
    ( $($opt:ident,)* ) => {
        pub fn optimize(
            mut prog: Program,
            enable_all_opts: bool,
            disable_all_opts: bool,
            enable_opts: Vec<String>,
            disable_opts: Vec<String>,
        ) -> Program {
            let opts = if enable_all_opts {
                vec![$( stringify!($opt), )*]
            } else if disable_all_opts {
                Vec::new()
            } else if !enable_opts.is_empty() {
                enable_opts.iter().map(|opt| opt.as_str()).collect()
            } else {
                vec![$( stringify!($opt), )*].iter().filter(|opt| !disable_opts.contains(&opt.to_string())).copied().collect()
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
    native_direct_jumps,
    native_branches,
    jump_localization,
    native_stack_vars,
    stack_regs,
    cached_mem_access,
}
