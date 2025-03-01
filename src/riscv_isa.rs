use crate::riscv_macro::*;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prog {
    pub entry: Addr,
    pub data_blocks: Vec<DataBlock>,
    pub code_blocks: Vec<CodeBlock>,
    pub tdata: Option<(Addr, usize)>,
    pub func_syms: HashSet<Addr>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DataBlock {
    pub address: Addr,
    pub section: String,
    pub symbol: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CodeBlock {
    pub address: Addr,
    pub section: String,
    pub symbol: String,
    pub insts: Vec<Inst>,
}

define_insts! {
    // RV32I
    Lui(r"lui\s+{},\s+{}", rd, imm),
    Auipc(r"auipc\s+{},\s+{}", rd, imm),
    Jal(r"jal\s+{},\s+{}", rd, addr),
    Jalr(r"jalr\s+{},\s+{}\({}\)", rd, imm, rs1),
    Beq(r"beq\s+{},\s+{},\s+{}", rs1, rs2, addr),
    Bne(r"bne\s+{},\s+{},\s+{}", rs1, rs2, addr),
    Blt(r"blt\s+{},\s+{},\s+{}", rs1, rs2, addr),
    Bge(r"bge\s+{},\s+{},\s+{}", rs1, rs2, addr),
    Bltu(r"bltu\s+{},\s+{},\s+{}", rs1, rs2, addr),
    Bgeu(r"bgeu\s+{},\s+{},\s+{}", rs1, rs2, addr),
    Lb(r"lb\s+{},\s+{}\({}\)", rd, imm, rs1),
    Lh(r"lh\s+{},\s+{}\({}\)", rd, imm, rs1),
    Lw(r"lw\s+{},\s+{}\({}\)", rd, imm, rs1),
    Lbu(r"lbu\s+{},\s+{}\({}\)", rd, imm, rs1),
    Lhu(r"lhu\s+{},\s+{}\({}\)", rd, imm, rs1),
    Sb(r"sb\s+{},\s+{}\({}\)", rs2, imm, rs1),
    Sh(r"sh\s+{},\s+{}\({}\)", rs2, imm, rs1),
    Sw(r"sw\s+{},\s+{}\({}\)", rs2, imm, rs1),
    Addi(r"addi\s+{},\s+{},\s+{}", rd, rs1, imm),
    Slti(r"slti\s+{},\s+{},\s+{}", rd, rs1, imm),
    Sltiu(r"sltiu\s+{},\s+{},\s+{}", rd, rs1, imm),
    Xori(r"xori\s+{},\s+{},\s+{}", rd, rs1, imm),
    Ori(r"ori\s+{},\s+{},\s+{}", rd, rs1, imm),
    Andi(r"andi\s+{},\s+{},\s+{}", rd, rs1, imm),
    Slli(r"slli\s+{},\s+{},\s+{}", rd, rs1, imm),
    Srli(r"srli\s+{},\s+{},\s+{}", rd, rs1, imm),
    Srai(r"srai\s+{},\s+{},\s+{}", rd, rs1, imm),
    Add(r"add\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Sub(r"sub\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Sll(r"sll\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Slt(r"slt\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Sltu(r"sltu\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Xor(r"xor\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Srl(r"srl\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Sra(r"sra\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Or(r"or\s+{},\s+{},\s+{}", rd, rs1, rs2),
    And(r"and\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Fence(r"fence\s+\S+|fence\.tso"),
    Ecall(r"ecall"),
    Ebreak(r"ebreak"),

    // RV64I (in addition to RV32I)
    Lwu(r"lwu\s+{},\s+{}\({}\)", rd, imm, rs1),
    Ld(r"ld\s+{},\s+{}\({}\)", rd, imm, rs1),
    Sd(r"sd\s+{},\s+{}\({}\)", rs2, imm, rs1),
    // `slli` is the same as its RV32I version
    // `srli` is the same as its RV32I version
    // `srai` is the same as its RV32I version
    Addiw(r"addiw\s+{},\s+{},\s+{}", rd, rs1, imm),
    Slliw(r"slliw\s+{},\s+{},\s+{}", rd, rs1, imm),
    Srliw(r"srliw\s+{},\s+{},\s+{}", rd, rs1, imm),
    Sraiw(r"sraiw\s+{},\s+{},\s+{}", rd, rs1, imm),
    Addw(r"addw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Subw(r"subw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Sllw(r"sllw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Srlw(r"srlw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Sraw(r"sraw\s+{},\s+{},\s+{}", rd, rs1, rs2),

    // RV32/RV64 Zifencei
    FenceI(r"fence\.i"),

    // RV32/RV64 Zicsr
    Csrrw(r"csrrw\s+{},\s+{},\s+{}", rd, csr, rs1),
    Csrrs(r"csrrs\s+{},\s+{},\s+{}", rd, csr, rs1),
    Csrrc(r"csrrc\s+{},\s+{},\s+{}", rd, csr, rs1),
    Csrrwi(r"csrrwi\s+{},\s+{},\s+{}", rd, csr, imm),
    Csrrsi(r"csrrsi\s+{},\s+{},\s+{}", rd, csr, imm),
    Csrrci(r"csrrci\s+{},\s+{},\s+{}", rd, csr, imm),

    // RV32M
    Mul(r"mul\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Mulh(r"mulh\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Mulhsu(r"mulhsu\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Mulhu(r"mulhu\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Div(r"div\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Divu(r"divu\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Rem(r"rem\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Remu(r"remu\s+{},\s+{},\s+{}", rd, rs1, rs2),

    // RV64M (in addition to RV32M)
    Mulw(r"mulw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Divw(r"divw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Divuw(r"divuw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Remw(r"remw\s+{},\s+{},\s+{}", rd, rs1, rs2),
    Remuw(r"remuw\s+{},\s+{},\s+{}", rd, rs1, rs2),

    // RV32A
    LrW(r"lr\.w{}\s+{},\s+\({}\)", mo, rd, rs1),
    ScW(r"sc\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoswapW(r"amoswap\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoaddW(r"amoadd\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoxorW(r"amoxor\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoandW(r"amoand\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoorW(r"amoor\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmominW(r"amomin\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmomaxW(r"amomax\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmominuW(r"amominu\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmomaxuW(r"amomaxu\.w{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),

    // RV64A (in addition to RV32A)
    LrD(r"lr\.d{}\s+{},\s+\({}\)", mo, rd, rs1),
    ScD(r"sc\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoswapD(r"amoswap\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoaddD(r"amoadd\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoxorD(r"amoxor\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoandD(r"amoand\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmoorD(r"amoor\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmominD(r"amomin\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmomaxD(r"amomax\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmominuD(r"amominu\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),
    AmomaxuD(r"amomaxu\.d{}\s+{},\s+{},\s+\({}\)", mo, rd, rs2, rs1),

    // RV32F
    Flw(r"flw\s+{},\s+{}\({}\)", frd, imm, rs1),
    Fsw(r"fsw\s+{},\s+{}\({}\)", frs2, imm, rs1),
    FmaddS(r"fmadd\.s\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FmsubS(r"fmsub\.s\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FnmsubS(r"fnmsub\.s\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FnmaddS(r"fnmadd\.s\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FaddS(r"fadd\.s\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FsubS(r"fsub\.s\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FmulS(r"fmul\.s\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FdivS(r"fdiv\.s\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FsqrtS(r"fsqrt\.s\s+{},\s+{}{}", frd, frs1, rm),
    FsgnjS(r"fsgnj\.s\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FsgnjnS(r"fsgnjn\.s\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FsgnjxS(r"fsgnjx\.s\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FminS(r"fmin\.s\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FmaxS(r"fmax\.s\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FcvtWS(r"fcvt\.w\.s\s+{},\s+{}{}", rd, frs1, rm),
    FcvtWuS(r"fcvt\.wu\.s\s+{},\s+{}{}", rd, frs1, rm),
    FmvXW(r"fmv\.x\.w\s+{},\s+{}", rd, frs1),
    FeqS(r"feq\.s\s+{},\s+{},\s+{}", rd, frs1, frs2),
    FltS(r"flt\.s\s+{},\s+{},\s+{}", rd, frs1, frs2),
    FleS(r"fle\.s\s+{},\s+{},\s+{}", rd, frs1, frs2),
    FclassS(r"fclass\.s\s+{},\s+{}", rd, frs1),
    FcvtSW(r"fcvt\.s\.w\s+{},\s+{}{}", frd, rs1, rm),
    FcvtSWu(r"fcvt\.s\.wu\s+{},\s+{}{}", frd, rs1, rm),
    FmvWX(r"fmv\.w\.x\s+{},\s+{}", frd, rs1),

    // RV64F (in addition to RV32F)
    FcvtLS(r"fcvt\.l\.s\s+{},\s+{}{}", rd, frs1, rm),
    FcvtLuS(r"fcvt\.lu\.s\s+{},\s+{}{}", rd, frs1, rm),
    FcvtSL(r"fcvt\.s\.l\s+{},\s+{}{}", frd, rs1, rm),
    FcvtSLu(r"fcvt\.s\.lu\s+{},\s+{}{}", frd, rs1, rm),

    // RV32D
    Fld(r"fld\s+{},\s+{}\({}\)", frd, imm, rs1),
    Fsd(r"fsd\s+{},\s+{}\({}\)", frs2, imm, rs1),
    FmaddD(r"fmadd\.d\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FmsubD(r"fmsub\.d\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FnmsubD(r"fnmsub\.d\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FnmaddD(r"fnmadd\.d\s+{},\s+{},\s+{},\s+{}{}", frd, frs1, frs2, frs3, rm),
    FaddD(r"fadd\.d\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FsubD(r"fsub\.d\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FmulD(r"fmul\.d\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FdivD(r"fdiv\.d\s+{},\s+{},\s+{}{}", frd, frs1, frs2, rm),
    FsqrtD(r"fsqrt\.d\s+{},\s+{}{}", frd, frs1, rm),
    FsgnjD(r"fsgnj\.d\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FsgnjnD(r"fsgnjn\.d\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FsgnjxD(r"fsgnjx\.d\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FminD(r"fmin\.d\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FmaxD(r"fmax\.d\s+{},\s+{},\s+{}", frd, frs1, frs2),
    FcvtSD(r"fcvt\.s\.d\s+{},\s+{}{}", frd, frs1, rm),
    FcvtDS(r"fcvt\.d\.s\s+{},\s+{}{}", frd, frs1, rm),
    FeqD(r"feq\.d\s+{},\s+{},\s+{}", rd, frs1, frs2),
    FltD(r"flt\.d\s+{},\s+{},\s+{}", rd, frs1, frs2),
    FleD(r"fle\.d\s+{},\s+{},\s+{}", rd, frs1, frs2),
    FclassD(r"fclass\.d\s+{},\s+{}", rd, frs1),
    FcvtWD(r"fcvt\.w\.d\s+{},\s+{}{}", rd, frs1, rm),
    FcvtWuD(r"fcvt\.wu\.d\s+{},\s+{}{}", rd, frs1, rm),
    FcvtDW(r"fcvt\.d\.w\s+{},\s+{}{}", frd, rs1, rm),
    FcvtDWu(r"fcvt\.d\.wu\s+{},\s+{}{}", frd, rs1, rm),

    // RV64D (in addition to RV32D)
    FcvtLD(r"fcvt\.l\.d\s+{},\s+{}{}", rd, frs1, rm),
    FcvtLuD(r"fcvt\.lu\.d\s+{},\s+{}{}", rd, frs1, rm),
    FmvXD(r"fmv\.x\.d\s+{},\s+{}", rd, frs1),
    FcvtDL(r"fcvt\.d\.l\s+{},\s+{}{}", frd, rs1, rm),
    FcvtDLu(r"fcvt\.d\.lu\s+{},\s+{}{}", frd, rs1, rm),
    FmvDX(r"fmv\.d\.x\s+{},\s+{}", frd, rs1),

    // Pseudoinstructions

    // Pseudoinstructions using symbols are compiled to base instructions

    Nop(r"nop"),
    Li(r"li\s+{},\s+{}", rd, imm),
    Mv(r"mv\s+{},\s+{}", rd, rs1),
    Not(r"not\s+{},\s+{}", rd, rs1),
    Neg(r"neg\s+{},\s+{}", rd, rs1),
    Negw(r"negw\s+{},\s+{}", rd, rs1),
    SextB(r"sext\.b\s+{},\s+{}", rd, rs1),
    SextH(r"sext\.h\s+{},\s+{}", rd, rs1),
    SextW(r"sext\.w\s+{},\s+{}", rd, rs1),
    ZextB(r"zext\.b\s+{},\s+{}", rd, rs1),
    ZextH(r"zext\.h\s+{},\s+{}", rd, rs1),
    ZextW(r"zext\.w\s+{},\s+{}", rd, rs1),
    Seqz(r"seqz\s+{},\s+{}", rd, rs1),
    Snez(r"snez\s+{},\s+{}", rd, rs1),
    Sltz(r"sltz\s+{},\s+{}", rd, rs1),
    Sgtz(r"sgtz\s+{},\s+{}", rd, rs1),

    FmvS(r"fmv\.s\s+{},\s+{}", frd, frs1),
    FabsS(r"fabs\.s\s+{},\s+{}", frd, frs1),
    FnegS(r"fneg\.s\s+{},\s+{}", frd, frs1),
    FmvD(r"fmv\.d\s+{},\s+{}", frd, frs1),
    FabsD(r"fabs\.d\s+{},\s+{}", frd, frs1),
    FnegD(r"fneg\.d\s+{},\s+{}", frd, frs1),

    Beqz(r"beqz\s+{},\s+{}", rs1, addr),
    Bnez(r"bnez\s+{},\s+{}", rs1, addr),
    Blez(r"blez\s+{},\s+{}", rs1, addr),
    Bgez(r"bgez\s+{},\s+{}", rs1, addr),
    Bltz(r"bltz\s+{},\s+{}", rs1, addr),
    Bgtz(r"bgtz\s+{},\s+{}", rs1, addr),

    // `bgt` is compiled to base instructions
    // `ble` is compiled to base instructions
    // `bgtu` is compiled to base instructions
    // `bleu` is compiled to base instructions

    J(r"j\s+{}", addr),
    PseudoJal(r"jal\s+{}", addr),
    Jr(r"jr\s+{}", rs1),
    PseudoJalr(r"jalr\s+{}", rs1),
    Ret(r"ret"),
    // `call` is compiled to base instructions
    // `tail` is compiled to base instructions

    PseudoFence(r"fence"),

    Rdinstret(r"rdinstret\s+{}", rd),
    // `rdinstreth` is for RV32I only
    Rdcycle(r"rdcycle\s+{}", rd),
    // `rdcycleh` is for RV32I only
    Rdtime(r"rdtime\s+{}", rd),
    // `rdtimeh` is for RV32I only

    Csrr(r"csrr\s+{},\s+{}", rd, csr),
    Csrw(r"csrw\s+{},\s+{}", csr, rs1),
    Csrs(r"csrs\s+{},\s+{}", csr, rs1),
    Csrc(r"csrc\s+{},\s+{}", csr, rs1),

    Csrwi(r"csrwi\s+{},\s+{}", csr, imm),
    Csrsi(r"csrsi\s+{},\s+{}", csr, imm),
    Csrci(r"csrci\s+{},\s+{}", csr, imm),

    Frcsr(r"frcsr\s+{}", rd),
    Fscsr(r"fscsr\s+{},\s+{}", rd, rs1),
    PseudoFscsr(r"fscsr\s+{}", rs1),

    Frrm(r"frrm\s+{}", rd),
    Fsrm(r"fsrm\s+{},\s+{}", rd, rs1),
    PseudoFsrm(r"fsrm\s+{}", rs1),
    Fsrmi(r"fsrmi\s+{},\s+{}", rd, imm),
    PseudoFsrmi(r"fsrmi\s+{}", imm),

    Frflags(r"frflags\s+{}", rd),
    Fsflags(r"fsflags\s+{},\s+{}", rd, rs1),
    PseudoFsflags(r"fsflags\s+{}", rs1),
    Fsflagsi(r"fsflagsi\s+{},\s+{}", rd, imm),
    PseudoFsflagsi(r"fsflagsi\s+{}", imm),

    // Misc
    Unimp(r"unimp"),
    OffsetJalr(r"jalr\s+{}\({}\)", imm, rs1),
    OffsetJr(r"jr\s+{}\({}\)", imm, rs1),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Reg {
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

impl Reg {
    fn new(s: &str) -> Self {
        use Reg::*;

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
            s => panic!("Unknown register `{s}`"),
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Reg::*;

        match self {
            Zero => write!(f, "zero"),
            Ra => write!(f, "ra"),
            Sp => write!(f, "sp"),
            Gp => write!(f, "gp"),
            Tp => write!(f, "tp"),
            T0 => write!(f, "t0"),
            T1 => write!(f, "t1"),
            T2 => write!(f, "t2"),
            S0 => write!(f, "s0"),
            S1 => write!(f, "s1"),
            A0 => write!(f, "a0"),
            A1 => write!(f, "a1"),
            A2 => write!(f, "a2"),
            A3 => write!(f, "a3"),
            A4 => write!(f, "a4"),
            A5 => write!(f, "a5"),
            A6 => write!(f, "a6"),
            A7 => write!(f, "a7"),
            S2 => write!(f, "s2"),
            S3 => write!(f, "s3"),
            S4 => write!(f, "s4"),
            S5 => write!(f, "s5"),
            S6 => write!(f, "s6"),
            S7 => write!(f, "s7"),
            S8 => write!(f, "s8"),
            S9 => write!(f, "s9"),
            S10 => write!(f, "s10"),
            S11 => write!(f, "s11"),
            T3 => write!(f, "t3"),
            T4 => write!(f, "t4"),
            T5 => write!(f, "t5"),
            T6 => write!(f, "t6"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum FReg {
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

impl FReg {
    fn new(s: &str) -> Self {
        use FReg::*;

        match s {
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
            s => panic!("Unknown FP register `{s}`"),
        }
    }
}

impl Display for FReg {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use FReg::*;

        match self {
            Ft0 => write!(f, "ft0"),
            Ft1 => write!(f, "ft1"),
            Ft2 => write!(f, "ft2"),
            Ft3 => write!(f, "ft3"),
            Ft4 => write!(f, "ft4"),
            Ft5 => write!(f, "ft5"),
            Ft6 => write!(f, "ft6"),
            Ft7 => write!(f, "ft7"),
            Fs0 => write!(f, "fs0"),
            Fs1 => write!(f, "fs1"),
            Fa0 => write!(f, "fa0"),
            Fa1 => write!(f, "fa1"),
            Fa2 => write!(f, "fa2"),
            Fa3 => write!(f, "fa3"),
            Fa4 => write!(f, "fa4"),
            Fa5 => write!(f, "fa5"),
            Fa6 => write!(f, "fa6"),
            Fa7 => write!(f, "fa7"),
            Fs2 => write!(f, "fs2"),
            Fs3 => write!(f, "fs3"),
            Fs4 => write!(f, "fs4"),
            Fs5 => write!(f, "fs5"),
            Fs6 => write!(f, "fs6"),
            Fs7 => write!(f, "fs7"),
            Fs8 => write!(f, "fs8"),
            Fs9 => write!(f, "fs9"),
            Fs10 => write!(f, "fs10"),
            Fs11 => write!(f, "fs11"),
            Ft8 => write!(f, "ft8"),
            Ft9 => write!(f, "ft9"),
            Ft10 => write!(f, "ft10"),
            Ft11 => write!(f, "ft11"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Imm(pub i64);

impl Imm {
    pub fn new(s: &str) -> Self {
        Imm(s
            .parse()
            .unwrap_or_else(|_| panic!("Invalid immediate `{s}`")))
    }
}

impl Display for Imm {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Imm(imm) = self;
        write!(f, "{imm}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Addr(pub u64);

impl Addr {
    pub fn new(s: &str) -> Self {
        Addr(u64::from_str_radix(s, 16).unwrap_or_else(|_| panic!("Invalid address `{s}`")))
    }
}

impl Display for Addr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Addr(addr) = self;
        write!(f, "0x{addr:x}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Csr {
    Fflags,
    Frm,
    Fcsr,
    Cycle,
    Time,
    Instret,
    // `Cycleh` is for RV32I only
    // `Timeh` is for RV32I only
    // `Instreth` is for RV32I only
    Unknown,
}

impl Csr {
    pub fn new(s: &str) -> Self {
        use Csr::*;

        match s {
            "fflags" => Fflags,
            "frm" => Frm,
            "fcsr" => Fcsr,
            "cycle" => Cycle,
            "time" => Time,
            "instret" => Instret,
            _ => Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Mo {
    Mono,
    Aq,
    Rl,
    AqRl,
}

impl Mo {
    pub fn new(s: &str) -> Self {
        use Mo::*;

        match s {
            "" => Mono,
            "aq" => Aq,
            "rl" => Rl,
            "aqrl" => AqRl,
            s => panic!("Unknown MO `{s}`"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Rm {
    Rne,
    Rtz,
    Rdn,
    Rup,
    Rmm,
    Dyn,
}

impl Rm {
    pub fn new(s: &str) -> Self {
        use Rm::*;

        match s {
            "rne" => Rne,
            "rtz" => Rtz,
            "rdn" => Rdn,
            "rup" => Rup,
            "rmm" => Rmm,
            "" => Dyn,
            s => panic!("Unknown RM `{s}`"),
        }
    }
}
