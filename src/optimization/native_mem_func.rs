use crate::llvm_isa as ll;
use crate::riscv_isa as rv;
use std::collections::HashSet;

pub fn native_mem_func(mut prog: ll::Program) -> ll::Program {
    prog.native_mem_func = true;
    for func in &mut prog.funcs {
        for block in &mut func.inst_blocks {
            if let rv::Inst::PseudoJal {
                symbol, address, ..
            }
            | rv::Inst::Jal {
                symbol, address, ..
            } = &block.rv_inst
            {
                match symbol.as_deref() {
                    Some("<memcpy>") => {
                        block.insts = vec![ll::Inst::Memcpy {
                            addr: ll::Value::Addr(*address),
                            stk: !func.dynamic,
                        }]
                    }
                    Some("<memmove>") => {
                        block.insts = vec![ll::Inst::Memmove {
                            addr: ll::Value::Addr(*address),
                            stk: !func.dynamic,
                        }]
                    }
                    Some("<memset>") => {
                        block.insts = vec![ll::Inst::Memset {
                            addr: ll::Value::Addr(*address),
                            stk: !func.dynamic,
                        }]
                    }
                    Some("<memcmp>") => {
                        block.insts = vec![ll::Inst::Memcmp {
                            addr: ll::Value::Addr(*address),
                            stk: !func.dynamic,
                        }]
                    }
                    _ => continue,
                }
                if !func.dynamic {
                    func.used_regs
                        .extend(vec![rv::Reg::A0, rv::Reg::A1, rv::Reg::A2]);
                    let set: HashSet<_> = func.used_regs.drain(..).collect();
                    func.used_regs.extend(set.into_iter());
                }
            }
        }
    }
    prog
}
