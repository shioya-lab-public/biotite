use crate::llvm_isa::{Inst, Prog, Value};
use crate::riscv_isa as rv;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn run(mut prog: Prog) -> Prog {
    prog.funcs.par_iter_mut().for_each(|func| {
        let mut used_regs = HashSet::new();
        let mut used_fregs = HashSet::new();
        let mut overwritten_regs = HashSet::new();
        let mut overwritten_fregs = HashSet::new();
        let mut branch = false;
        for block in &func.inst_blocks {
            if let rv::Inst::Jal { .. }
            | rv::Inst::Jalr { .. }
            | rv::Inst::Beq { .. }
            | rv::Inst::Bne { .. }
            | rv::Inst::Blt { .. }
            | rv::Inst::Bge { .. }
            | rv::Inst::Bltu { .. }
            | rv::Inst::Bgeu { .. }
            | rv::Inst::Beqz { .. }
            | rv::Inst::Bnez { .. }
            | rv::Inst::Blez { .. }
            | rv::Inst::Bgez { .. }
            | rv::Inst::Bltz { .. }
            | rv::Inst::Bgtz { .. }
            | rv::Inst::J { .. }
            | rv::Inst::PseudoJal { .. }
            | rv::Inst::Jr { .. }
            | rv::Inst::PseudoJalr { .. }
            | rv::Inst::OffsetJalr { .. }
            | rv::Inst::OffsetJr { .. } = block.rv_inst
            {
                branch = true;
            }
            let (dest_regs, dest_fregs, src_regs, src_fregs) = get_regs(&block.rv_inst);

            used_regs.extend(src_regs);
            used_fregs.extend(src_fregs);
            if !branch {
                for dest_reg in &dest_regs {
                    if !used_regs.contains(dest_reg) {
                        overwritten_regs.insert(*dest_reg);
                    }
                }
                for dest_freg in &dest_fregs {
                    if !used_fregs.contains(dest_freg) {
                        overwritten_fregs.insert(*dest_freg);
                    }
                }
            }
            used_regs.extend(dest_regs);
            used_fregs.extend(dest_fregs);

            if let rv::Inst::J { .. } = &block.rv_inst {
                if let Inst::Call { .. } = &block.insts[0] {
                    used_regs.insert(rv::Reg::Ra);
                }
            }
            if let rv::Inst::Jal { .. } | rv::Inst::PseudoJal { .. } = &block.rv_inst {
                if let Inst::Memcpy { .. }
                | Inst::Memmove { .. }
                | Inst::Memset { .. }
                | Inst::Memcmp { .. } = &block.insts[1]
                {
                    used_regs.extend(vec![rv::Reg::A0, rv::Reg::A1, rv::Reg::A2]);
                }
            }
        }

        let synced_regs = &used_regs ^ &overwritten_regs;
        let synced_fregs = &used_fregs ^ &overwritten_fregs;
        if func.is_opaque {
            func.synced_regs = used_regs.clone().into_iter().collect();
            func.synced_fregs = used_fregs.clone().into_iter().collect();
        } else {
            func.synced_regs = synced_regs.into_iter().collect();
            func.synced_fregs = synced_fregs.into_iter().collect();
        }
        func.used_regs = used_regs.into_iter().collect();
        func.used_fregs = used_fregs.into_iter().collect();
        func.used_regs.sort_unstable();
        func.used_fregs.sort_unstable();
        func.synced_regs.sort_unstable();
        func.synced_fregs.sort_unstable();

        for block in &mut func.inst_blocks {
            for inst in &mut block.insts {
                match inst {
                    Inst::Call {
                        used_regs,
                        used_fregs,
                        ..
                    }
                    | Inst::Dispfunc {
                        used_regs,
                        used_fregs,
                        ..
                    } => {
                        *used_regs = func.used_regs.clone();
                        *used_fregs = func.used_fregs.clone();
                    }
                    Inst::Load { ptr, .. } | Inst::Store { ptr, .. } => {
                        if let Value::Reg(reg) = ptr {
                            *ptr = Value::StkReg(*reg);
                        } else if let Value::FReg(freg) = ptr {
                            *ptr = Value::StkFReg(*freg);
                        }
                    }
                    Inst::Checkret { stk, .. }
                    | Inst::Contret { stk, .. }
                    | Inst::Dispret { stk, .. }
                    | Inst::Memcpy { stk, .. }
                    | Inst::Memmove { stk, .. }
                    | Inst::Memset { stk, .. }
                    | Inst::Memcmp { stk, .. } => *stk = true,
                    _ => continue,
                }
            }
        }
    });

    prog
}

fn get_regs(inst: &rv::Inst) -> (Vec<rv::Reg>, Vec<rv::FReg>, Vec<rv::Reg>, Vec<rv::FReg>) {
    use rv::Inst::*;

    match inst.clone() {
        // RV32I
        Lui { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Auipc { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Jal { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Jalr { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Beq { rs1, rs2, .. } => (vec![], vec![], vec![rs1, rs2], vec![]),
        Bne { rs1, rs2, .. } => (vec![], vec![], vec![rs1, rs2], vec![]),
        Blt { rs1, rs2, .. } => (vec![], vec![], vec![rs1, rs2], vec![]),
        Bge { rs1, rs2, .. } => (vec![], vec![], vec![rs1, rs2], vec![]),
        Bltu { rs1, rs2, .. } => (vec![], vec![], vec![rs1, rs2], vec![]),
        Bgeu { rs1, rs2, .. } => (vec![], vec![], vec![rs1, rs2], vec![]),
        Lb { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Lh { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Lw { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Lbu { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Lhu { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Sb { rs2, rs1, .. } => (vec![], vec![], vec![rs2, rs1], vec![]),
        Sh { rs2, rs1, .. } => (vec![], vec![], vec![rs2, rs1], vec![]),
        Sw { rs2, rs1, .. } => (vec![], vec![], vec![rs2, rs1], vec![]),
        Addi { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Slti { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Sltiu { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Xori { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Ori { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Andi { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Slli { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Srli { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Srai { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Add { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Sub { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Sll { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Slt { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Sltu { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Xor { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Srl { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Sra { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Or { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        And { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Fence { .. } => (vec![], vec![], vec![], vec![]),
        Ecall { .. } => (
            vec![],
            vec![],
            vec![
                rv::Reg::A7,
                rv::Reg::A0,
                rv::Reg::A1,
                rv::Reg::A2,
                rv::Reg::A3,
                rv::Reg::A4,
                rv::Reg::A5,
            ],
            vec![],
        ),
        Ebreak { .. } => (vec![], vec![], vec![], vec![]),

        // RV64I (in addition to RV32I)
        Lwu { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Ld { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Sd { rs2, rs1, .. } => (vec![], vec![], vec![rs2, rs1], vec![]),
        Addiw { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Slliw { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Srliw { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Sraiw { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Addw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Subw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Sllw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Srlw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Sraw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),

        // RV32/RV64 Zifencei
        FenceI { .. } => (vec![], vec![], vec![], vec![]),

        // RV32/RV64 Zicsr
        Csrrw { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Csrrs { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Csrrc { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Csrrwi { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Csrrsi { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Csrrci { rd, .. } => (vec![rd], vec![], vec![], vec![]),

        // RV32M
        Mul { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Mulh { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Mulhsu { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Mulhu { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Div { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Divu { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Rem { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Remu { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),

        // RV64M (in addition to RV32M)
        Mulw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Divw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Divuw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Remw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),
        Remuw { rd, rs1, rs2, .. } => (vec![rd], vec![], vec![rs1, rs2], vec![]),

        // RV32A
        LrW { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        ScW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoswapW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoaddW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoxorW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoandW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoorW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmominW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmomaxW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmominuW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmomaxuW { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),

        // RV64A (in addition to RV32A)
        LrD { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        ScD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoswapD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoaddD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoxorD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoandD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmoorD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmominD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmomaxD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmominuD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),
        AmomaxuD { rd, rs2, rs1, .. } => (vec![rd], vec![], vec![rs2, rs1], vec![]),

        // RV32F
        Flw { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        Fsw { frs2, rs1, .. } => (vec![], vec![], vec![rs1], vec![frs2]),
        FmaddS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FmsubS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FnmsubS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FnmaddS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FaddS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsubS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FmulS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FdivS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsqrtS { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FsgnjS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsgnjnS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsgnjxS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FminS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FmaxS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FcvtWS { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FcvtWuS { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FmvXW { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FeqS { rd, frs1, frs2, .. } => (vec![rd], vec![], vec![], vec![frs1, frs2]),
        FltS { rd, frs1, frs2, .. } => (vec![rd], vec![], vec![], vec![frs1, frs2]),
        FleS { rd, frs1, frs2, .. } => (vec![rd], vec![], vec![], vec![frs1, frs2]),
        FclassS { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        FcvtSW { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        FcvtSWu { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        FmvWX { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),

        // RV64F (in addition to RV32F)
        FcvtLS { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FcvtLuS { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FcvtSL { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        FcvtSLu { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),

        // RV32D
        Fld { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        Fsd { frs2, rs1, .. } => (vec![], vec![], vec![rs1], vec![frs2]),
        FmaddD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FmsubD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FnmsubD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FnmaddD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2, frs3]),
        FaddD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsubD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FmulD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FdivD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsqrtD { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FsgnjD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsgnjnD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FsgnjxD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FminD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FmaxD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd], vec![], vec![frs1, frs2]),
        FcvtSD { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FcvtDS { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FeqD { rd, frs1, frs2, .. } => (vec![rd], vec![], vec![], vec![frs1, frs2]),
        FltD { rd, frs1, frs2, .. } => (vec![rd], vec![], vec![], vec![frs1, frs2]),
        FleD { rd, frs1, frs2, .. } => (vec![rd], vec![], vec![], vec![frs1, frs2]),
        FclassD { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        FcvtWD { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FcvtWuD { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FcvtDW { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        FcvtDWu { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),

        // RV64D (in addition to RV32D)
        FcvtLD { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FcvtLuD { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FmvXD { rd, frs1, .. } => (vec![rd], vec![], vec![], vec![frs1]),
        FcvtDL { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        FcvtDLu { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),
        FmvDX { frd, rs1, .. } => (vec![], vec![frd], vec![rs1], vec![]),

        // Pseudoinstructions
        Nop { .. } => (vec![], vec![], vec![], vec![]),
        Li { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Mv { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Not { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Neg { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Negw { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        SextW { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Seqz { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Snez { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Sltz { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),
        Sgtz { rd, rs1, .. } => (vec![rd], vec![], vec![rs1], vec![]),

        FmvS { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FabsS { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FnegS { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FmvD { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FabsD { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),
        FnegD { frd, frs1, .. } => (vec![], vec![frd], vec![], vec![frs1]),

        Beqz { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),
        Bnez { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),
        Blez { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),
        Bgez { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),
        Bltz { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),
        Bgtz { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),

        J { .. } => (vec![], vec![], vec![], vec![]),
        PseudoJal { .. } => (vec![rv::Reg::Ra], vec![], vec![], vec![]),
        Jr { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),
        PseudoJalr { rs1, .. } => (vec![rv::Reg::Ra], vec![], vec![rs1], vec![]),
        Ret { .. } => (vec![], vec![], vec![rv::Reg::Ra], vec![]),

        PseudoFence { .. } => (vec![], vec![], vec![], vec![]),

        Rdinstret { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Rdcycle { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Rdtime { rd, .. } => (vec![rd], vec![], vec![], vec![]),

        Csrr { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Csrw { .. } => (vec![], vec![], vec![], vec![]),
        Csrs { .. } => (vec![], vec![], vec![], vec![]),
        Csrc { .. } => (vec![], vec![], vec![], vec![]),

        Csrwi { .. } => (vec![], vec![], vec![], vec![]),
        Csrsi { .. } => (vec![], vec![], vec![], vec![]),
        Csrci { .. } => (vec![], vec![], vec![], vec![]),

        Frcsr { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Fscsr { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        PseudoFscsr { .. } => (vec![], vec![], vec![], vec![]),

        Frrm { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Fsrm { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        PseudoFsrm { .. } => (vec![], vec![], vec![], vec![]),

        Frflags { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        Fsflags { rd, .. } => (vec![rd], vec![], vec![], vec![]),
        PseudoFsflags { .. } => (vec![], vec![], vec![], vec![]),

        // Misc
        Unimp { .. } => (vec![], vec![], vec![], vec![]),
        OffsetJalr { rs1, .. } => (vec![rv::Reg::Ra], vec![], vec![rs1], vec![]),
        OffsetJr { rs1, .. } => (vec![], vec![], vec![rs1], vec![]),
    }
}
