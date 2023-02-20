use crate::llvm_isa::{Inst, Prog, Type, Value};
use crate::riscv_isa as rv;
use rayon::prelude::*;

pub fn run(mut prog: Prog) -> Prog {
    prog.funcs.par_iter_mut().for_each(|func| {
        let mut allocs = Vec::new();
        let mut frees = Vec::new();
        let mut ra_locs = Vec::new();
        let mut vars = Vec::new();
        for block in &func.inst_blocks {
            match &block.rv_inst {
                rv::Inst::Addi {
                    rd: rv::Reg::Sp,
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                } => {
                    if *imm < 0 {
                        allocs.push(*imm);
                    } else {
                        frees.push(*imm);
                    }
                }
                rv::Inst::Ld {
                    rd: rv::Reg::Ra,
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                } => ra_locs.push(*imm),
                rv::Inst::Lb {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Lbu {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Sb {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                } => vars.push((*imm, 8)),
                rv::Inst::Lh {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Lhu {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Sh {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                } => vars.push((*imm, 16)),
                rv::Inst::Lw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Sw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Lwu {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Flw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Fsw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                } => vars.push((*imm, 32)),
                rv::Inst::Ld {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Sd {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Fld {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                }
                | rv::Inst::Fsd {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                } => vars.push((*imm, 64)),
                inst if format!("{inst:?}").contains("Sp") => return,
                _ => continue,
            }
        }

        if allocs.len() != 1 || frees.len() != 1 || allocs[0] + frees[0] != 0 {
            return;
        }
        let max_offset = frees[0];

        if ra_locs.iter().min() != ra_locs.iter().max() {
            return;
        }
        let ra_loc = ra_locs.get(0).cloned().unwrap_or_default();

        vars.sort_unstable();
        vars.dedup();
        if let Some((0, _)) = vars.first() {
            return;
        }
        if let (_, true) = vars
            .iter()
            .fold((0, false), |(sp, overlapped), (offset, width)| {
                if overlapped || sp > *offset {
                    (offset + width / 8, true)
                } else {
                    (offset + width / 8, false)
                }
            })
        {
            return;
        }
        func.stack_vars = vars
            .into_iter()
            .take_while(|(offset, _)| *offset < max_offset)
            .map(|(offset, width)| Value::Stack(offset as usize, width as usize))
            .collect();

        for block in &mut func.inst_blocks {
            match block.rv_inst {
                rv::Inst::Lb {
                    address,
                    rd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I8,
                            ptr: Value::Stack(imm as usize, 8),
                        },
                        Inst::Sext {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I8,
                            val: Value::Temp(address, 0),
                            ty2: Type::I64,
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 1),
                            ptr: Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Lh {
                    address,
                    rd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I16,
                            ptr: Value::Stack(imm as usize, 16),
                        },
                        Inst::Sext {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I16,
                            val: Value::Temp(address, 0),
                            ty2: Type::I64,
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 1),
                            ptr: Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Lw {
                    address,
                    rd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I32,
                            ptr: Value::Stack(imm as usize, 32),
                        },
                        Inst::Sext {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I32,
                            val: Value::Temp(address, 0),
                            ty2: Type::I64,
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 1),
                            ptr: Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Lbu {
                    address,
                    rd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I8,
                            ptr: Value::Stack(imm as usize, 8),
                        },
                        Inst::Zext {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I8,
                            val: Value::Temp(address, 0),
                            ty2: Type::I64,
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 1),
                            ptr: Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Lhu {
                    address,
                    rd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I16,
                            ptr: Value::Stack(imm as usize, 16),
                        },
                        Inst::Zext {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I16,
                            val: Value::Temp(address, 0),
                            ty2: Type::I64,
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 1),
                            ptr: Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Sb {
                    address,
                    rs2,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I64,
                            ptr: Value::Reg(rs2),
                        },
                        Inst::Trunc {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I64,
                            val: Value::Temp(address, 0),
                            ty2: Type::I8,
                        },
                        Inst::Store {
                            ty: Type::I8,
                            val: Value::Temp(address, 1),
                            ptr: Value::Stack(imm as usize, 8),
                        },
                    ]
                }
                rv::Inst::Sh {
                    address,
                    rs2,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I64,
                            ptr: Value::Reg(rs2),
                        },
                        Inst::Trunc {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I64,
                            val: Value::Temp(address, 0),
                            ty2: Type::I16,
                        },
                        Inst::Store {
                            ty: Type::I16,
                            val: Value::Temp(address, 1),
                            ptr: Value::Stack(imm as usize, 16),
                        },
                    ]
                }
                rv::Inst::Sw {
                    address,
                    rs2,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I64,
                            ptr: Value::Reg(rs2),
                        },
                        Inst::Trunc {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I64,
                            val: Value::Temp(address, 0),
                            ty2: Type::I32,
                        },
                        Inst::Store {
                            ty: Type::I32,
                            val: Value::Temp(address, 1),
                            ptr: Value::Stack(imm as usize, 32),
                        },
                    ]
                }
                rv::Inst::Lwu {
                    address,
                    rd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I32,
                            ptr: Value::Stack(imm as usize, 32),
                        },
                        Inst::Zext {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I32,
                            val: Value::Temp(address, 0),
                            ty2: Type::I64,
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 1),
                            ptr: Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Ld {
                    address,
                    rd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset && imm != ra_loc => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I64,
                            ptr: Value::Stack(imm as usize, 64),
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 0),
                            ptr: Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Sd {
                    address,
                    rs2,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset && imm != ra_loc => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I64,
                            ptr: Value::Reg(rs2),
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 0),
                            ptr: Value::Stack(imm as usize, 64),
                        },
                    ]
                }
                rv::Inst::Flw {
                    address,
                    frd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I32,
                            ptr: Value::Stack(imm as usize, 32),
                        },
                        Inst::Bitcast {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I32,
                            val: Value::Temp(address, 0),
                            ty2: Type::Float,
                        },
                        Inst::Fpext {
                            rslt: Value::Temp(address, 2),
                            ty1: Type::Float,
                            val: Value::Temp(address, 1),
                            ty2: Type::Double,
                        },
                        Inst::Store {
                            ty: Type::Double,
                            val: Value::Temp(address, 2),
                            ptr: Value::FReg(frd),
                        },
                    ]
                }
                rv::Inst::Fsw {
                    address,
                    frs2,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::Double,
                            ptr: Value::FReg(frs2),
                        },
                        Inst::Fptrunc {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::Double,
                            val: Value::Temp(address, 0),
                            ty2: Type::Float,
                        },
                        Inst::Bitcast {
                            rslt: Value::Temp(address, 2),
                            ty1: Type::Float,
                            val: Value::Temp(address, 1),
                            ty2: Type::I32,
                        },
                        Inst::Store {
                            ty: Type::I32,
                            val: Value::Temp(address, 2),
                            ptr: Value::Stack(imm as usize, 32),
                        },
                    ]
                }
                rv::Inst::Fld {
                    address,
                    frd,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::I64,
                            ptr: Value::Stack(imm as usize, 64),
                        },
                        Inst::Bitcast {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::I64,
                            val: Value::Temp(address, 0),
                            ty2: Type::Double,
                        },
                        Inst::Store {
                            ty: Type::Double,
                            val: Value::Temp(address, 1),
                            ptr: Value::FReg(frd),
                        },
                    ]
                }
                rv::Inst::Fsd {
                    address,
                    frs2,
                    imm: rv::Imm(imm),
                    rs1: rv::Reg::Sp,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        Inst::Load {
                            rslt: Value::Temp(address, 0),
                            ty: Type::Double,
                            ptr: Value::FReg(frs2),
                        },
                        Inst::Bitcast {
                            rslt: Value::Temp(address, 1),
                            ty1: Type::Double,
                            val: Value::Temp(address, 0),
                            ty2: Type::I64,
                        },
                        Inst::Store {
                            ty: Type::I64,
                            val: Value::Temp(address, 1),
                            ptr: Value::Stack(imm as usize, 64),
                        },
                    ]
                }
                _ => continue,
            }
        }
    });
    prog
}
