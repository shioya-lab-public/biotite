use crate::llvm_isa as ll;
use crate::llvm_macro::next_pc;
use crate::riscv_isa as rv;

fn get_next_pc(inst: &rv::Inst) -> ll::Value {
    use crate::llvm_isa::*;
    use crate::riscv_isa as RV;
    next_pc!(next_pc, inst.address(), inst.is_compressed())
}

pub fn longjmp_except(mut prog: ll::Program) -> ll::Program {
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
