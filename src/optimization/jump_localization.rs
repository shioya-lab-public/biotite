use crate::llvm_isa as ll;
use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;
use std::collections::HashSet;

fn get_next_pc(inst: &rv::Inst) -> ll::Value {
    use crate::llvm_isa::*;
    use crate::riscv_isa as RV;
    next_pc!(next_pc, inst.address(), inst.is_compressed())
}

pub fn run(mut prog: ll::Program) -> ll::Program {
    for func in &mut prog.funcs {
        let is_dyn = func.inst_blocks.iter().any(|block| {
            matches!(
                block.rv_inst,
                rv::Inst::Jr { .. }
                    | rv::Inst::OffsetJr { .. }
                    | rv::Inst::Jalr { .. }
                    | rv::Inst::PseudoJalr { .. }
                    | rv::Inst::OffsetJalr { .. }
            )
        });
        func.dynamic = is_dyn;

        let addrs: HashSet<_> = func
            .inst_blocks
            .iter()
            .map(|b| b.rv_inst.address())
            .collect();
        for block in &mut func.inst_blocks {
            match block.rv_inst {
                rv::Inst::J { address, addr, .. } => {
                    if !addrs.contains(&addr) {
                        let rslt = ll::Value::Temp(address, 0);
                        let func = ll::Value::Addr(addr);
                        let val = ll::Value::Addr(rv::Addr(0));
                        block.insts = vec![
                            ll::Inst::Call {
                                rslt,
                                func,
                                regs: Vec::new(),
                                fregs: Vec::new(),
                            },
                            ll::Inst::Ret { val },
                        ];
                    }
                }
                rv::Inst::Jal {
                    address, rd, addr, ..
                } => {
                    block.insts = vec![
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: get_next_pc(&block.rv_inst),
                            ptr: ll::Value::Reg(rd),
                        },
                        ll::Inst::Call {
                            rslt: ll::Value::Temp(address, 0),
                            func: ll::Value::Addr(addr),
                            regs: Vec::new(),
                            fregs: Vec::new(),
                        },
                        ll::Inst::ContRet {
                            addr: ll::Value::Addr(address),
                            next_pc: get_next_pc(&block.rv_inst),
                            stk: false,
                        },
                    ]
                }
                rv::Inst::PseudoJal { address, addr, .. } => {
                    block.insts = vec![
                        ll::Inst::Store {
                            ty: ll::Type::I64,
                            val: get_next_pc(&block.rv_inst),
                            ptr: ll::Value::Reg(rv::Reg::Ra),
                        },
                        ll::Inst::Call {
                            rslt: ll::Value::Temp(address, 0),
                            func: ll::Value::Addr(addr),
                            regs: Vec::new(),
                            fregs: Vec::new(),
                        },
                        ll::Inst::ContRet {
                            addr: ll::Value::Addr(address),
                            next_pc: get_next_pc(&block.rv_inst),
                            stk: false,
                        },
                    ]
                }
                _ => (),
            }
        }

        if is_dyn {
            for block in &mut func.inst_blocks {
                match block.rv_inst {
                    rv::Inst::Jr { address, rs1, .. } => {
                        block.insts = vec![
                            ll::Inst::Load {
                                rslt: ll::Value::Temp(address, 0),
                                ty: ll::Type::I64,
                                ptr: ll::Value::Reg(rs1),
                            },
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: ll::Value::Temp(address, 0),
                                ptr: ll::Value::EntryPtr,
                            },
                            ll::Inst::Br {
                                addr: ll::Value::Addr(rv::Addr(0)),
                            },
                        ]
                    }
                    rv::Inst::OffsetJr {
                        address, imm, rs1, ..
                    } => {
                        block.insts = vec![
                            ll::Inst::Load {
                                rslt: ll::Value::Temp(address, 0),
                                ty: ll::Type::I64,
                                ptr: ll::Value::Reg(rs1),
                            },
                            ll::Inst::Add {
                                rslt: ll::Value::Temp(address, 1),
                                ty: ll::Type::I64,
                                op1: ll::Value::Temp(address, 0),
                                op2: ll::Value::Imm(imm),
                            },
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: ll::Value::Temp(address, 1),
                                ptr: ll::Value::EntryPtr,
                            },
                            ll::Inst::Br {
                                addr: ll::Value::Addr(rv::Addr(0)),
                            },
                        ]
                    }
                    rv::Inst::Jalr {
                        address,
                        rd,
                        imm,
                        rs1,
                        ..
                    } => {
                        block.insts = vec![
                            ll::Inst::Load {
                                rslt: ll::Value::Temp(address, 0),
                                ty: ll::Type::I64,
                                ptr: ll::Value::Reg(rs1),
                            },
                            ll::Inst::Add {
                                rslt: ll::Value::Temp(address, 1),
                                ty: ll::Type::I64,
                                op1: ll::Value::Temp(address, 0),
                                op2: ll::Value::Imm(imm),
                            },
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: get_next_pc(&block.rv_inst),
                                ptr: ll::Value::Reg(rd),
                            },
                            ll::Inst::DispFunc {
                                func: ll::Value::Temp(address, 1),
                                regs: Vec::new(),
                                fregs: Vec::new(),
                                addr: address,
                            },
                        ]
                    }
                    rv::Inst::PseudoJalr { address, rs1, .. } => {
                        block.insts = vec![
                            ll::Inst::Load {
                                rslt: ll::Value::Temp(address, 0),
                                ty: ll::Type::I64,
                                ptr: ll::Value::Reg(rs1),
                            },
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: get_next_pc(&block.rv_inst),
                                ptr: ll::Value::Reg(rv::Reg::Ra),
                            },
                            ll::Inst::DispFunc {
                                func: ll::Value::Temp(address, 0),
                                regs: Vec::new(),
                                fregs: Vec::new(),
                                addr: address,
                            },
                        ]
                    }
                    rv::Inst::OffsetJalr {
                        address, imm, rs1, ..
                    } => {
                        block.insts = vec![
                            ll::Inst::Load {
                                rslt: ll::Value::Temp(address, 0),
                                ty: ll::Type::I64,
                                ptr: ll::Value::Reg(rs1),
                            },
                            ll::Inst::Add {
                                rslt: ll::Value::Temp(address, 1),
                                ty: ll::Type::I64,
                                op1: ll::Value::Temp(address, 0),
                                op2: ll::Value::Imm(imm),
                            },
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: get_next_pc(&block.rv_inst),
                                ptr: ll::Value::Reg(rv::Reg::Ra),
                            },
                            ll::Inst::DispFunc {
                                func: ll::Value::Temp(address, 1),
                                regs: Vec::new(),
                                fregs: Vec::new(),
                                addr: address,
                            },
                        ]
                    }
                    rv::Inst::Ret { address, .. } => {
                        block.insts = vec![ll::Inst::CheckRet {
                            addr: ll::Value::Addr(address),
                            stk: false,
                        }]
                    }
                    _ => (),
                }
            }
        } else {
            for block in &mut func.inst_blocks {
                if let rv::Inst::Ret { .. } = block.rv_inst {
                    let val = ll::Value::Addr(rv::Addr(0));
                    block.insts = vec![ll::Inst::Ret { val }];
                }
            }
        }
    }
    non_local_jumps(prog)
}

pub fn non_local_jumps(mut prog: ll::Program) -> ll::Program {
    for func in &mut prog.funcs {
        if is_longjmp_except_func(func) {
            func.dynamic = true;
            for block in &mut func.inst_blocks {
                match block.rv_inst {
                    rv::Inst::Jal {
                        address, rd, addr, ..
                    } => {
                        block.insts = vec![
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: get_next_pc(&block.rv_inst),
                                ptr: ll::Value::Reg(rd),
                            },
                            ll::Inst::Call {
                                rslt: ll::Value::Temp(address, 0),
                                func: ll::Value::Addr(addr),
                                regs: Vec::new(),
                                fregs: Vec::new(),
                            },
                            ll::Inst::DispRet {
                                addr: ll::Value::Addr(address),
                                next_pc: get_next_pc(&block.rv_inst),
                                stk: false,
                            },
                        ]
                    }
                    rv::Inst::PseudoJal { address, addr, .. } => {
                        block.insts = vec![
                            ll::Inst::Store {
                                ty: ll::Type::I64,
                                val: get_next_pc(&block.rv_inst),
                                ptr: ll::Value::Reg(rv::Reg::Ra),
                            },
                            ll::Inst::Call {
                                rslt: ll::Value::Temp(address, 0),
                                func: ll::Value::Addr(addr),
                                regs: Vec::new(),
                                fregs: Vec::new(),
                            },
                            ll::Inst::DispRet {
                                addr: ll::Value::Addr(address),
                                next_pc: get_next_pc(&block.rv_inst),
                                stk: false,
                            },
                        ]
                    }
                    _ => (),
                }
            }
        }
    }
    prog
}

fn is_longjmp_except_func(func: &ll::Func) -> bool {
    let mut i = 0;
    while i < func.inst_blocks.len() - 1 {
        if let (
            rv::Inst::Auipc {
                rd: rv::Reg::Ra, ..
            },
            rv::Inst::OffsetJalr {
                rs1: rv::Reg::Ra, ..
            },
        ) = (
            &func.inst_blocks[i].rv_inst,
            &func.inst_blocks[i + 1].rv_inst,
        ) {
            return true;
        }
        i += 1;
    }
    if func
        .inst_blocks
        .iter()
        .position(|block| matches!(block.rv_inst.symbol(), Some(sym) if sym == "<_setjmp>"))
        .is_some()
    {
        return true;
    }
    func.inst_blocks
        .iter()
        .position(|block| matches!(block.rv_inst.symbol(), Some(sym) if sym == "<_Unwind_Resume>" || sym == "<__cxa_begin_catch>" || sym == "<__cxa_end_catch>"))
        .is_some()
}
