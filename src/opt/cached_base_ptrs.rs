use crate::llvm_isa::{Func, Inst, Prog, Value};
use crate::riscv_isa as rv;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

macro_rules! use_cache {
    ( $rs1:ident, $func:ident, $block:ident, $cache:ident, $imm:ident) => {
        if $rs1 != &rv::Reg::Gp && ($func.stack_vars.is_empty() || $rs1 != &rv::Reg::Sp) {
            let Inst::Getmemptr { rslt, .. } = $block.insts[2] else { unreachable!() };
            if let Some((ptr, offset)) = $cache.get($rs1) {
                let inst = Inst::Getelementptr {
                    rslt,
                    ptr: *ptr,
                    offset: Value::Imm(rv::Imm($imm - offset)),
                };
                $block.insts.splice(0..3, vec![inst]);
            }
            $cache.insert($rs1, (rslt, *$imm));
        }
    };
}

pub fn run(mut prog: Prog) -> Prog {
    prog.funcs.par_iter_mut().for_each(|func| {
        if func.is_opaque {
            return;
        }
        let targets = find_jump_targets(func);
        let mut cache = HashMap::new();
        for block in &mut func.inst_blocks {
            if targets.contains(&block.rv_inst.address()) {
                cache.clear();
            }
            match &block.rv_inst {
                rv::Inst::Lb {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }|
                rv::Inst::Lh {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lw {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lbu {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lhu {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Lwu {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Ld {
                    rd,
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => {
                    use_cache!(rs1, func, block, cache, imm);
                    cache.remove(rd);
                }

                rv::Inst::Sb {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Sh {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Sw {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                |rv::Inst::Sd {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } |rv::Inst::Flw {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                }
                | rv::Inst::Fld {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } |rv::Inst::Fsw {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } |rv::Inst::Fsd {
                    imm: rv::Imm(imm),
                    rs1,
                    ..
                } => use_cache!(rs1, func, block, cache, imm),

                // RV32/RV64
                rv::Inst::Lui {rd,..}
                | rv::Inst::Auipc {rd,..}
                | rv::Inst::Addi {rd,..}
                | rv::Inst::Slti {rd,..}
                | rv::Inst::Sltiu {rd,..}
                | rv::Inst::Xori {rd,..}
                | rv::Inst::Ori {rd,..}
                | rv::Inst::Andi {rd,..}
                | rv::Inst::Slli {rd,..}
                | rv::Inst::Srli {rd,..}
                | rv::Inst::Srai {rd,..}
                | rv::Inst::Add {rd,..}
                | rv::Inst::Sub {rd,..}
                | rv::Inst::Sll {rd,..}
                | rv::Inst::Slt {rd,..}
                | rv::Inst::Sltu {rd,..}
                | rv::Inst::Xor {rd,..}
                | rv::Inst::Srl {rd,..}
                | rv::Inst::Sra {rd,..}
                | rv::Inst::Or {rd,..}
                | rv::Inst::And {rd,..}
                | rv::Inst::Addiw {rd,..}
                | rv::Inst::Slliw {rd,..}
                | rv::Inst::Srliw {rd,..}
                | rv::Inst::Sraiw {rd,..}
                | rv::Inst::Addw {rd,..}
                | rv::Inst::Subw {rd,..}
                | rv::Inst::Sllw {rd,..}
                | rv::Inst::Srlw {rd,..}
                | rv::Inst::Sraw {rd,..}

                // RV32/RV64 Zicsr
                | rv::Inst::Csrrw {rd,..}
                | rv::Inst::Csrrs {rd,..}
                | rv::Inst::Csrrc {rd,..}
                | rv::Inst::Csrrwi {rd,..}
                | rv::Inst::Csrrsi {rd,..}
                | rv::Inst::Csrrci {rd,..}

                // RV32M
                | rv::Inst::Mul {rd,..}
                | rv::Inst::Mulh {rd,..}
                | rv::Inst::Mulhsu {rd,..}
                | rv::Inst::Mulhu {rd,..}
                | rv::Inst::Div {rd,..}
                | rv::Inst::Divu {rd,..}
                | rv::Inst::Rem {rd,..}
                | rv::Inst::Remu {rd,..}

                // RV64M (in addition to RV32M)
                | rv::Inst::Mulw {rd,..}
                | rv::Inst::Divw {rd,..}
                | rv::Inst::Divuw {rd,..}
                | rv::Inst::Remw {rd,..}
                | rv::Inst::Remuw {rd,..}

                // RV32A
                | rv::Inst::LrW {rd,..}
                | rv::Inst::ScW {rd,..}
                | rv::Inst::AmoswapW {rd,..}
                | rv::Inst::AmoaddW {rd,..}
                | rv::Inst::AmoxorW {rd,..}
                | rv::Inst::AmoandW {rd,..}
                | rv::Inst::AmoorW {rd,..}
                | rv::Inst::AmominW {rd,..}
                | rv::Inst::AmomaxW {rd,..}
                | rv::Inst::AmominuW {rd,..}
                | rv::Inst::AmomaxuW {rd,..}

                // RV64A (in addition to RV32A)
                | rv::Inst::LrD {rd,..}
                | rv::Inst::ScD {rd,..}
                | rv::Inst::AmoswapD {rd,..}
                | rv::Inst::AmoaddD {rd,..}
                | rv::Inst::AmoxorD {rd,..}
                | rv::Inst::AmoandD {rd,..}
                | rv::Inst::AmoorD {rd,..}
                | rv::Inst::AmominD {rd,..}
                | rv::Inst::AmomaxD {rd,..}
                | rv::Inst::AmominuD {rd,..}
                | rv::Inst::AmomaxuD {rd,..}

                // RV32F/RV64F/RV32D/RV64D
                | rv::Inst::FcvtWS {rd,..}
                | rv::Inst::FcvtWuS {rd,..}
                | rv::Inst::FmvXW {rd,..}
                | rv::Inst::FeqS {rd,..}
                | rv::Inst::FltS {rd,..}
                | rv::Inst::FleS {rd,..}
                | rv::Inst::FclassS {rd,..}
                | rv::Inst::FcvtLS {rd,..}
                | rv::Inst::FcvtLuS {rd,..}
                | rv::Inst::FeqD {rd,..}
                | rv::Inst::FltD {rd,..}
                | rv::Inst::FleD {rd,..}
                | rv::Inst::FclassD {rd,..}
                | rv::Inst::FcvtWD {rd,..}
                | rv::Inst::FcvtWuD {rd,..}
                | rv::Inst::FcvtLD {rd,..}
                | rv::Inst::FcvtLuD {rd,..}
                | rv::Inst::FmvXD {rd,..}

                // Pseudoinstructions
                | rv::Inst::Li {rd,..}
                | rv::Inst::Mv {rd,..}
                | rv::Inst::Not {rd,..}
                | rv::Inst::Neg {rd,..}
                | rv::Inst::Negw {rd,..}
                | rv::Inst::SextW {rd,..}
                | rv::Inst::Seqz {rd,..}
                | rv::Inst::Snez {rd,..}
                | rv::Inst::Sltz {rd,..}
                | rv::Inst::Sgtz {rd,..}
                | rv::Inst::Rdinstret {rd,..}
                | rv::Inst::Rdcycle {rd,..}
                | rv::Inst::Rdtime {rd,..}
                | rv::Inst::Csrr {rd,..}
                | rv::Inst::Frcsr {rd,..}
                | rv::Inst::Fscsr {rd,..}
                | rv::Inst::Frrm {rd,..}
                | rv::Inst::Fsrm {rd,..}
                | rv::Inst::Frflags {rd,..}
                | rv::Inst::Fsflags {rd,..} => {cache.remove(rd);}

                // Function calls
                rv::Inst::Jal {rd,..}
                | rv::Inst::Jalr {rd,..} => {
                    cache.remove(rd);
                    cache.remove(&rv::Reg::Ra);
                    cache.remove(&rv::Reg::T0);
                    cache.remove(&rv::Reg::T1);
                    cache.remove(&rv::Reg::T2);
                    cache.remove(&rv::Reg::A0);
                    cache.remove(&rv::Reg::A1);
                    cache.remove(&rv::Reg::A2);
                    cache.remove(&rv::Reg::A3);
                    cache.remove(&rv::Reg::A4);
                    cache.remove(&rv::Reg::A5);
                    cache.remove(&rv::Reg::A6);
                    cache.remove(&rv::Reg::A7);
                    cache.remove(&rv::Reg::T3);
                    cache.remove(&rv::Reg::T4);
                    cache.remove(&rv::Reg::T5);
                    cache.remove(&rv::Reg::T6);
                }
                rv::Inst::Ecall {..} => {cache.remove(&rv::Reg::A0);}
                rv::Inst::PseudoJal{..}
                | rv::Inst::PseudoJalr{..}
                | rv::Inst::OffsetJalr{..} => {
                    cache.remove(&rv::Reg::Ra);
                    cache.remove(&rv::Reg::T0);
                    cache.remove(&rv::Reg::T1);
                    cache.remove(&rv::Reg::T2);
                    cache.remove(&rv::Reg::A0);
                    cache.remove(&rv::Reg::A1);
                    cache.remove(&rv::Reg::A2);
                    cache.remove(&rv::Reg::A3);
                    cache.remove(&rv::Reg::A4);
                    cache.remove(&rv::Reg::A5);
                    cache.remove(&rv::Reg::A6);
                    cache.remove(&rv::Reg::A7);
                    cache.remove(&rv::Reg::T3);
                    cache.remove(&rv::Reg::T4);
                    cache.remove(&rv::Reg::T5);
                    cache.remove(&rv::Reg::T6);
                }

                _ => continue,
            }
        }
    });
    prog
}

fn find_jump_targets(func: &Func) -> HashSet<rv::Addr> {
    let mut targets = HashSet::new();
    for block in &func.inst_blocks {
        match &block.rv_inst {
            rv::Inst::Beq { addr, .. }
            | rv::Inst::Bne { addr, .. }
            | rv::Inst::Blt { addr, .. }
            | rv::Inst::Bge { addr, .. }
            | rv::Inst::Bltu { addr, .. }
            | rv::Inst::Bgeu { addr, .. }
            | rv::Inst::Beqz { addr, .. }
            | rv::Inst::Bnez { addr, .. }
            | rv::Inst::Blez { addr, .. }
            | rv::Inst::Bgez { addr, .. }
            | rv::Inst::Bltz { addr, .. }
            | rv::Inst::Bgtz { addr, .. }
            | rv::Inst::J { addr, .. } => targets.insert(*addr),
            _ => continue,
        };
    }
    targets
}
