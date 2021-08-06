use crate::{addr, define_instruction, imm, rd, rs1, rs2};

pub trait FromStr {
    fn from_str(s: &str) -> Self;
}

define_instruction! {
    // RV64I
    Add(rd, rs1, rs2),
    Addw(rd, rs1, rs2),
    Addi(rd, rs1, imm),
    Addiw(rd, rs1, imm),
    And(rd, rs1, rs2),
    Andi(rd, rs1, imm),
    Auipc(rd, imm),
    Beq(rs1, rs2, addr),
    Bge(rs1, rs2, addr),
    Bgeu(rs1, rs2, addr),
    Blt(rs1, rs2, addr),
    Bltu(rs1, rs2, addr),
    Bne(rs1, rs2, addr),
    // `csrrc` is not implemented.
    // `csrrci` is not implemented.
    // `csrrs` is not implemented.
    // `csrrsi` is not implemented.
    // `csrrw` is not implemented.
    // `csrrwi` is not implemented.
    Ebreak(),
    Ecall(),
    // `fence` is not implemented.
    // `fence.i` is not implemented.
    Jal(rd, addr),
    Jalr(rd, rs1, imm),
    Lb(rd, rs1, imm),
    Lbu(rd, rs1, imm),
    Ld(rd, rs1, imm),
    Lh(rd, rs1, imm),
    Lhu(rd, rs1, imm),
    Lui(rd, imm),
    Lw(rd, rs1, imm),
    Lwu(rd, rs1, imm),
    Or(rd, rs1, rs2),
    Ori(rd, rs1, imm),
    Sb(rs1, imm, rs2),
    Sd(rs1, imm, rs2),
    Sh(rs1, imm, rs2),
    Sll(rd, rs1, rs2),
    Sllw(rd, rs1, rs2),
    Slli(rd, rs1, imm),
    Slliw(rd, rs1, imm),
    Slt(rd, rs1, rs2),
    Slti(rd, rs1, imm),
    Sltiu(rd, rs1, imm),
    Sltu(rd, rs1, rs2),
    Sra(rd, rs1, rs2),
    Sraw(rd, rs1, rs2),
    Srai(rd, rs1, imm),
    Sraiw(rd, rs1, imm),
    Srl(rd, rs1, rs2),
    Srlw(rd, rs1, rs2),
    Srli(rd, rs1, imm),
    Srliw(rd, rs1, imm),
    Sub(rd, rs1, rs2),
    Subw(rd, rs1, rs2),
    Sw(rs1, imm, rs2),
    Xor(rd, rs1, rs2),
    Xori(rd, rs1, imm),

    // Pseudo
    Beqz(rs1, addr),
    Bnez(rs1, addr),
    // `fabs.s` is not implemented.
    // `fabs.d` is not implemented.
    // `fmv.s` is not implemented.
    // `fmv.d` is not implemented.
    // `fneg.s` is not implemented.
    // `fneg.d` is not implemented.
    J(addr),
    Jr(rs1),
    // `la` will not appear in actual assembly.
    Li(rd, imm),
    Mv(rd, rs1),
    Neg(rd, rs1),
    Nop(),
    Not(rd, rs1),
    Ret(),
    Seqz(rd, rs1),
    Snez(rd, rs1),

    // Misc
    SextW(rd, rs1),
}

pub type RiscvImmediate = i64;

impl FromStr for RiscvImmediate {
    fn from_str(s: &str) -> Self {
        if s == "default" {
            0
        } else if let Some(s) = s.strip_prefix("0x") {
            RiscvImmediate::from_str_radix(s, 16).unwrap()
        } else {
            s.parse().unwrap()
        }
    }
}

pub type RiscvAddress = usize;

impl FromStr for RiscvAddress {
    fn from_str(s: &str) -> Self {
        RiscvAddress::from_str_radix(s, 16).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RiscvRegister {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

impl FromStr for RiscvRegister {
    fn from_str(s: &str) -> Self {
        use RiscvRegister::*;

        match s {
            "zero" => Zero,
            "ra" => Ra,
            "sp" => Sp,
            "gp" => Gp,
            "tp" => Tp,
            "t0" => T0,
            "t1" => T1,
            "t2" => T2,
            "s0" => S0,
            "s1" => S1,
            "a0" => A0,
            "a1" => A1,
            "a2" => A2,
            "a3" => A3,
            "a4" => A4,
            "a5" => A5,
            "a6" => A6,
            "a7" => A7,
            "s2" => S2,
            "s3" => S3,
            "s4" => S4,
            "s5" => S5,
            "s6" => S6,
            "s7" => S7,
            "s8" => S8,
            "s9" => S9,
            "s10" => S10,
            "s11" => S11,
            "t3" => T3,
            "t4" => T4,
            "t5" => T5,
            "t6" => T6,
            "default" => Ra,
            _ => unreachable!(),
        }
    }
}

pub mod riscv_regex {
    use crate::define_regex;
    use regex::Regex;

    const ADDRESS: &str = r"(?P<address>[[:xdigit:]]+)";
    const LAB: &str = r"<(?P<label>\S+)>";
    const RD: &str = r"(?P<rd>\S+)";
    const RS1: &str = r"(?P<rs1>\S+)";
    const RS2: &str = r"(?P<rs2>\S+)";
    const IMM: &str = r"(?P<imm>\S+)";
    const ADDR: &str = r"(?P<addr>[[:xdigit:]]+)";
    const COMM: &str = r"(?P<comm>\s+.+)?";

    define_regex! {
        // function labels
        LABEL(r"{} {}:", ADDRESS, LAB),

        // RV64I
        ADD(r"{}:.+\s+add\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        ADDW(r"{}:.+\s+addw\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        ADDI(r"{}:.+\s+addi\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        ADDIW(r"{}:.+\s+addiw\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        AND(r"{}:.+\s+and\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        ANDI(r"{}:.+\s+andi\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        AUIPC(r"{}:.+\s+auipc\s+{},{}{}", ADDRESS, RD, IMM, COMM),
        BEQ(r"{}:.+\s+beq\s+{},{},{}{}", ADDRESS, RS1, RS2, ADDR, COMM),
        BGE(r"{}:.+\s+bge\s+{},{},{}{}", ADDRESS, RS1, RS2, ADDR, COMM),
        BGEU(r"{}:.+\s+bgeu\s+{},{},{}{}", ADDRESS, RS1, RS2, ADDR, COMM),
        BLT(r"{}:.+\s+blt\s+{},{},{}{}", ADDRESS, RS1, RS2, ADDR, COMM),
        BLTU(r"{}:.+\s+bltu\s+{},{},{}{}", ADDRESS, RS1, RS2, ADDR, COMM),
        BNE(r"{}:.+\s+bne\s+{},{},{}{}", ADDRESS, RS1, RS2, ADDR, COMM),
        EBREAK(r"{}:.+\s+ebreak{}", ADDRESS, COMM),
        ECALL(r"{}:.+\s+ecall{}", ADDRESS, COMM),
        JAL(r"{}:.+\s+jal\s+{},{}{}", ADDRESS, RD, ADDR, COMM),
        JALR(r"{}:.+\s+jalr\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        JALR_IMPLICIT(r"{}:.+\s+jalr\s+{}\({}\){}", ADDRESS, IMM, RS1, COMM),
        JALR_MORE_IMPLICIT(r"{}:.+\s+jalr\s+{}{}", ADDRESS, RS1, COMM),
        LB(r"{}:.+\s+lb\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        LBU(r"{}:.+\s+lbu\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        LD(r"{}:.+\s+ld\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        LH(r"{}:.+\s+lh\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        LHU(r"{}:.+\s+lhu\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        LUI(r"{}:.+\s+lui\s+{},{}{}", ADDRESS, RD, IMM, COMM),
        LW(r"{}:.+\s+lw\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        LWU(r"{}:.+\s+lwu\s+{},{}\({}\){}", ADDRESS, RD, IMM, RS1, COMM),
        OR(r"{}:.+\s+or\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        ORI(r"{}:.+\s+ori\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SB(r"{}:.+\s+sb\s+{},{}\({}\){}", ADDRESS, RS2, IMM, RS1, COMM),
        SD(r"{}:.+\s+sd\s+{},{}\({}\){}", ADDRESS, RS2, IMM, RS1, COMM),
        SH(r"{}:.+\s+sh\s+{},{}\({}\){}", ADDRESS, RS2, IMM, RS1, COMM),
        SLL(r"{}:.+\s+sll\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SLLW(r"{}:.+\s+sllw\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SLLI(r"{}:.+\s+slli\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SLLIW(r"{}:.+\s+slliw\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SLT(r"{}:.+\s+slt\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SLTI(r"{}:.+\s+slti\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SLTIU(r"{}:.+\s+sltiu\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SLTU(r"{}:.+\s+sltu\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SRA(r"{}:.+\s+sra\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SRAW(r"{}:.+\s+sraw\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SRAI(r"{}:.+\s+srai\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SRAIW(r"{}:.+\s+sraiw\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SRL(r"{}:.+\s+srl\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SRLW(r"{}:.+\s+srlw\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SRLI(r"{}:.+\s+srli\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SRLIW(r"{}:.+\s+srliw\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),
        SUB(r"{}:.+\s+sub\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SUBW(r"{}:.+\s+subw\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        SW(r"{}:.+\s+sw\s+{},{}\({}\){}", ADDRESS, RS2, IMM, RS1, COMM),
        XOR(r"{}:.+\s+xor\s+{},{},{}{}", ADDRESS, RD, RS1, RS2, COMM),
        XORI(r"{}:.+\s+xori\s+{},{},{}{}", ADDRESS, RD, RS1, IMM, COMM),

        // Pseudo
        BEQZ(r"{}:.+\s+beqz\s+{},{}{}", ADDRESS, RS1, ADDR, COMM),
        BNEZ(r"{}:.+\s+bnez\s+{},{}{}", ADDRESS, RS1, ADDR, COMM),
        J(r"{}:.+\s+j\s+{}{}", ADDRESS, ADDR, COMM),
        JR(r"{}:.+\s+jr\s+{}{}", ADDRESS, RS1, COMM),
        LI(r"{}:.+\s+li\s+{},{}{}", ADDRESS, RD, IMM, COMM),
        MV(r"{}:.+\s+mv\s+{},{}{}", ADDRESS, RD, RS1, COMM),
        NEG(r"{}:.+\s+neg\s+{},{}{}", ADDRESS, RD, RS1, COMM),
        NOP(r"{}:.+\s+nop{}", ADDRESS, COMM),
        NOT(r"{}:.+\s+not\s+{},{}{}", ADDRESS, RD, RS1, COMM),
        RET(r"{}:.+\s+ret{}", ADDRESS, COMM),
        SEQZ(r"{}:.+\s+seqz\s+{},{}{}", ADDRESS, RD, RS1, COMM),
        SNEZ(r"{}:.+\s+snez\s+{},{}{}", ADDRESS, RD, RS1, COMM),

        // Misc
        SEXTW(r"{}:.+\s+sext.w\s+{},{}{}", ADDRESS, RD, RS1, COMM),
    }
}
