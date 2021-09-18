use crate::{addr, define_instruction, imm, ord, rd, rs1, rs2, rs3};
use regex::{Regex, RegexSet};

#[derive(Debug, PartialEq, Clone)]
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

            // _ => unreachable!(),
            s => {
                panic!("### {}", s);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum RiscvOrdering {
    Empty,
    Aq,
    Rl,
    Aqrl,
}

impl RiscvOrdering {
    pub fn new(s: &str) -> Self {
        use RiscvOrdering::*;

        match s {
            s if s.trim().is_empty() => Empty,
            ".aq" => Aq,
            ".rl" => Rl,
            ".aqrl" => Aqrl,
            s => panic!("Unknown ordering: {}", s),
        }
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
    Fence("fence(\\.tso)?", "\\S*"), // LLVM only supports `fence` in its most general form like this.
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

    // RV32M
    Mul("mul", "{},{},{}", rd, rs1, rs2),
    Mulh("mulh", "{},{},{}", rd, rs1, rs2),
    Mulhsu("mulhsu", "{},{},{}", rd, rs1, rs2),
    Mulhu("mulhu", "{},{},{}", rd, rs1, rs2),
    Div("div", "{},{},{}", rd, rs1, rs2),
    Divu("divu", "{},{},{}", rd, rs1, rs2),
    Rem("rem", "{},{},{}", rd, rs1, rs2),
    Remu("remu", "{},{},{}", rd, rs1, rs2),

    // RV64M
    Mulw("mulw", "{},{},{}", rd, rs1, rs2),
    Divw("divw", "{},{},{}", rd, rs1, rs2),
    Divuw("divuw", "{},{},{}", rd, rs1, rs2),
    Remw("remw", "{},{},{}", rd, rs1, rs2),
    Remuw("remuw", "{},{},{}", rd, rs1, rs2),

    // RV32A
    LrW("lr\\.w{}", "{},\\({}\\)", ord, rd, rs1),
    ScW("sc\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoswapW("amoswap\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoaddW("amoadd\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoxorW("amoxor\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoandW("amoand\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoorW("amoor\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmominW("amomin\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmomaxW("amomax\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmominuW("amominu\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmomaxuW("amomaxu\\.w{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),

    // RV64A
    LrD("lr\\.d{}", "{},\\({}\\)", ord, rd, rs1),
    ScD("sc\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoswapD("amoswap\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoaddD("amoadd\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoxorD("amoxor\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoandD("amoand\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmoorD("amoor\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmominD("amomin\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmomaxD("amomax\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmominuD("amominu\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),
    AmomaxuD("amomaxu\\.d{}", "{},{},\\({}\\)", ord, rd, rs2, rs1),

    // RV32F (Rounding modes are ignored.)
    Flw("flw", "{},{}\\({}\\)", rd, imm, rs1),
    Fsw("fsw", "{},{}\\({}\\)", rs2, imm, rs1),
    FmaddS("fmadd\\.s", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FmsubS("fmsub\\.s", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FnmsubS("fnmsub\\.s", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FnmaddS("fnmadd\\.s", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FaddS("fadd\\.s", "{},{},{}\\S*", rd, rs1, rs2),
    FsubS("fsub\\.s", "{},{},{}\\S*", rd, rs1, rs2),
    FmulS("fmul\\.s", "{},{},{}\\S*", rd, rs1, rs2),
    FdivS("fdiv\\.s", "{},{},{}\\S*", rd, rs1, rs2),
    FsqrtS("fsqrt\\.s", "{},{}\\S*", rd, rs1),
    FsgnjS("fsgnj\\.s", "{},{},{}", rd, rs1, rs2),
    FsgnjnS("fsgnjn\\.s", "{},{},{}", rd, rs1, rs2),
    FsgnjxS("fsgnjx\\.s", "{},{},{}", rd, rs1, rs2),
    FminS("fmin\\.s", "{},{},{}", rd, rs1, rs2),
    FmaxS("fmax\\.s", "{},{},{}", rd, rs1, rs2),
    FcvtWS("fcvt\\.w\\.s", "{},{}\\S*", rd, rs1),
    FcvtWuS("fcvt\\.wu\\.s", "{},{}\\S*", rd, rs1),
    FmvXW("fmv\\.x\\.w", "{},{}", rd, rs1),
    FeqS("feq\\.s", "{},{},{}", rd, rs1, rs2),
    FltS("flt\\.s", "{},{},{}", rd, rs1, rs2),
    FleS("fle\\.s", "{},{},{}", rd, rs1, rs2),
    FclassS("fclass\\.s", "{},{}", rd, rs1),
    FcvtSW("fcvt\\.s\\.w", "{},{}\\S*", rd, rs1),
    FcvtSWu("fcvt\\.s\\.wu", "{},{}\\S*", rd, rs1),
    FmvWX("fmv\\.w\\.x", "{},{}", rd, rs1),

    // RV64F (Rounding modes are ignored.)
    FcvtLS("fcvt\\.l\\.s", "{},{}\\S*", rd, rs1),
    FcvtLuS("fcvt\\.lu\\.s", "{},{}\\S*", rd, rs1),
    FcvtSL("fcvt\\.s\\.l", "{},{}\\S*", rd, rs1),
    FcvtSLu("fcvt\\.s\\.lu", "{},{}\\S*", rd, rs1),

    // RV32D (Rounding modes are ignored.)
    Fld("fld", "{},{}\\({}\\)", rd, imm, rs1),
    Fsd("fsd", "{},{}\\({}\\)", rs2, imm, rs1),
    FmaddD("fmadd\\.d", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FmsubD("fmsub\\.d", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FnmsubD("fnmsub\\.d", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FnmaddD("fnmadd\\.d", "{},{},{},{}\\S*", rd, rs1, rs2, rs3),
    FaddD("fadd\\.d", "{},{},{}\\S*", rd, rs1, rs2),
    FsubD("fsub\\.d", "{},{},{}\\S*", rd, rs1, rs2),
    FmulD("fmul\\.d", "{},{},{}\\S*", rd, rs1, rs2),
    FdivD("fdiv\\.d", "{},{},{}\\S*", rd, rs1, rs2),
    FsqrtD("fsqrt\\.d", "{},{}\\S*", rd, rs1),
    FsgnjD("fsgnj\\.d", "{},{},{}", rd, rs1, rs2),
    FsgnjnD("fsgnjn\\.d", "{},{},{}", rd, rs1, rs2),
    FsgnjxD("fsgnjx\\.d", "{},{},{}", rd, rs1, rs2),
    FminD("fmin\\.d", "{},{},{}", rd, rs1, rs2),
    FmaxD("fmax\\.d", "{},{},{}", rd, rs1, rs2),
    FcvtSD("fcvt\\.s\\.d", "{},{}\\S*", rd, rs1),
    FcvtDS("fcvt\\.d\\.s", "{},{}\\S*", rd, rs1),
    FeqD("feq\\.d", "{},{},{}", rd, rs1, rs2),
    FltD("flt\\.d", "{},{},{}", rd, rs1, rs2),
    FleD("fle\\.d", "{},{},{}", rd, rs1, rs2),
    FclassD("fclass\\.d", "{},{}", rd, rs1),
    FcvtWD("fcvt\\.w\\.d", "{},{}\\S*", rd, rs1),
    FcvtWuD("fcvt\\.wu\\.d", "{},{}\\S*", rd, rs1),
    FcvtDW("fcvt\\.d\\.w", "{},{}\\S*", rd, rs1),
    FcvtDWu("fcvt\\.d\\.wu", "{},{}\\S*", rd, rs1),

    // RV64D (Rounding modes are ignored.)
    FcvtLD("fcvt\\.l\\.d", "{},{}\\S*", rd, rs1),
    FcvtLuD("fcvt\\.lu\\.d", "{},{}\\S*", rd, rs1),
    FmvXD("fmv\\.x\\.d", "{},{}", rd, rs1),
    FcvtDL("fcvt\\.d\\.l", "{},{}\\S*", rd, rs1),
    FcvtDLu("fcvt\\.d\\.lu", "{},{}\\S*", rd, rs1),
    FmvDX("fmv\\.d\\.x", "{},{}", rd, rs1),

    // Pseudoinstructions
    Nop("nop", ""),
    Li("li", "{},{}", rd, imm),
    Mv("mv", "{},{}", rd, rs1),
    Not("not", "{},{}", rd, rs1),
    Neg("neg", "{},{}", rd, rs1),
    Negw("negw", "{},{}", rd, rs1),
    SextW("sext\\.w", "{},{}", rd, rs1),
    Seqz("seqz", "{},{}", rd, rs1),
    Snez("snez", "{},{}", rd, rs1),
    Sltz("sltz", "{},{}", rd, rs1),
    Sgtz("sgtz", "{},{}", rd, rs1),

    FmvS("fmv\\.s", "{},{}", rd, rs1),
    FabsS("fabs\\.s", "{},{}", rd, rs1),
    FnegS("fneg\\.s", "{},{}", rd, rs1),
    FmvD("fmv\\.d", "{},{}", rd, rs1),
    FabsD("fabs\\.d", "{},{}", rd, rs1),
    FnegD("fneg\\.d", "{},{}", rd, rs1),

    Beqz("beqz", "{},{}", rs1, addr),
    Bnez("bnez", "{},{}", rs1, addr),
    Blez("blez", "{},{}", rs1, addr),
    Bgez("bgez", "{},{}", rs1, addr),
    Bltz("bltz", "{},{}", rs1, addr),
    Bgtz("bgtz", "{},{}", rs1, addr),

    J("j\\s", "{}", addr),
    // `jal addr` is always disassembled to be `jal ra,addr`.
    Jr("jr", "{}", rs1),
    // `jalr rs1` is handled in `RiscvInstruction::new_irregular()`.
    Ret("ret", ""),
    // Currently GCC emits errors like `relocation truncated to fit: R_RISCV_JAL against `.L22'`
    // when I try to use a far function call to test `call` and `tail` instructions. I will add
    // these two instructions if later I spot them in real code.
}
