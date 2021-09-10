use crate::{addr, define_instruction, imm, rd, rs1, rs2};
use regex::{Regex, RegexSet};

#[derive(Debug, PartialEq)]
pub enum RiscvRegister {
    // Integer
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

    // Floating-Point
    Ft0,
    Ft1,
    Ft2,
    Ft3,
    Ft4,
    Ft5,
    Ft6,
    Ft7,
    Fs0,
    Fs1,
    Fa0,
    Fa1,
    Fa2,
    Fa3,
    Fa4,
    Fa5,
    Fa6,
    Fa7,
    Fs2,
    Fs3,
    Fs4,
    Fs5,
    Fs6,
    Fs7,
    Fs8,
    Fs9,
    Fs10,
    Fs11,
    Ft8,
    Ft9,
    Ft10,
    Ft11,
}

impl RiscvRegister {
    fn new(s: &str) -> Self {
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

            "ft0" => Ft0,
            "ft1" => Ft1,
            "ft2" => Ft2,
            "ft3" => Ft3,
            "ft4" => Ft4,
            "ft5" => Ft5,
            "ft6" => Ft6,
            "ft7" => Ft7,
            "fs0" => Fs0,
            "fs1" => Fs1,
            "fa0" => Fa0,
            "fa1" => Fa1,
            "fa2" => Fa2,
            "fa3" => Fa3,
            "fa4" => Fa4,
            "fa5" => Fa5,
            "fa6" => Fa6,
            "fa7" => Fa7,
            "fs2" => Fs2,
            "fs3" => Fs3,
            "fs4" => Fs4,
            "fs5" => Fs5,
            "fs6" => Fs6,
            "fs7" => Fs7,
            "fs8" => Fs8,
            "fs9" => Fs9,
            "fs10" => Fs10,
            "fs11" => Fs11,
            "ft8" => Ft8,
            "ft9" => Ft9,
            "ft10" => Ft10,
            "ft11" => Ft11,

            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RiscvImmediate(i64);

impl RiscvImmediate {
    pub fn new(s: &str) -> Self {
        let imm = match s.strip_prefix("0x") {
            Some(s) => i64::from_str_radix(s, 16).unwrap(),
            None => s.parse().unwrap(),
        };
        RiscvImmediate(imm)
    }
}

impl From<i64> for RiscvImmediate {
    fn from(imm: i64) -> Self {
        RiscvImmediate::new(&imm.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RiscvAddress(usize);

impl RiscvAddress {
    pub fn new(s: &str) -> Self {
        RiscvAddress(usize::from_str_radix(s, 16).unwrap())
    }
}

impl From<usize> for RiscvAddress {
    fn from(imm: usize) -> Self {
        RiscvAddress(imm)
    }
}

define_instruction! {
    // RV32I
    Lui("lui", "{},{}", rd, imm),
    Auipc("auipc", "{},{}", rd, imm),
    Jal("jal", "{},{}", rd, addr),
    Jalr("jalr", "{},{}\\({}\\)", rd, imm, rs1),
    Beq("beq", "{},{},{}", rs1, rs2, addr),
    Bne("bne", "{},{},{}", rs1, rs2, addr),
    Blt("blt", "{},{},{}", rs1, rs2, addr),
    Bge("bge", "{},{},{}", rs1, rs2, addr),
    Bltu("bltu", "{},{},{}", rs1, rs2, addr),
    Bgeu("bgeu", "{},{},{}", rs1, rs2, addr),
    Lb("lb", "{},{}\\({}\\)", rd, imm, rs1),
    Lh("lh", "{},{}\\({}\\)", rd, imm, rs1),
    Lw("lw", "{},{}\\({}\\)", rd, imm, rs1),
    Lbu("lbu", "{},{}\\({}\\)", rd, imm, rs1),
    Lhu("lhu", "{},{}\\({}\\)", rd, imm, rs1),
    Sb("sb", "{},{}\\({}\\)", rs2, imm, rs1),
    Sh("sh", "{},{}\\({}\\)", rs2, imm, rs1),
    Sw("sw", "{},{}\\({}\\)", rs2, imm, rs1),
    Addi("addi", "{},{},{}", rd, rs1, imm),
    Slti("slti", "{},{},{}", rd, rs1, imm),
    Sltiu("sltiu", "{},{},{}", rd, rs1, imm),
    Xori("xori", "{},{},{}", rd, rs1, imm),
    Ori("ori", "{},{},{}", rd, rs1, imm),
    Andi("andi", "{},{},{}", rd, rs1, imm),
    Slli("slli", "{},{},{}", rd, rs1, imm),
    Srli("srli", "{},{},{}", rd, rs1, imm),
    Srai("srai", "{},{},{}", rd, rs1, imm),
    Add("add", "{},{},{}", rd, rs1, rs2),
    Sub("sub", "{},{},{}", rd, rs1, rs2),
    Sll("sll", "{},{},{}", rd, rs1, rs2),
    Slt("slt", "{},{},{}", rd, rs1, rs2),
    Sltu("sltu", "{},{},{}", rd, rs1, rs2),
    Xor("xor", "{},{},{}", rd, rs1, rs2),
    Srl("srl", "{},{},{}", rd, rs1, rs2),
    Sra("sra", "{},{},{}", rd, rs1, rs2),
    Or("or", "{},{},{}", rd, rs1, rs2),
    And("and", "{},{},{}", rd, rs1, rs2),
    Fence("fence", ".*"), // LLVM only supports `fence` in its most general form like this.
    Ecall("ecall", ""),
    Ebreak("ebreak", ""),

    // RV64I
    Lwu("lwu", "{},{}\\({}\\)", rd, imm, rs1),
    Ld("ld", "{},{}\\({}\\)", rd, imm, rs1),
    Sd("sd", "{},{}\\({}\\)", rs2, imm, rs1),
    Addiw("addiw", "{},{},{}", rd, rs1, imm),
    Slliw("slliw", "{},{},{}", rd, rs1, imm),
    Srliw("srliw", "{},{},{}", rd, rs1, imm),
    Sraiw("sraiw", "{},{},{}", rd, rs1, imm),
    Addw("addw", "{},{},{}", rd, rs1, rs2),
    Subw("subw", "{},{},{}", rd, rs1, rs2),
    Sllw("sllw", "{},{},{}", rd, rs1, rs2),
    Srlw("srlw", "{},{},{}", rd, rs1, rs2),
    Sraw("sraw", "{},{},{}", rd, rs1, rs2),

    // RV32F
    Flw("flw", "{},{}\\({}\\)", rd, imm, rs1),
}
