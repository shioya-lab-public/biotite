use crate::llvm_isa::{Inst, Prog, Value};
use crate::riscv_isa as rv;

pub fn run(mut prog: Prog) -> Prog {
    let Some(gp) = compute_gp(&prog) else {return prog;};
    for func in &mut prog.funcs {
        for block in &mut func.inst_blocks {
            match block.rv_inst {
                rv::Inst::Lb {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Lh {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Lw {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Lbu {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Lhu {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Sb {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Sh {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Sw {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Lwu {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Ld {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Sd {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Flw {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Fsw {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Fld {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                }
                | rv::Inst::Fsd {
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Gp,
                    ..
                } => {
                    let Inst::Getmemptr { rslt, .. } = block.insts[2] else { unreachable!() };
                    let inst = Inst::Getmemptr {
                        rslt,
                        addr: Value::Imm(rv::Imm(gp + imm)),
                    };
                    block.insts.splice(0..3, vec![inst]);
                }
                _ => continue,
            }
        }
    }
    prog
}

fn compute_gp(prog: &Prog) -> Option<i64> {
    for func in &prog.funcs {
        for i in 0..(func.inst_blocks.len() - 1) {
            if let (
                rv::Inst::Auipc {
                    rd: rv::Reg::Gp,
                    imm: rv::Imm(upper),
                    address: rv::Addr(addr),
                    ..
                },
                rv::Inst::Addi {
                    rd: rv::Reg::Gp,
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(lower),
                    ..
                },
            ) = (
                &func.inst_blocks[i].rv_inst,
                &func.inst_blocks[i + 1].rv_inst,
            ) {
                return Some((upper << 12) + *addr as i64 + lower);
            }
        }
    }
    None
}
