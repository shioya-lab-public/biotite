pub type RiscvDisassembly = (); // (label, address, inst)

pub enum RiscvInstruction {
    // RV64I
    Add(RiscvRegister, RiscvRegister, RiscvRegister),
    Addw(RiscvRegister, RiscvRegister, RiscvRegister),
    Addi(RiscvRegister, RiscvRegister, RiscvSize),
    Addiw(RiscvRegister, RiscvRegister, RiscvSize),
    And(RiscvRegister, RiscvRegister, RiscvRegister),
    Andi(RiscvRegister, RiscvRegister, RiscvSize),
    Auipc(RiscvRegister, RiscvSize),
    Beq(RiscvRegister, RiscvRegister, RiscvSize),
    Bge(RiscvRegister, RiscvRegister, RiscvSize),
    Bgeu(RiscvRegister, RiscvRegister, RiscvSize),
    Blt(RiscvRegister, RiscvRegister, RiscvSize),
    Bltu(RiscvRegister, RiscvRegister, RiscvSize),
    Bne(RiscvRegister, RiscvRegister, RiscvSize),
    // `csrrc` is not implemented.
    // `csrrci` is not implemented.
    // `csrrs` is not implemented.
    // `csrrsi` is not implemented.
    // `csrrw` is not implemented.
    // `csrrwi` is not implemented.
    // `ebreak` is not implemented.
    // `ecall` is not implemented.
    // `fence` is not implemented.
    // `fence.i` is not implemented.
    Jal(RiscvRegister, RiscvSize),
    Jalr(RiscvRegister, RiscvRegister, RiscvSize),
    Lb(RiscvRegister, RiscvRegister, RiscvSize),
    Lbu(RiscvRegister, RiscvRegister, RiscvSize),
    Ld(RiscvRegister, RiscvRegister, RiscvSize),
    Lh(RiscvRegister, RiscvRegister, RiscvSize),
    Lhu(RiscvRegister, RiscvRegister, RiscvSize),
    Lui(RiscvRegister, RiscvSize),
    Lw(RiscvRegister, RiscvRegister, RiscvSize),
    Lwu(RiscvRegister, RiscvRegister, RiscvSize),
    Or(RiscvRegister, RiscvRegister, RiscvRegister),
    Ori(RiscvRegister, RiscvRegister, RiscvSize),
    Sb(RiscvRegister, RiscvSize, RiscvRegister),
    Sd(RiscvRegister, RiscvSize, RiscvRegister),
    Sh(RiscvRegister, RiscvSize, RiscvRegister),
    Sll(RiscvRegister, RiscvSize, RiscvRegister),
    Sllw(RiscvRegister, RiscvSize, RiscvRegister),
    Slli(RiscvRegister, RiscvRegister, RiscvSize),
    Slliw(RiscvRegister, RiscvRegister, RiscvSize),
    Slt(RiscvRegister, RiscvRegister, RiscvRegister),
    Slti(RiscvRegister, RiscvRegister, RiscvSize),
    Sltiu(RiscvRegister, RiscvRegister, RiscvSize),
    Sltu(RiscvRegister, RiscvRegister, RiscvRegister),
    Sra(RiscvRegister, RiscvSize, RiscvRegister),
    Sraw(RiscvRegister, RiscvSize, RiscvRegister),
    Srai(RiscvRegister, RiscvSize, RiscvSize),
    Sraiw(RiscvRegister, RiscvSize, RiscvSize),
    Srl(RiscvRegister, RiscvRegister, RiscvRegister),
    Srlw(RiscvRegister, RiscvRegister, RiscvRegister),
    Srli(RiscvRegister, RiscvRegister, RiscvSize),
    Srliw(RiscvRegister, RiscvRegister, RiscvSize),
    Sub(RiscvRegister, RiscvRegister, RiscvRegister),
    Subw(RiscvRegister, RiscvRegister, RiscvRegister),
    Sw(RiscvRegister, RiscvSize, RiscvRegister),
    Xor(RiscvRegister, RiscvRegister, RiscvRegister),
    Xori(RiscvRegister, RiscvRegister, RiscvSize),

    // Pseudo
    Beqz(RiscvRegister, RiscvSize),
    Bnez(RiscvRegister, RiscvSize),
    // `fabs.s` is not implemented.
    // `fabs.d` is not implemented.
    // `fmv.s` is not implemented.
    // `fmv.d` is not implemented.
    // `fneg.s` is not implemented.
    // `fneg.d` is not implemented.
    J(RiscvSize),
    Jr(RiscvRegister),
    La(RiscvRegister, String),
    Li(RiscvRegister, RiscvSize),
    Mv(RiscvRegister, RiscvRegister),
    Neg(RiscvRegister, RiscvRegister),
    Nop,
    Not(RiscvRegister, RiscvRegister),
    Ret,
    Seqz(RiscvRegister, RiscvRegister),
    Snez(RiscvRegister, RiscvRegister),
}

pub type RiscvSize = i64;

pub enum RiscvRegister {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    Fp,
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

pub mod riscv_regex {
    use regex::Regex;

    const RD: &str = r"(?P<rd>.+?)";
    const RS1: &str = r"(?P<rs1>.+?)";
    const RS2: &str = r"(?P<rs2>.+?)";
    const IMM: &str = r"(?P<imm>.+?)";

    lazy_static! {
        pub static ref ADD: Regex =
            Regex::new(&format!(r"add\t,(?P<rs1>.+?),(?P<imm>.+)")).unwrap();
    }
}
