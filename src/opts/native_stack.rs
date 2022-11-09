use crate::llvm_isa as ll;
use crate::riscv_isa as rv;

pub fn native_stack(mut prog: ll::Program) -> ll::Program {
    let mut t = 67;
    'outer: for func in &mut prog.funcs {
        let mut allocs = Vec::new();
        let mut frees = Vec::new();
        let mut vars = Vec::new();
        for block in &func.inst_blocks {
            match block.rv_inst {
                rv::Inst::Addi {
                    rd: rv::Reg::Sp,
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    ..
                } => {
                    if imm < 0 {
                        allocs.push(imm);
                    } else {
                        frees.push(imm);
                    }
                }
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
                } => vars.push((imm, 1)),
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
                } => vars.push((imm, 2)),
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
                } => vars.push((imm, 4)),
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
                } => vars.push((imm, 8)),
                inst => if contains_sp(inst) {continue 'outer},
            }
        }

        if allocs.len() != 1 || frees.len() != 1 || allocs[0] + frees[0] != 0 {
            continue;
        }
        vars.sort_unstable_by_key(|(i, _)| *i);
        vars.dedup();
        if let Some((0, _)) = vars.first() {
            continue;
        }
        if let (_, true) = vars.iter().fold((0, false), |(sp, overlapped), (i, l)| {
            if overlapped || sp > *i {
                (i + l, true)
            } else {
                (i + l, false)
            }
        }) {
            continue;
        }

        if t == 1 {
            println!("{}", func.symbol);
        } else if t == 0 {
            continue;
        }
        t -= 1;

        let max_offset = frees[0];
        func.stack_vars = vars
            .into_iter()
            .take_while(|(i, _)| *i < max_offset)
            .map(|(offset, bytes)| ll::Value::Stack(func.address, offset as usize, (bytes * 8) as usize))
            .collect();
        for block in &mut func.inst_blocks {
            match block.rv_inst {
                rv::Inst::Lb {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } if imm < max_offset => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I8,
                            ptr: ll::Value::Stack(func.address, imm as usize, 8),
                        },
                        ll::Inst::Sext {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Lbu {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } if imm < max_offset  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I8,
                            ptr: ll::Value::Stack(func.address, imm as usize, 8),
                        },
                        ll::Inst::Zext {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Sb {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                } if imm < max_offset  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Reg(rs2),
                        },
                        ll::Inst::Trunc {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I64,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I8,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I8,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Stack(func.address, imm as usize, 8),
                        },
                    ]
                }
                rv::Inst::Lh {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                }  if imm < max_offset => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I16,
                            ptr: ll::Value::Stack(func.address, imm as usize, 16),
                        },
                        ll::Inst::Sext {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I16,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Lhu {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } if imm < max_offset  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I16,
                            ptr: ll::Value::Stack(func.address, imm as usize, 16),
                        },
                        ll::Inst::Zext {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I16,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Sh {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                } if imm < max_offset  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Reg(rs2),
                        },
                        ll::Inst::Trunc {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I64,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I16,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I16,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Stack(func.address, imm as usize, 16),
                        },
                    ]
                }
                rv::Inst::Lw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                }  if imm < max_offset => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I32,
                            ptr: ll::Value::Stack(func.address, imm as usize, 32),
                        },
                        ll::Inst::Sext {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I32,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Lwu {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } if imm < max_offset  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I32,
                            ptr: ll::Value::Stack(func.address, imm as usize, 32),
                        },
                        ll::Inst::Zext {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I32,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Sw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                }  if imm < max_offset => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Reg(rs2),
                        },
                        ll::Inst::Trunc {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I64,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I32,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I32,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Stack(func.address, imm as usize, 32),
                        },
                    ]
                }
                rv::Inst::Flw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    frd,
                    ..
                }  if imm < max_offset => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I32,
                            ptr: ll::Value::Stack(func.address, imm as usize, 32),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I32,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::Float,
                        },
                        ll::Inst::Fpext {
                            rslt: ll::Value::Temp(address, 2),
                            ty1: ll::Type::Float,
                            val: ll::Value::Temp(address, 1),
                            ty2: ll::Type::Double,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::Double,
                            val: ll::Value::Temp(address, 2),
                            ptr: ll::Value::FReg(frd),
                        },
                    ]
                }
                rv::Inst::Fsw {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    frs2,
                    ..
                } if imm < max_offset  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::Double,
                            ptr: ll::Value::FReg(frs2),
                        },
                        ll::Inst::Fptrunc {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::Double,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::Float,
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 2),
                            ty1: ll::Type::Float,
                            val: ll::Value::Temp(address, 1),
                            ty2: ll::Type::I32,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I32,
                            val: ll::Value::Temp(address, 2),
                            ptr: ll::Value::Stack(func.address, imm as usize, 32),
                        },
                    ]
                }
                rv::Inst::Ld {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } if imm < max_offset && rd != rv::Reg::Ra => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Stack(func.address, imm as usize, 64),
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 0),
                            ptr: ll::Value::Reg(rd),
                        },
                    ]
                }
                rv::Inst::Sd {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                }  if imm < max_offset && rs2 != rv::Reg::Ra  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Reg(rs2),
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 0),
                            ptr: ll::Value::Stack(func.address, imm as usize, 64),
                        },
                    ]
                }
                rv::Inst::Fld {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    frd,
                    ..
                }  if imm < max_offset => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Stack(func.address, imm as usize, 64),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I64,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::Double,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::Double,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::FReg(frd),
                        },
                    ]
                }
                rv::Inst::Fsd {
                    rs1: rv::Reg::Sp,
                    imm: rv::Imm(imm),
                    address,
                    frs2,
                    ..
                } if imm < max_offset  => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::Double,
                            ptr: ll::Value::FReg(frs2),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::Double,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Stack(func.address, imm as usize, 64),
                        },
                    ]
                }
                _ => (),
            }
        }
    }

    prog
}

fn contains_sp(inst: rv::Inst) -> bool {
    format!("{inst:?}").contains("Sp")
}
