use crate::llvm_isa::{Inst, Prog, Type, Value};
use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run(mut prog: Prog) -> Prog {
    let funcs: HashSet<_> = prog
        .funcs
        .iter()
        .map(|func| {
            let Value::Addr(addr) = func.address else {unreachable!();};
            addr
        })
        .collect();
    prog.funcs.par_iter_mut().for_each(|func| {
        func.is_opaque = func.inst_blocks.iter().any(|block| {
            matches!(
                block.rv_inst,
                rv::Inst::Jalr { .. }
                    | rv::Inst::Jr { .. }
                    | rv::Inst::PseudoJalr { .. }
                    | rv::Inst::OffsetJalr { .. }
                    | rv::Inst::OffsetJr { .. }
            )
        });

        let addrs: HashSet<_> = func
            .inst_blocks
            .iter()
            .map(|block| block.rv_inst.address())
            .collect();
        for block in &mut func.inst_blocks {
            match block.rv_inst {
                rv::Inst::J { address, addr, .. } => {
                    if !addrs.contains(&addr) {
                        if funcs.contains(&addr) {
                            block.insts = vec![
                                Inst::Call {
                                    rslt: Value::Temp(address, 0),
                                    target: Value::Addr(addr),
                                    regs: Vec::new(),
                                    fregs: Vec::new(),
                                },
                                Inst::Load {
                                    rslt: Value::Temp(address, 1),
                                    ty: Type::I64,
                                    ptr: Value::Reg(rv::Reg::Ra),
                                },
                                Inst::Ret {
                                    val: Value::Temp(address, 1),
                                },
                            ];
                        } else {
                            block.insts = vec![Inst::Unreachable];
                        }
                    }
                }
                rv::Inst::Beq { addr, .. }
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
                | rv::Inst::Bgtz { addr, .. }
                    if !addrs.contains(&addr) =>
                {
                    block.insts = vec![Inst::Unreachable];
                }
                rv::Inst::Jal { address, addr, .. } | rv::Inst::PseudoJal { address, addr, .. }
                    if matches!(block.insts[1], Inst::Ret { .. }) =>
                {
                    block.insts.splice(
                        1..2,
                        vec![
                            Inst::Call {
                                rslt: Value::Temp(address, 0),
                                target: Value::Addr(addr),
                                regs: Vec::new(),
                                fregs: Vec::new(),
                            },
                            Inst::Contret {
                                addr: Value::Addr(address),
                                next_pc: next_pc!(
                                    next_pc,
                                    block.rv_inst.address(),
                                    block.rv_inst.is_compressed()
                                ),
                                stk: false,
                            },
                        ],
                    );
                }
                _ => continue,
            }
        }

        if func.is_opaque {
            for block in &mut func.inst_blocks {
                match block.rv_inst {
                    rv::Inst::Jalr { address, .. } | rv::Inst::OffsetJalr { address, .. } => {
                        block.insts[3] = Inst::Dispfunc {
                            addr: Value::Addr(address),
                            target: Value::Temp(address, 1),
                            regs: Vec::new(),
                            fregs: Vec::new(),
                        }
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
                                Inst::Br {
                                    addr: Value::Dispatcher,
                                },
                            ],
                        );
                    }
                    rv::Inst::PseudoJalr { address, .. } => {
                        block.insts[2] = Inst::Dispfunc {
                            addr: Value::Addr(address),
                            target: Value::Temp(address, 0),
                            regs: Vec::new(),
                            fregs: Vec::new(),
                        }
                    }
                    rv::Inst::Ret { address, .. } => {
                        block.insts = vec![Inst::Checkret {
                            addr: Value::Addr(address),
                            stk: false,
                        }]
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
                                Inst::Br {
                                    addr: Value::Dispatcher,
                                },
                            ],
                        );
                    }
                    _ => continue,
                }
            }
        }
    });

    catch_non_local_jumps(prog)
}

fn catch_non_local_jumps(mut prog: Prog) -> Prog {
    prog.funcs.par_iter_mut().for_each(|func| {
        if func.inst_blocks.iter().any(|block| {
            matches!(
                block.rv_inst.symbol(),
                Some("<_setjmp>")
                    | Some("<setjmp>")
                    | Some("<_Unwind_Resume>")
                    | Some("<__cxa_begin_catch>")
                    | Some("<__cxa_end_catch>")
            )
        }) {
            func.is_opaque = true;
            for block in &mut func.inst_blocks {
                if let rv::Inst::Jal { .. } | rv::Inst::PseudoJal { .. } = block.rv_inst {
                    if let Some(Inst::Contret { addr, next_pc, stk }) = block.insts.get(2).cloned()
                    {
                        block.insts[2] = Inst::Dispret { addr, next_pc, stk };
                    }
                }
            }
        }
    });
    prog
}
