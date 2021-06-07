pub enum RiscvInstruction {
    Addi(RiscvRegister, i64, RiscvRegister),
    Sd(RiscvRegister, i64, RiscvRegister),
    Li(RiscvRegister, i64),
    Mv(RiscvRegister, RiscvRegister),
    Ld(RiscvRegister, i64, RiscvRegister),
    Ret,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum RiscvRegister {
    Sp,
    S0,
    A5,
    A0,
}

pub mod riscv_instruction_regex {
    use regex::Regex;

    lazy_static! {
        pub static ref ADDI: Regex =
            Regex::new(r"addi\t(?P<rd>.+?),(?P<rs1>.+?),(?P<imm>.+)").unwrap();
        pub static ref SD: Regex =
            Regex::new(r"sd\t(?P<rs2>.+?),(?P<imm>.+)\((?P<rs1>.+)\)").unwrap();
        pub static ref LI: Regex = Regex::new(r"li\t(?P<rd>.+?),(?P<imm>.+)").unwrap();
        pub static ref MV: Regex = Regex::new(r"mv\t(?P<rd>.+?),(?P<rs1>.+)").unwrap();
        pub static ref LD: Regex =
            Regex::new(r"ld\t(?P<rd>.+?),(?P<imm>.+)\((?P<rs1>.+)\)").unwrap();
        pub static ref RET: Regex = Regex::new(r"ret").unwrap();
    }
}
