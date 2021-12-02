use crate::{
    addr, csr, define_instruction, frd, frs1, frs2, frs3, imm, iorw, ord, rd, rm, rs1, rs2,
};
use regex::{Regex, RegexSet};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub abi: Abi,
    pub code_blocks: Vec<CodeBlock>,
    pub data_blocks: Vec<DataBlock>,
}

#[derive(Debug, PartialEq)]
pub enum Abi {
    Ilp32,
    Ilp32f,
    Ilp32d,
    Lp64,
    Lp64f,
    Lp64d,
}

impl Abi {
    pub fn new(s: &Option<String>) -> Self {
        use Abi::*;

        match s.as_ref().map(|s| s.as_str()) {
            None => Abi::default(),
            Some("ilp32") => Ilp32,
            Some("ilp32f") => Ilp32f,
            Some("ilp32d") => Ilp32d,
            Some("lp64") => Lp64,
            Some("lp64f") => Lp64f,
            Some("lp64d") => Lp64d,
            Some(abi) => panic!("Unknown ABI: `{}`", abi),
        }
    }
}

impl Default for Abi {
    fn default() -> Self {
        Abi::Lp64d
    }
}

#[derive(Debug, PartialEq)]
pub struct CodeBlock {
    pub section: String,
    pub symbol: String,
    pub address: Address,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
pub struct DataBlock {
    pub section: String,
    pub symbol: String,
    pub address: Address,
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Ordering {
    None,
    Aq,
    Rl,
    AqRl,
}

impl Ordering {
    pub fn new(s: &str) -> Self {
        use Ordering::*;

        match s {
            "" => None,
            "aq" => Aq,
            "rl" => Rl,
            "aqrl" => AqRl,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Register {
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

impl Register {
    fn new(s: &str) -> Self {
        use Register::*;

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
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FPRegister {
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

impl FPRegister {
    fn new(s: &str) -> Self {
        use FPRegister::*;

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
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Immediate(pub i64);

impl Immediate {
    pub fn new(s: &str) -> Self {
        Immediate(match s.strip_prefix("0x") {
            Some(s) => i64::from_str_radix(s, 16).unwrap(),
            None => s.parse().unwrap(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Address(pub u64);

impl Address {
    pub fn new(s: &str) -> Self {
        Address(u64::from_str_radix(s, 16).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Csr {
    Fflags,
    Frm,
    Fcsr,
    Cycle,
    Time,
    Instret,
    Cycleh,
    Timeh,
    Instreth,
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
            "cycleh" => Cycleh,
            "timeh" => Timeh,
            "instreth" => Instreth,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rounding {
    Rne,
    Rtz,
    Rdn,
    Rup,
    Rmm,
    Dyn,
}

impl Rounding {
    pub fn new(s: &str) -> Self {
        use Rounding::*;

        match s {
            "rne" => Rne,
            "rtz" => Rtz,
            "rdn" => Rdn,
            "rup" => Rup,
            "rmm" => Rmm,
            "" => Dyn,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Iorw(pub String);

impl Iorw {
    pub fn new(s: &str) -> Self {
        Iorw(s.to_string())
    }
}

define_instruction! {
    // RV32I
    Lui(r"lui\s+{},{}", rd, imm),
    Auipc(r"auipc\s+{},{}", rd, imm),
    Jal(r"jal\s+{},{}", rd, addr),
    Jalr(r"jalr\s+{},{}\({}\)", rd, imm, rs1),
    ImplicitJalr(r"jalr\s+{}\({}\)", imm, rs1), // `rd` is omitted if it is `ra`.
    Beq(r"beq\s+{},{},{}", rs1, rs2, addr),
    Bne(r"bne\s+{},{},{}", rs1, rs2, addr),
    Blt(r"blt\s+{},{},{}", rs1, rs2, addr),
    Bge(r"bge\s+{},{},{}", rs1, rs2, addr),
    Bltu(r"bltu\s+{},{},{}", rs1, rs2, addr),
    Bgeu(r"bgeu\s+{},{},{}", rs1, rs2, addr),
    Lb(r"lb\s+{},{}\({}\)", rd, imm, rs1),
    Lh(r"lh\s+{},{}\({}\)", rd, imm, rs1),
    Lw(r"lw\s+{},{}\({}\)", rd, imm, rs1),
    Lbu(r"lbu\s+{},{}\({}\)", rd, imm, rs1),
    Lhu(r"lhu\s+{},{}\({}\)", rd, imm, rs1),
    Sb(r"sb\s+{},{}\({}\)", rs2, imm, rs1),
    Sh(r"sh\s+{},{}\({}\)", rs2, imm, rs1),
    Sw(r"sw\s+{},{}\({}\)", rs2, imm, rs1),
    Addi(r"addi\s+{},{},{}", rd, rs1, imm),
    Slti(r"slti\s+{},{},{}", rd, rs1, imm),
    Sltiu(r"sltiu\s+{},{},{}", rd, rs1, imm),
    Xori(r"xori\s+{},{},{}", rd, rs1, imm),
    Ori(r"ori\s+{},{},{}", rd, rs1, imm),
    Andi(r"andi\s+{},{},{}", rd, rs1, imm),
    Slli(r"slli\s+{},{},{}", rd, rs1, imm),
    Srli(r"srli\s+{},{},{}", rd, rs1, imm),
    Srai(r"srai\s+{},{},{}", rd, rs1, imm),
    Add(r"add\s+{},{},{}", rd, rs1, rs2),
    Sub(r"sub\s+{},{},{}", rd, rs1, rs2),
    Sll(r"sll\s+{},{},{}", rd, rs1, rs2),
    Slt(r"slt\s+{},{},{}", rd, rs1, rs2),
    Sltu(r"sltu\s+{},{},{}", rd, rs1, rs2),
    Xor(r"xor\s+{},{},{}", rd, rs1, rs2),
    Srl(r"srl\s+{},{},{}", rd, rs1, rs2),
    Sra(r"sra\s+{},{},{}", rd, rs1, rs2),
    Or(r"or\s+{},{},{}", rd, rs1, rs2),
    And(r"and\s+{},{},{}", rd, rs1, rs2),
    Fence(r"fence{}", iorw),
    Ecall(r"ecall"),
    Ebreak(r"ebreak"),

    // RV64I
    Lwu(r"lwu\s+{},{}\({}\)", rd, imm, rs1),
    Ld(r"ld\s+{},{}\({}\)", rd, imm, rs1),
    Sd(r"sd\s+{},{}\({}\)", rs2, imm, rs1),
    // `slli` is the same as RV32I.
    // `srli` is the same as RV32I.
    // `srai` is the same as RV32I.
    Addiw(r"addiw\s+{},{},{}", rd, rs1, imm),
    Slliw(r"slliw\s+{},{},{}", rd, rs1, imm),
    Srliw(r"srliw\s+{},{},{}", rd, rs1, imm),
    Sraiw(r"sraiw\s+{},{},{}", rd, rs1, imm),
    Addw(r"addw\s+{},{},{}", rd, rs1, rs2),
    Subw(r"subw\s+{},{},{}", rd, rs1, rs2),
    Sllw(r"sllw\s+{},{},{}", rd, rs1, rs2),
    Srlw(r"srlw\s+{},{},{}", rd, rs1, rs2),
    Sraw(r"sraw\s+{},{},{}", rd, rs1, rs2),

    // RV32/RV64 Zifencei
    FenceI(r"fence\.i"),

    // RV32/RV64 Zicsr
    Csrrw(r"csrrw\s+{},{},{}", rd, csr, rs1),
    Csrrs(r"csrrs\s+{},{},{}", rd, csr, rs1),
    Csrrc(r"csrrc\s+{},{},{}", rd, csr, rs1),
    Csrrwi(r"csrrwi\s+{},{},{}", rd, csr, imm),
    Csrrsi(r"csrrsi\s+{},{},{}", rd, csr, imm),
    Csrrci(r"csrrci\s+{},{},{}", rd, csr, imm),

    // RV32M
    Mul(r"mul\s+{},{},{}", rd, rs1, rs2),
    Mulh(r"mulh\s+{},{},{}", rd, rs1, rs2),
    Mulhsu(r"mulhsu\s+{},{},{}", rd, rs1, rs2),
    Mulhu(r"mulhu\s+{},{},{}", rd, rs1, rs2),
    Div(r"div\s+{},{},{}", rd, rs1, rs2),
    Divu(r"divu\s+{},{},{}", rd, rs1, rs2),
    Rem(r"rem\s+{},{},{}", rd, rs1, rs2),
    Remu(r"remu\s+{},{},{}", rd, rs1, rs2),

    // RV64M
    Mulw(r"mulw\s+{},{},{}", rd, rs1, rs2),
    Divw(r"divw\s+{},{},{}", rd, rs1, rs2),
    Divuw(r"divuw\s+{},{},{}", rd, rs1, rs2),
    Remw(r"remw\s+{},{},{}", rd, rs1, rs2),
    Remuw(r"remuw\s+{},{},{}", rd, rs1, rs2),

    // RV32A
    LrW(r"lr\.w{}\s+{},\({}\)", ord, rd, rs1),
    ScW(r"sc\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoswapW(r"amoswap\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoaddW(r"amoadd\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoxorW(r"amoxor\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoandW(r"amoand\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoorW(r"amoor\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmominW(r"amomin\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmomaxW(r"amomax\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmominuW(r"amominu\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmomaxuW(r"amomaxu\.w{}\s+{},{},\({}\)", ord, rd, rs2, rs1),

    // RV64A
    LrD(r"lr\.d{}\s+{},\({}\)", ord, rd, rs1),
    ScD(r"sc\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoswapD(r"amoswap\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoaddD(r"amoadd\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoxorD(r"amoxor\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoandD(r"amoand\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmoorD(r"amoor\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmominD(r"amomin\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmomaxD(r"amomax\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmominuD(r"amominu\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),
    AmomaxuD(r"amomaxu\.d{}\s+{},{},\({}\)", ord, rd, rs2, rs1),

    // RV32F
    Flw(r"flw\s+{},{}\({}\)", frd, imm, rs1),
    Fsw(r"fsw\s+{},{}\({}\)", frs2, imm, rs1),
    FmaddS(r"fmadd\.s\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FmsubS(r"fmsub\.s\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FnmsubS(r"fnmsub\.s\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FnmaddS(r"fnmadd\.s\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FaddS(r"fadd\.s\s+{},{},{}{}", frd, frs1, frs2, rm),
    FsubS(r"fsub\.s\s+{},{},{}{}", frd, frs1, frs2, rm),
    FmulS(r"fmul\.s\s+{},{},{}{}", frd, frs1, frs2, rm),
    FdivS(r"fdiv\.s\s+{},{},{}{}", frd, frs1, frs2, rm),
    FsqrtS(r"fsqrt\.s\s+{},{}{}", frd, frs1, rm),
    FsgnjS(r"fsgnj\.s\s+{},{},{}", frd, frs1, frs2),
    FsgnjnS(r"fsgnjn\.s\s+{},{},{}", frd, frs1, frs2),
    FsgnjxS(r"fsgnjx\.s\s+{},{},{}", frd, frs1, frs2),
    FminS(r"fmin\.s\s+{},{},{}", frd, frs1, frs2),
    FmaxS(r"fmax\.s\s+{},{},{}", frd, frs1, frs2),
    FcvtWS(r"fcvt\.w\.s\s+{},{}{}", rd, frs1, rm),
    FcvtWuS(r"fcvt\.wu\.s\s+{},{}{}", rd, frs1, rm),
    FmvXW(r"fmv\.x\.w\s+{},{}", rd, frs1),
    FeqS(r"feq\.s\s+{},{},{}", rd, frs1, frs2),
    FltS(r"flt\.s\s+{},{},{}", rd, frs1, frs2),
    FleS(r"fle\.s\s+{},{},{}", rd, frs1, frs2),
    FclassS(r"fclass\.s\s+{},{}", rd, frs1),
    FcvtSW(r"fcvt\.s\.w\s+{},{}{}", frd, rs1, rm),
    FcvtSWu(r"fcvt\.s\.wu\s+{},{}{}", frd, rs1, rm),
    FmvWX(r"fmv\.w\.x\s+{},{}", frd, rs1),

    // RV64F
    FcvtLS(r"fcvt\.l\.s\s+{},{}{}", rd, frs1, rm),
    FcvtLuS(r"fcvt\.lu\.s\s+{},{}{}", rd, frs1, rm),
    FcvtSL(r"fcvt\.s\.l\s+{},{}{}", frd, rs1, rm),
    FcvtSLu(r"fcvt\.s\.lu\s+{},{}{}", frd, rs1, rm),

    // RV32D
    Fld(r"fld\s+{},{}\({}\)", frd, imm, rs1),
    Fsd(r"fsd\s+{},{}\({}\)", frs2, imm, rs1),
    FmaddD(r"fmadd\.d\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FmsubD(r"fmsub\.d\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FnmsubD(r"fnmsub\.d\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FnmaddD(r"fnmadd\.d\s+{},{},{},{}{}", frd, frs1, frs2, frs3, rm),
    FaddD(r"fadd\.d\s+{},{},{}{}", frd, frs1, frs2, rm),
    FsubD(r"fsub\.d\s+{},{},{}{}", frd, frs1, frs2, rm),
    FmulD(r"fmul\.d\s+{},{},{}{}", frd, frs1, frs2, rm),
    FdivD(r"fdiv\.d\s+{},{},{}{}", frd, frs1, frs2, rm),
    FsqrtD(r"fsqrt\.d\s+{},{}{}", frd, frs1, rm),
    FsgnjD(r"fsgnj\.d\s+{},{},{}", frd, frs1, frs2),
    FsgnjnD(r"fsgnjn\.d\s+{},{},{}", frd, frs1, frs2),
    FsgnjxD(r"fsgnjx\.d\s+{},{},{}", frd, frs1, frs2),
    FminD(r"fmin\.d\s+{},{},{}", frd, frs1, frs2),
    FmaxD(r"fmax\.d\s+{},{},{}", frd, frs1, frs2),
    FcvtSD(r"fcvt\.s\.d\s+{},{}{}", frd, rs1, rm),
    FcvtDS(r"fcvt\.d\.s\s+{},{}{}", rd, frs1, rm),
    FeqD(r"feq\.d\s+{},{},{}", rd, frs1, frs2),
    FltD(r"flt\.d\s+{},{},{}", rd, frs1, frs2),
    FleD(r"fle\.d\s+{},{},{}", rd, frs1, frs2),
    FclassD(r"fclass\.d\s+{},{}", rd, frs1),
    FcvtWD(r"fcvt\.w\.d\s+{},{}{}", rd, frs1, rm),
    FcvtWuD(r"fcvt\.wu\.d\s+{},{}{}", rd, frs1, rm),
    FcvtDW(r"fcvt\.d\.w\s+{},{}{}", frd, rs1, rm),
    FcvtDWu(r"fcvt\.d\.wu\s+{},{}{}", frd, rs1, rm),

    // RV64D
    FcvtLD(r"fcvt\.l\.d\s+{},{}{}", rd, frs1, rm),
    FcvtLuD(r"fcvt\.lu\.d\s+{},{}{}", rd, frs1, rm),
    FmvXD(r"fmv\.x\.d\s+{},{}", rd, frs1),
    FcvtDL(r"fcvt\.d\.l\s+{},{}{}", frd, rs1, rm),
    FcvtDLu(r"fcvt\.d\.lu\s+{},{}{}", frd, rs1, rm),
    FmvDX(r"fmv\.d\.x\s+{},{}", frd, rs1),

    // Pseudoinstructions
    // Pseudoinstructions using symbols are always converted to base instructions.

    Nop(r"nop"),
    Li(r"li\s+{},{}", rd, imm),
    Mv(r"mv\s+{},{}", rd, rs1),
    Not(r"not\s+{},{}", rd, rs1),
    Neg(r"neg\s+{},{}", rd, rs1),
    Negw(r"negw\s+{},{}", rd, rs1),
    SextW(r"sext\.w\s+{},{}", rd, rs1),
    Seqz(r"seqz\s+{},{}", rd, rs1),
    Snez(r"snez\s+{},{}", rd, rs1),
    Sltz(r"sltz\s+{},{}", rd, rs1),
    Sgtz(r"sgtz\s+{},{}", rd, rs1),

    FmvS(r"fmv\.s\s+{},{}", frd, frs1),
    FabsS(r"fabs\.s\s+{},{}", frd, frs1),
    FnegS(r"fneg\.s\s+{},{}", frd, frs1),
    FmvD(r"fmv\.d\s+{},{}", frd, frs1),
    FabsD(r"fabs\.d\s+{},{}", frd, frs1),
    FnegD(r"fneg\.d\s+{},{}", frd, frs1),

    Beqz(r"beqz\s+{},{}", rs1, addr),
    Bnez(r"bnez\s+{},{}", rs1, addr),
    Blez(r"blez\s+{},{}", rs1, addr),
    Bgez(r"bgez\s+{},{}", rs1, addr),
    Bltz(r"bltz\s+{},{}", rs1, addr),
    Bgtz(r"bgtz\s+{},{}", rs1, addr),

    // `bgt` is always converted to `blt`.
    // `ble` is always converted to `bge`.
    // `bgtu` is always converted to `bltu`.
    // `bleu` is always converted to `bgeu`.

    J(r"j\s+{}", addr),
    // `jal offset` is always converted to `jal ra,addr`.
    Jr(r"jr\s+{}", rs1),
    PseudoJalr(r"jalr\s+{}", rs1),
    Ret(r"ret"),
    // `call offset` is always converted to the base instruction.
    // `tail offset` is always converted to the base instruction.

    PseudoFence(r"fence"),

    Rdinstret(r"rdinstret\s+{}", rd),
    Rdinstreth(r"rdinstreth\s+{}", rd),
    Rdcycle(r"rdcycle\s+{}", rd),
    Rdcycleh(r"rdcycleh\s+{}", rd),
    Rdtime(r"rdtime\s+{}", rd),
    Rdtimeh(r"rdtimeh\s+{}", rd),

    Csrr(r"csrr\s+{},{}", rd, csr),
    Csrw(r"csrw\s+{},{}", csr, rs1),
    Csrs(r"csrs\s+{},{}", csr, rs1),
    Csrc(r"csrc\s+{},{}", csr, rs1),

    Csrwi(r"csrwi\s+{},{}", csr, imm),
    Csrsi(r"csrsi\s+{},{}", csr, imm),
    Csrci(r"csrci\s+{},{}", csr, imm),

    Frcsr(r"frcsr\s+{}", rd),
    Fscsr(r"fscsr\s+{},{}", rd, rs1),
    Fwcsr(r"fscsr\s+{}", rs1), // `fscsr rs` is renamed to `fwcsr` to avoid conflicts.

    Frrm(r"frrm\s+{}", rd),
    Fsrm(r"fsrm\s+{},{}", rd, rs1),
    Fwrm(r"fsrm\s+{}", rs1), // `fsrm rs` is renamed to `fwrm` to avoid conflicts.

    Frflags(r"frflags\s+{}", rd),
    Fsflags(r"fsflags\s+{},{}", rd, rs1),
    Fwflags(r"fsflags\s+{}", rs1), // `fsflags rs` is renamed to `fwflags` to avoid conflicts.
}
