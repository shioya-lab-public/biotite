use crate::llvm_isa as ll;
use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;

fn get_next_pc(inst: &rv::Inst) -> ll::Value {
    use crate::llvm_isa::*;
    use crate::riscv_isa as RV;
    next_pc!(next_pc, inst.address(), inst.is_compressed())
}

pub fn longjmp(mut prog: ll::Program) -> ll::Program {
    for func in &mut prog.funcs {
        if is_longjmp_func(func) {
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
                            },
                            ll::Inst::DispRet {
                                addr: ll::Value::Addr(address),
                                next_pc: get_next_pc(&block.rv_inst),
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
                            },
                            ll::Inst::DispRet {
                                addr: ll::Value::Addr(address),
                                next_pc: get_next_pc(&block.rv_inst),
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

fn is_longjmp_func(func: &ll::Func) -> bool {
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
    func.inst_blocks
        .iter()
        .position(|block| matches!(block.rv_inst.symbol(), Some(sym) if sym == "<_setjmp>"))
        .is_some()
}
