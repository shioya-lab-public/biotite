use crate::llvm_isa as ll;
use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;
use std::collections::HashSet;

fn get_next_pc(inst: &rv::Inst) -> ll::Value {
    use crate::llvm_isa::*;
    use crate::riscv_isa as RV;
    next_pc!(next_pc, inst.address(), inst.is_compressed())
}

pub fn direct_jalr(mut prog: ll::Program) -> ll::Program {
    let funcs: HashSet<_> = prog.funcs.iter().map(|f| (f.address)).collect();
    for func in &mut prog.funcs {
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
                    let ll_insts = vec![ll::Inst::Unreachable {}];
                    jals.push((i, rv_inst, ll_insts));
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
                ) if rd == rs1 && rd != &rv::Reg::Ra => {
                    let target =
                        rv::Addr((((upper << 12) as u64 + addr) as i128 + *lower as i128) as u64);
                    if funcs.contains(&target) {
                        let rv_inst = rv::Inst::PseudoJal {
                            address: rv::Addr(*addr),
                            is_compressed: *is_compressed,
                            addr: target,
                            symbol: None,
                        };
                        let ll_insts = vec![
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: get_next_pc(&rv_inst),
                                ptr: ll::Value::Reg(rv::Reg::Ra),
                            },
                            ll::Inst::Ret {
                                val: ll::Value::Addr(target),
                            },
                        ];
                        jals.push((i, rv_inst, ll_insts));
                    }
                }
                _ => (),
            }
            i += 1;
        }

        jals.reverse();
        for (i, rv_inst, ll_insts) in jals {
            func.inst_blocks[i].rv_inst = rv_inst;
            func.inst_blocks[i].insts = ll_insts;
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
