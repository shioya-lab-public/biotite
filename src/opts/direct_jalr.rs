use crate::llvm_isa as ll;
use crate::riscv_isa as rv;
use std::collections::HashSet;

pub fn direct_jalr(mut prog: ll::Program) -> ll::Program {
    let funcs: HashSet<_> = prog.funcs.iter().map(|f| (f.address)).collect();
    for func in &mut prog.funcs {
        let addrs: HashSet<_> = func
            .inst_blocks
            .iter()
            .map(|b| b.rv_inst.address())
            .collect();
        let mut i = 0;
        let mut jals = Vec::new();
        while i < func.inst_blocks.len() - 1 {
            match (
                &func.inst_blocks[i].rv_inst,
                &func.inst_blocks[i + 1].rv_inst,
            ) {
                (
                    rv::Inst::Auipc {
                        address: rv::Addr(addr),
                        is_compressed,
                        rd: rv::Reg::Ra,
                        imm: rv::Imm(0),
                        ..
                    },
                    rv::Inst::PseudoJalr {
                        rs1: rv::Reg::Zero, ..
                    },
                ) => {
                    let rv_inst = rv::Inst::Ebreak {
                        address: rv::Addr(*addr),
                        is_compressed: *is_compressed,
                        symbol: None,
                    };
                    let ll_inst = ll::Inst::Unreachable {};
                    jals.push((i, rv_inst, ll_inst));
                }
                (
                    rv::Inst::Auipc {
                        address: rv::Addr(addr),
                        is_compressed,
                        rd,
                        imm: rv::Imm(upper),
                        ..
                    },
                    rv::Inst::OffsetJalr {
                        rs1,
                        imm: rv::Imm(lower),
                        ..
                    },
                ) if rd == rs1 => {
                    let target =
                        rv::Addr((((upper << 12) as u64 + addr) as i128 + *lower as i128) as u64);
                    if funcs.contains(&target) {
                        let rv_inst = rv::Inst::PseudoJal {
                            address: rv::Addr(*addr),
                            is_compressed: *is_compressed,
                            addr: target,
                            symbol: None,
                        };
                        let ll_inst = ll::Inst::Call {
                            rslt: ll::Value::Temp(rv::Addr(*addr), 0),
                            func: ll::Value::Addr(target),
                            regs: Vec::new(),
                            fregs: Vec::new(),
                        };
                        jals.push((i, rv_inst, ll_inst));
                    } else if addrs.contains(&target) {
                        let rv_inst = rv::Inst::J {
                            address: rv::Addr(*addr),
                            is_compressed: *is_compressed,
                            addr: target,
                            symbol: None,
                        };
                        let ll_inst = ll::Inst::Br {
                            addr: ll::Value::Addr(target),
                        };
                        jals.push((i, rv_inst, ll_inst));
                    }
                }
                _ => (),
            }
            i += 1;
        }
        jals.reverse();
        for (i, rv_inst, ll_inst) in jals {
            func.inst_blocks[i].rv_inst = rv_inst;
            func.inst_blocks[i].insts = vec![ll_inst];
            let address = func.inst_blocks[i + 1].rv_inst.address();
            let is_compressed = func.inst_blocks[i + 1].rv_inst.is_compressed();
            func.inst_blocks[i + 1].rv_inst = rv::Inst::Unimp {
                address,
                is_compressed,
                symbol: None,
            };
            func.inst_blocks[i + 1].insts = Vec::new();
        }
    }
    prog
}
