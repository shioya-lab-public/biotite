use crate::llvm_isa::{Inst, Prog, Value};
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
            if let rv::Inst::J { addr, .. } = block.rv_inst {
                if addrs.contains(&addr) {
                    block.insts = vec![Inst::Br {
                        addr: Value::Addr(addr),
                    }];
                }
            }
        }
    });
    prog
}
