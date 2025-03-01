use crate::llvm_isa::Prog;

pub fn run(mut prog: Prog) -> Prog {
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
    for (i, appended) in appendeds.into_iter().enumerate() {
        let appended = appended
            .into_iter()
            .flat_map(|i| prog.funcs[i].inst_blocks.clone())
            .collect::<Vec<_>>();
        prog.funcs[i].inst_blocks.extend(appended);
    }
    prog
}
