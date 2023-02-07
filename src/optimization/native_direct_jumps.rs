use crate::llvm_isa as ll;
use crate::riscv_isa as rv;
use std::collections::HashSet;

pub fn run(mut prog: ll::Program) -> ll::Program {
    for func in &mut prog.funcs {
        let addrs: HashSet<_> = func
            .inst_blocks
            .iter()
            .map(|b| b.rv_inst.address())
            .collect();
        for block in &mut func.inst_blocks {
            if let rv::Inst::J { addr, .. } = block.rv_inst {
                if addrs.contains(&addr) {
                    let addr = ll::Value::Addr(addr);
                    block.insts = vec![ll::Inst::Br { addr }];
                }
            }
        }
    }
    prog
}
