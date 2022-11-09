use crate::llvm_isa as ll;
use crate::riscv_isa as rv;

pub fn static_data(mut prog: ll::Program) -> ll::Program {
    let Some(gp) = find_gp(&prog) else {return prog;};
    let len = prog.memory.len();
    for func in &mut prog.funcs {
        for block in &mut func.inst_blocks {
            match block.rv_inst {
                rv::Inst::Lb {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 1),
                            ty: ll::Type::I8,
                            ptr: ll::Value::Temp(address, 0),
                        },
                        ll::Inst::Sext {
                            rslt: ll::Value::Temp(address, 2),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 1),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 2),
                            ptr: ll::Value::Reg(rd),
                        },
                    ];
                }
                rv::Inst::Lbu {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 1),
                            ty: ll::Type::I8,
                            ptr: ll::Value::Temp(address, 0),
                        },
                        ll::Inst::Zext {
                            rslt: ll::Value::Temp(address, 2),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 1),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 2),
                            ptr: ll::Value::Reg(rd),
                        },
                    ];
                }
                rv::Inst::Sb {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                } => {
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
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 2),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I8,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Temp(address, 2),
                        },
                    ];
                }
                rv::Inst::Lh {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I16,
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 2),
                            ty: ll::Type::I16,
                            ptr: ll::Value::Temp(address, 1),
                        },
                        ll::Inst::Sext {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I16,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 3),
                            ptr: ll::Value::Reg(rd),
                        },
                    ];
                }
                rv::Inst::Lhu {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I16,
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 2),
                            ty: ll::Type::I16,
                            ptr: ll::Value::Temp(address, 1),
                        },
                        ll::Inst::Zext {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I16,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 3),
                            ptr: ll::Value::Reg(rd),
                        },
                    ];
                }
                rv::Inst::Sh {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                } => {
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
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 2),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::I16,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I16,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Temp(address, 3),
                        },
                    ];
                }
                rv::Inst::Lw {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I32,
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 2),
                            ty: ll::Type::I32,
                            ptr: ll::Value::Temp(address, 1),
                        },
                        ll::Inst::Sext {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I32,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 3),
                            ptr: ll::Value::Reg(rd),
                        },
                    ];
                }
                rv::Inst::Lwu {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I32,
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 2),
                            ty: ll::Type::I32,
                            ptr: ll::Value::Temp(address, 1),
                        },
                        ll::Inst::Zext {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I32,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 3),
                            ptr: ll::Value::Reg(rd),
                        },
                    ];
                }
                rv::Inst::Sw {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                } => {
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
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 2),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::I32,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I32,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Temp(address, 3),
                        },
                    ];
                }
                rv::Inst::Flw {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    frd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I32,
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 2),
                            ty: ll::Type::I32,
                            ptr: ll::Value::Temp(address, 1),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I32,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::Float,
                        },
                        ll::Inst::Fpext {
                            rslt: ll::Value::Temp(address, 4),
                            ty1: ll::Type::Float,
                            val: ll::Value::Temp(address, 3),
                            ty2: ll::Type::Double,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::Double,
                            val: ll::Value::Temp(address, 4),
                            ptr: ll::Value::FReg(frd),
                        },
                    ];
                }
                rv::Inst::Fsw {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    frs2,
                    ..
                } => {
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
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 3),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 4),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 3),
                            ty2: ll::Type::I32,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I32,
                            val: ll::Value::Temp(address, 2),
                            ptr: ll::Value::Temp(address, 4),
                        },
                    ];
                }
                rv::Inst::Ld {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 2),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Temp(address, 1),
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 2),
                            ptr: ll::Value::Reg(rd),
                        },
                    ];
                }
                rv::Inst::Sd {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    rs2,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 0),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Reg(rs2),
                        },
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 1),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 2),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 1),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 0),
                            ptr: ll::Value::Temp(address, 2),
                        },
                    ];
                }
                rv::Inst::Fld {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    frd,
                    ..
                } => {
                    block.insts = vec![
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 0),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 1),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 0),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Load {
                            rslt: ll::Value::Temp(address, 2),
                            ty: ll::Type::I64,
                            ptr: ll::Value::Temp(address, 1),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I64,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::Double,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::Double,
                            val: ll::Value::Temp(address, 3),
                            ptr: ll::Value::FReg(frd),
                        },
                    ];
                }
                rv::Inst::Fsd {
                    rs1: rv::Reg::Gp,
                    imm: rv::Imm(imm),
                    address,
                    frs2,
                    ..
                } => {
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
                        ll::Inst::Getdataptr {
                            rslt: ll::Value::Temp(address, 2),
                            len,
                            addr: ll::Value::Imm(rv::Imm(gp + imm)),
                        },
                        ll::Inst::Bitcast {
                            rslt: ll::Value::Temp(address, 3),
                            ty1: ll::Type::I8,
                            val: ll::Value::Temp(address, 2),
                            ty2: ll::Type::I64,
                        },
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: ll::Value::Temp(address, 1),
                            ptr: ll::Value::Temp(address, 3),
                        },
                    ];
                }
                _ => {}
            }
        }
    }

    prog
}

fn find_gp(prog: &ll::Program) -> Option<i64> {
    for func in &prog.funcs {
        let mut i = 0;
        while i < func.inst_blocks.len() - 1 {
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
            ) = (func.inst_blocks[i].rv_inst, func.inst_blocks[i + 1].rv_inst)
            {
                return Some((upper << 12) + addr as i64 + lower);
            }
            i += 1;
        }
    }
    None
}
