use crate::llvm_isa as ll;
use crate::riscv_isa as rv;
use std::collections::HashSet;

pub fn run(mut prog: ll::Program) -> ll::Program {
    for func in &mut prog.funcs {
        if func.dynamic {
            continue;
        }
        let mut regs = HashSet::new();
        let mut fregs = HashSet::new();
        let mut ecall = false;
        for block in &func.inst_blocks {
            let (rs, frs) = get_regs(block.rv_inst.clone());
            regs.extend(rs);
            fregs.extend(frs);
            if let rv::Inst::Ecall { .. } = block.rv_inst {
                ecall = true;
            }
        }
        if ecall {
            regs.extend(vec![
                rv::Reg::A7,
                rv::Reg::A0,
                rv::Reg::A1,
                rv::Reg::A2,
                rv::Reg::A3,
                rv::Reg::A4,
                rv::Reg::A5,
            ]);
        }
        func.used_regs = regs.into_iter().collect();
        func.used_fregs = fregs.into_iter().collect();
        for block in &mut func.inst_blocks {
            for inst in &mut block.insts {
                if let ll::Inst::Call { regs, fregs, .. } = inst {
                    *regs = func.used_regs.clone();
                    *fregs = func.used_fregs.clone();
                }
                match inst {
                    ll::Inst::Load { ptr, .. } => {
                        if let ll::Value::Reg(reg) = ptr {
                            *ptr = ll::Value::StkReg(*reg);
                        } else if let ll::Value::FReg(freg) = ptr {
                            *ptr = ll::Value::StkFReg(*freg);
                        }
                    }
                    ll::Inst::Store { ptr, .. } => {
                        if let ll::Value::Reg(reg) = ptr {
                            *ptr = ll::Value::StkReg(*reg);
                        } else if let ll::Value::FReg(freg) = ptr {
                            *ptr = ll::Value::StkFReg(*freg);
                        }
                    }
                    ll::Inst::ContRet { stk, .. } => *stk = true,
                    ll::Inst::DispRet { stk, .. } => *stk = true,
                    ll::Inst::DispFunc { regs, fregs, .. } => {
                        *regs = func.used_regs.clone();
                        *fregs = func.used_fregs.clone();
                    }
                    ll::Inst::CheckRet { stk, .. } => *stk = true,
                    _ => (),
                }
            }
        }
    }
    prog
}

fn get_regs(inst: rv::Inst) -> (Vec<rv::Reg>, Vec<rv::FReg>) {
    use rv::Inst::*;

    match inst {
        // RV32I
        Lui { rd, .. } => (vec![rd], vec![]),
        Auipc { rd, .. } => (vec![rd], vec![]),
        Jal { rd, .. } => (vec![rd], vec![]),
        Jalr { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Beq { rs1, rs2, .. } => (vec![rs1, rs2], vec![]),
        Bne { rs1, rs2, .. } => (vec![rs1, rs2], vec![]),
        Blt { rs1, rs2, .. } => (vec![rs1, rs2], vec![]),
        Bge { rs1, rs2, .. } => (vec![rs1, rs2], vec![]),
        Bltu { rs1, rs2, .. } => (vec![rs1, rs2], vec![]),
        Bgeu { rs1, rs2, .. } => (vec![rs1, rs2], vec![]),
        Lb { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Lh { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Lw { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Lbu { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Lhu { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Sb { rs2, rs1, .. } => (vec![rs2, rs1], vec![]),
        Sh { rs2, rs1, .. } => (vec![rs2, rs1], vec![]),
        Sw { rs2, rs1, .. } => (vec![rs2, rs1], vec![]),
        Addi { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Slti { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Sltiu { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Xori { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Ori { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Andi { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Slli { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Srli { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Srai { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Add { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Sub { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Sll { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Slt { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Sltu { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Xor { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Srl { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Sra { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Or { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        And { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Fence { .. } => (vec![], vec![]),
        Ecall { .. } => (vec![], vec![]),
        Ebreak { .. } => (vec![], vec![]),

        // RV64I (in addition to RV32I)
        Lwu { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Ld { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Sd { rs2, rs1, .. } => (vec![rs2, rs1], vec![]),
        // `slli` is the same as its RV32I version.
        // `srli` is the same as its RV32I version.
        // `srai` is the same as its RV32I version.
        Addiw { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Slliw { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Srliw { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Sraiw { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Addw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Subw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Sllw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Srlw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Sraw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),

        // RV32/RV64 Zifencei
        FenceI { .. } => (vec![], vec![]),

        // RV32/RV64 Zicsr
        Csrrw { rd, .. } => (vec![rd], vec![]),
        Csrrs { rd, .. } => (vec![rd], vec![]),
        Csrrc { rd, .. } => (vec![rd], vec![]),
        Csrrwi { rd, .. } => (vec![rd], vec![]),
        Csrrsi { rd, .. } => (vec![rd], vec![]),
        Csrrci { rd, .. } => (vec![rd], vec![]),

        // RV32M
        Mul { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Mulh { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Mulhsu { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Mulhu { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Div { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Divu { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Rem { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Remu { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),

        // RV64M (in addition to RV32M)
        Mulw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Divw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Divuw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Remw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),
        Remuw { rd, rs1, rs2, .. } => (vec![rd, rs1, rs2], vec![]),

        // RV32A
        LrW { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        ScW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoswapW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoaddW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoxorW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoandW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoorW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmominW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmomaxW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmominuW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmomaxuW { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),

        // RV64A (in addition to RV32A)
        LrD { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        ScD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoswapD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoaddD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoxorD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoandD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmoorD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmominD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmomaxD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmominuD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),
        AmomaxuD { rd, rs2, rs1, .. } => (vec![rd, rs2, rs1], vec![]),

        // RV32F
        Flw { frd, rs1, .. } => (vec![rs1], vec![frd]),
        Fsw { frs2, rs1, .. } => (vec![rs1], vec![frs2]),
        FmaddS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FmsubS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FnmsubS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FnmaddS {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FaddS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsubS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FmulS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FdivS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsqrtS { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FsgnjS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsgnjnS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsgnjxS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FminS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FmaxS {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FcvtWS { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FcvtWuS { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FmvXW { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FeqS { rd, frs1, frs2, .. } => (vec![rd], vec![frs1, frs2]),
        FltS { rd, frs1, frs2, .. } => (vec![rd], vec![frs1, frs2]),
        FleS { rd, frs1, frs2, .. } => (vec![rd], vec![frs1, frs2]),
        FclassS { rd, .. } => (vec![rd], vec![]),
        FcvtSW { frd, rs1, .. } => (vec![rs1], vec![frd]),
        FcvtSWu { frd, rs1, .. } => (vec![rs1], vec![frd]),
        FmvWX { frd, rs1, .. } => (vec![rs1], vec![frd]),

        // RV64F (in addition to RV32F)
        FcvtLS { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FcvtLuS { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FcvtSL { frd, rs1, .. } => (vec![rs1], vec![frd]),
        FcvtSLu { frd, rs1, .. } => (vec![rs1], vec![frd]),

        // RV32D
        Fld { frd, rs1, .. } => (vec![rs1], vec![frd]),
        Fsd { frs2, rs1, .. } => (vec![rs1], vec![frs2]),
        FmaddD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FmsubD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FnmsubD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FnmaddD {
            frd,
            frs1,
            frs2,
            frs3,
            ..
        } => (vec![], vec![frd, frs1, frs2, frs3]),
        FaddD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsubD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FmulD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FdivD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsqrtD { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FsgnjD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsgnjnD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FsgnjxD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FminD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FmaxD {
            frd, frs1, frs2, ..
        } => (vec![], vec![frd, frs1, frs2]),
        FcvtSD { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FcvtDS { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FeqD { rd, frs1, frs2, .. } => (vec![rd], vec![frs1, frs2]),
        FltD { rd, frs1, frs2, .. } => (vec![rd], vec![frs1, frs2]),
        FleD { rd, frs1, frs2, .. } => (vec![rd], vec![frs1, frs2]),
        FclassD { rd, .. } => (vec![rd], vec![]),
        FcvtWD { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FcvtWuD { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FcvtDW { frd, rs1, .. } => (vec![rs1], vec![frd]),
        FcvtDWu { frd, rs1, .. } => (vec![rs1], vec![frd]),

        // RV64D (in addition to RV32D)
        FcvtLD { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FcvtLuD { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FmvXD { rd, frs1, .. } => (vec![rd], vec![frs1]),
        FcvtDL { frd, rs1, .. } => (vec![rs1], vec![frd]),
        FcvtDLu { frd, rs1, .. } => (vec![rs1], vec![frd]),
        FmvDX { frd, rs1, .. } => (vec![rs1], vec![frd]),

        // Pseudoinstructions
        // Pseudoinstructions using symbols are compiled to other instructions.
        Nop { .. } => (vec![], vec![]),
        Li { rd, .. } => (vec![rd], vec![]),
        Mv { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Not { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Neg { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Negw { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        SextW { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Seqz { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Snez { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Sltz { rd, rs1, .. } => (vec![rd, rs1], vec![]),
        Sgtz { rd, rs1, .. } => (vec![rd, rs1], vec![]),

        FmvS { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FabsS { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FnegS { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FmvD { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FabsD { frd, frs1, .. } => (vec![], vec![frd, frs1]),
        FnegD { frd, frs1, .. } => (vec![], vec![frd, frs1]),

        Beqz { rs1, .. } => (vec![rs1], vec![]),
        Bnez { rs1, .. } => (vec![rs1], vec![]),
        Blez { rs1, .. } => (vec![rs1], vec![]),
        Bgez { rs1, .. } => (vec![rs1], vec![]),
        Bltz { rs1, .. } => (vec![rs1], vec![]),
        Bgtz { rs1, .. } => (vec![rs1], vec![]),

        // `bgt` is compiled to other instructions.
        // `ble` is compiled to other instructions.
        // `bgtu` is compiled to other instructions.
        // `bleu` is compiled to other instructions.
        J { .. } => (vec![], vec![]),
        PseudoJal { .. } => (vec![rv::Reg::Ra], vec![]),
        Jr { rs1, .. } => (vec![rs1], vec![]),
        PseudoJalr { rs1, .. } => (vec![rs1, rv::Reg::Ra], vec![]),
        Ret { .. } => (vec![rv::Reg::Ra], vec![]),
        // `call` is compiled to other instructions.
        // `tail` is compiled to other instructions.
        PseudoFence { .. } => (vec![], vec![]),

        Rdinstret { rd, .. } => (vec![rd], vec![]),
        Rdcycle { rd, .. } => (vec![rd], vec![]),
        Rdtime { rd, .. } => (vec![rd], vec![]),

        Csrr { rd, .. } => (vec![rd], vec![]),
        Csrw { .. } => (vec![], vec![]),
        Csrs { .. } => (vec![], vec![]),
        Csrc { .. } => (vec![], vec![]),

        Csrwi { .. } => (vec![], vec![]),
        Csrsi { .. } => (vec![], vec![]),
        Csrci { .. } => (vec![], vec![]),

        Frcsr { rd, .. } => (vec![rd], vec![]),
        Fscsr { rd, .. } => (vec![rd], vec![]),
        PseudoFscsr { .. } => (vec![], vec![]),

        Frrm { rd, .. } => (vec![rd], vec![]),
        Fsrm { rd, .. } => (vec![rd], vec![]),
        PseudoFsrm { .. } => (vec![], vec![]),

        Frflags { rd, .. } => (vec![rd], vec![]),
        Fsflags { rd, .. } => (vec![rd], vec![]),
        PseudoFsflags { .. } => (vec![], vec![]),

        // Misc
        Unimp { .. } => (vec![], vec![]),
        OffsetJalr { rs1, .. } => (vec![rs1, rv::Reg::Ra], vec![]),
        OffsetJr { rs1, .. } => (vec![rs1], vec![]),
    }
}
