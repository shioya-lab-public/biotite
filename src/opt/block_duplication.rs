//! An optimization pass that duplicates assembly blocks to find possible functions.

use crate::llvm_isa::Prog;

pub fn run(mut prog: Prog) -> Prog {
    // For each block marked as a function, record following non-function blocks until the next function block.
    let mut appendeds = Vec::new();
    let mut cur = 0;
    for (i, func) in prog.funcs.iter().enumerate() {
        appendeds.push(Vec::new());
        if prog.func_syms.contains(&func.address) {
            cur = i;
        } else {
            appendeds[cur].push(i);
        }
    }

    // Duplicate recorded non-function blocks and merge them into the leading function block.
    for (i, appended) in appendeds.into_iter().enumerate() {
        let appended = appended
            .into_iter()
            .flat_map(|i| prog.funcs[i].inst_blocks.clone())
            .collect::<Vec<_>>();
        prog.funcs[i].inst_blocks.extend(appended);
    }

    prog
}
