use crate::llvm_isa::Prog;

pub fn run(mut prog: Prog) -> Prog {
    let mut appended = Vec::new();
    let mut cur = 0;
    for (i, func) in prog.funcs.iter().enumerate() {
        appended.push(Vec::new());
        if prog
            .func_syms
            .contains(&(func.symbol.clone(), func.address))
        {
            cur = i;
        } else {
            appended[cur].push(i);
        }
    }

    for (i, appended) in appended.into_iter().enumerate() {
        let appended: Vec<_> = appended
            .into_iter()
            .map(|i| prog.funcs[i].inst_blocks.clone())
            .collect();
        for inst_blocks in appended {
            prog.funcs[i].inst_blocks.extend(inst_blocks);
        }
    }

    prog
}
