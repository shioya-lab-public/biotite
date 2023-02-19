use crate::llvm_isa::{Inst, Prog, Value};
use crate::riscv_isa as rv;

pub fn run(mut prog: Prog) -> Prog {
    prog.native_mem_utils = true;
    for func in &mut prog.funcs {
        for block in &mut func.inst_blocks {
            if let rv::Inst::Jal {
                symbol, address, ..
            }
            | rv::Inst::PseudoJal {
                symbol, address, ..
            } = &block.rv_inst
            {
                match symbol.as_deref() {
                    Some("<memcpy>") => {
                        block.insts[1] = Inst::Memcpy {
                            addr: Value::Addr(*address),
                            stk: false,
                        }
                    }
                    Some("<memmove>") => {
                        block.insts[1] = Inst::Memmove {
                            addr: Value::Addr(*address),
                            stk: false,
                        }
                    }
                    Some("<memset>") => {
                        block.insts[1] = Inst::Memset {
                            addr: Value::Addr(*address),
                            stk: false,
                        }
                    }
                    Some("<memcmp>") => {
                        block.insts[1] = Inst::Memcmp {
                            addr: Value::Addr(*address),
                            stk: false,
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
    prog
}
