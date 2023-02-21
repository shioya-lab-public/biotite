use crate::llvm_isa::{Inst, Prog};
use crate::riscv_isa as rv;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run(mut prog: Prog) -> Prog {
    prog.funcs.par_iter_mut().for_each(|func| {
        let addrs: HashSet<_> = func
            .inst_blocks
            .iter()
            .map(|block| block.rv_inst.address())
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
                    let Inst::Select{ cond, op1, op2, ..} = block.insts.pop().unwrap() else { unreachable!(); };
                    block.insts.push(Inst::Conbr {
                        cond,
                        iftrue: op1,
                        iffalse: op2,
                    });
                }
            }
        }
    });
    prog
}
