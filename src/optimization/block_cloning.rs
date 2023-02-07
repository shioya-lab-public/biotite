use crate::llvm_isa as ll;

pub fn run(mut prog: ll::Program) -> ll::Program {
    let mut appended = Vec::new();
    let mut current = 0;
    for (i, func) in prog.funcs.iter().enumerate() {
        appended.push(Vec::new());
        if prog.func_syms[&(func.symbol.clone(), func.address)] {
            current = i;
        } else {
            appended[current].push(i);
        }
    }
    for (i, appended) in appended.into_iter().enumerate() {
        let appended: Vec<_> = appended
            .into_iter()
            .map(|i| {
                (
                    prog.funcs[i].inst_blocks.clone(),
                    prog.funcs[i].stack_vars.clone(),
                )
            })
            .collect();
        for (inst_blocks, stack_vars) in appended {
            prog.funcs[i].inst_blocks.extend(inst_blocks);
            prog.funcs[i].stack_vars.extend(stack_vars);
        }
    }
    prog
}
