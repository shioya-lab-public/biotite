use crate::llvm_isa as ll;
use crate::riscv_isa as rv;
use std::collections::HashSet;

pub fn run(mut prog: ll::Prog) -> ll::Prog {
    for func in &mut prog.funcs {
        let addrs: HashSet<_> = func
            .inst_blocks
            .iter()
            .map(|b| b.rv_inst.address())
            .collect();
        for block in &mut func.inst_blocks {
            if let rv::Inst::Beq { addr, .. }
            | rv::Inst::Bne { addr, .. }
            | rv::Inst::Blt { addr, .. }
            | rv::Inst::Bge { addr, .. }
            | rv::Inst::Bltu { addr, .. }
            | rv::Inst::Bgeu { addr, .. }
            | rv::Inst::Beqz { addr, .. }
            | rv::Inst::Bnez { addr, .. }
            | rv::Inst::Blez { addr, .. }
            | rv::Inst::Bgez { addr, .. }
            | rv::Inst::Bltz { addr, .. }
            | rv::Inst::Bgtz { addr, .. } = block.rv_inst
            {
                if addrs.contains(&addr) {
                    block.insts.pop();
                    let Some(ll::Inst::Select{ cond, op1, op2, ..}) = block.insts.pop() else {unreachable!()};
                    block.insts.push(ll::Inst::ConBr {
                        cond,
                        iftrue: op1,
                        iffalse: op2,
                    });
                }
            }
        }
    }
    prog
}
