use crate::llvm_isa as ll;
use crate::riscv_isa as rv;
use std::collections::HashSet;

pub fn offset_mem_op(mut prog: ll::Program) -> ll::Program {
    for func in &mut prog.funcs {
        if func.dynamic {
            continue;
        }
        let targets = find_jump_targets(func);
        let mut stat = None;
        for block in &mut func.inst_blocks {
            if targets.contains(&block.rv_inst.address()) {
                stat = None;
            }
            match &block.rv_inst {
                rv::Inst::Lb {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lw {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lbu {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lhu {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lwu {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Ld {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => {
                    if rs1 == &rv::Reg::Gp {
                        stat = None;
                    } else if rs1 == &rv::Reg::Sp && !func.stack_vars.is_empty() {
                        stat = None;
                    } else {
                        let ll::Inst::Getmemptr { rslt, .. } = block.insts[2] else { unreachable!()};
                        if let Some((reg, offset, ptr)) = stat {
                            if reg == rs1 {
                                let inst = ll::Inst::Getelementptr {
                                    rslt,
                                    ptr,
                                    offset: ll::Value::Imm(rv::Imm(imm - offset)),
                                };
                                block.insts.splice(0..3, vec![inst]);
                            }
                        }
                        if rd == rs1 {
                            stat = None;
                        } else {
                            stat = Some((rs1, imm, rslt));
                        }
                    }
                }
                rv::Inst::Sb {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Sh {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Sw {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => {
                    if rs1 == &rv::Reg::Gp {
                        stat = None;
                    } else if rs1 == &rv::Reg::Sp && !func.stack_vars.is_empty() {
                        stat = None;
                    } else {
                        let ll::Inst::Getmemptr { rslt, .. } = block.insts[4] else {unreachable!()};
                        if let Some((reg, offset, ptr)) = stat {
                            if reg == rs1 {
                                let inst = ll::Inst::Getelementptr {
                                    rslt,
                                    ptr,
                                    offset: ll::Value::Imm(rv::Imm(imm - offset)),
                                };
                                block.insts.splice(2..5, vec![inst]);
                            }
                        }
                        stat = Some((rs1, imm, rslt));
                    }
                }
                rv::Inst::Sd {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => {
                    if rs1 == &rv::Reg::Gp {
                        stat = None;
                    } else if rs1 == &rv::Reg::Sp && !func.stack_vars.is_empty() {
                        stat = None;
                    } else {
                        let ll::Inst::Getmemptr { rslt, .. } = block.insts[3] else {unreachable!()};
                        if let Some((reg, offset, ptr)) = stat {
                            if reg == rs1 {
                                let inst = ll::Inst::Getelementptr {
                                    rslt,
                                    ptr,
                                    offset: ll::Value::Imm(rv::Imm(imm - offset)),
                                };
                                block.insts.splice(1..4, vec![inst]);
                            }
                        }
                        stat = Some((rs1, imm, rslt));
                    }
                }
                rv::Inst::Flw {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Fld {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => {
                    if rs1 == &rv::Reg::Gp {
                        stat = None;
                    } else if rs1 == &rv::Reg::Sp && !func.stack_vars.is_empty() {
                        stat = None;
                    } else {
                        let ll::Inst::Getmemptr { rslt, .. } = block.insts[2] else { unreachable!()};
                        if let Some((reg, offset, ptr)) = stat {
                            if reg == rs1 {
                                let inst = ll::Inst::Getelementptr {
                                    rslt,
                                    ptr,
                                    offset: ll::Value::Imm(rv::Imm(imm - offset)),
                                };
                                block.insts.splice(0..3, vec![inst]);
                            }
                        }
                        stat = Some((rs1, imm, rslt));
                    }
                }
                rv::Inst::Fsw {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => {
                    if rs1 == &rv::Reg::Gp {
                        stat = None;
                    } else if rs1 == &rv::Reg::Sp && !func.stack_vars.is_empty() {
                        stat = None;
                    } else {
                        let ll::Inst::Getmemptr { rslt, .. } = block.insts[5] else {unreachable!()};
                        if let Some((reg, offset, ptr)) = stat {
                            if reg == rs1 {
                                let inst = ll::Inst::Getelementptr {
                                    rslt,
                                    ptr,
                                    offset: ll::Value::Imm(rv::Imm(imm - offset)),
                                };
                                block.insts.splice(3..6, vec![inst]);
                            }
                        }
                        stat = Some((rs1, imm, rslt));
                    }
                }
                rv::Inst::Fsd {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => {
                    if rs1 == &rv::Reg::Gp {
                        stat = None;
                    } else if rs1 == &rv::Reg::Sp && !func.stack_vars.is_empty() {
                        stat = None;
                    } else {
                        let ll::Inst::Getmemptr { rslt, .. } = block.insts[4] else {unreachable!()};
                        if let Some((reg, offset, ptr)) = stat {
                            if reg == rs1 {
                                let inst = ll::Inst::Getelementptr {
                                    rslt,
                                    ptr,
                                    offset: ll::Value::Imm(rv::Imm(imm - offset)),
                                };
                                block.insts.splice(2..5, vec![inst]);
                            }
                        }
                        stat = Some((rs1, imm, rslt));
                    }
                }
                _ => stat = None,
            }
        }
    }
    prog
}

fn find_jump_targets(func: &ll::Func) -> HashSet<rv::Addr> {
    let mut blocks = HashSet::new();
    for block in &func.inst_blocks {
        match &block.rv_inst {
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
            | rv::Inst::J { addr, .. } => blocks.insert(*addr),
            _ => continue,
        };
    }
    blocks
}
