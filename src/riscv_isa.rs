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

pub type RiscvAddress = usize;

define_instruction! {
    Lui("lui", "{},{}", rd, imm),
    Jalr("jalr", "{},{}\\({}\\)", rd, imm, rs1),
    Ecall("ecall", ""),
}
