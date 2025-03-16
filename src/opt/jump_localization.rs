//! An optimization pass that performs the jump localization transformation.

use crate::llvm_isa::{Inst, Prog, Type, Value};
use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run(mut prog: Prog) -> Prog {
    // Construct an opqaue version for each transparent function.
    let fallback_funcs = prog
        .funcs
        .par_iter_mut()
        .filter_map(|func| {
            if !func.inst_blocks.iter().any(|block| {
                matches!(
                    block.rv_inst,
                    rv::Inst::Jalr { .. }
                        | rv::Inst::Jr { .. }
                        | rv::Inst::PseudoJalr { .. }
                        | rv::Inst::OffsetJalr { .. }
                        | rv::Inst::OffsetJr { .. }
                )
            }) {
                func.is_opaque = false;
                let mut fallback_func = func.clone();
                fallback_func.is_opaque = true;
                fallback_func.is_fallback = true;
                Some(fallback_func)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    prog.funcs.extend(fallback_funcs);

    // Perform the jump localization transformation.
    // Notice the return address check is directly implemented in the definition of the `call` instruction.
    let funcs = prog
        .funcs
        .iter()
        .map(|func| {
            let Value::Addr(addr) = func.address else {
                unreachable!();
            };
            addr
        })
        .collect::<HashSet<_>>();
    prog.funcs.par_iter_mut().for_each(|func| {
        let insts = func
            .inst_blocks
            .iter()
            .map(|block| block.rv_inst.address())
            .collect::<HashSet<_>>();
        for block in &mut func.inst_blocks {
            match block.rv_inst {
                rv::Inst::J {
                    address,
                    is_compressed,
                    addr,
                    ..
                } if !insts.contains(&addr) && funcs.contains(&addr) => {
                    block.insts = vec![Inst::Call {
                        rslt: Value::Temp(address, 0),
                        target: Value::Addr(addr),
                        next_pc: next_pc!(next_pc, address, is_compressed),
                        used_regs: Vec::new(),
                        used_fregs: Vec::new(),
                    }];
                }
                rv::Inst::Jal {
                    address,
                    is_compressed,
                    addr,
                    ..
                }
                | rv::Inst::PseudoJal {
                    address,
                    is_compressed,
                    addr,
                    ..
                } if matches!(block.insts[1], Inst::Ret { .. }) => {
                    // We need the test because some `jal` instructions may
                    // have been optimized in the `native_mem_utils` pass.
                    block.insts[1] = Inst::Call {
                        rslt: Value::Temp(address, 0),
                        target: Value::Addr(addr),
                        next_pc: next_pc!(next_pc, address, is_compressed),
                        used_regs: Vec::new(),
                        used_fregs: Vec::new(),
                    };
                }
                rv::Inst::Jalr { address, .. } | rv::Inst::OffsetJalr { address, .. } => {
                    block.insts[3] = Inst::Dispfunc {
                        addr: Value::Addr(address),
                        target: Value::Temp(address, 1),
                        used_regs: Vec::new(),
                        used_fregs: Vec::new(),
                    };
                }
                rv::Inst::PseudoJalr { address, .. } => {
                    block.insts[2] = Inst::Dispfunc {
                        addr: Value::Addr(address),
                        target: Value::Temp(address, 0),
                        used_regs: Vec::new(),
                        used_fregs: Vec::new(),
                    };
                }
                rv::Inst::Jr { address, .. } => {
                    block.insts.splice(
                        1..2,
                        vec![
                            Inst::Store {
                                ty: Type::I64,
                                val: Value::Temp(address, 0),
                                ptr: Value::EntryPtr,
                            },
                            Inst::Br { addr: Value::Disp },
                        ],
                    );
                }
                rv::Inst::OffsetJr { address, .. } => {
                    block.insts.splice(
                        2..3,
                        vec![
                            Inst::Store {
                                ty: Type::I64,
                                val: Value::Temp(address, 1),
                                ptr: Value::EntryPtr,
                            },
                            Inst::Br { addr: Value::Disp },
                        ],
                    );
                }
                rv::Inst::Ret { address, .. } if func.is_opaque => {
                    block.insts = vec![Inst::Checkret {
                        addr: Value::Addr(address),
                        stk: false,
                    }];
                }
                _ => continue,
            }
        }
    });

    prog
}
