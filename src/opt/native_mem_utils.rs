use crate::llvm_isa::{Inst, Prog, Value};
use crate::riscv_isa as rv;
use rayon::prelude::*;

const NATIVE_MEM_UTILS: &str = "declare i32 @memcmp(ptr, ptr, i64)
declare void @llvm.memset.p0.i64(ptr, i8, i64, i1 immarg)
declare void @llvm.memcpy.p0.p0.i64(ptr, ptr, i64, i1 immarg)
declare void @llvm.memmove.p0.p0.i64(ptr, ptr, i64, i1 immarg)";

pub fn run(mut prog: Prog) -> Prog {
    prog.native_mem_utils = NATIVE_MEM_UTILS;
    prog.funcs.par_iter_mut().for_each(|func| {
        for block in &mut func.inst_blocks {
            if let rv::Inst::Jal {
                address, symbol, ..
            }
            | rv::Inst::PseudoJal {
                address, symbol, ..
            } = &block.rv_inst
            {
                match symbol.as_deref() {
                    Some("<memcmp>") => {
                        block.insts[1] = Inst::Memcmp {
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
                    _ => continue,
                }
            }
        }
    });
    prog
}
