use crate::riscv_isa::{RiscvAddress, RiscvInstruction};
use regex::Regex;
use std::collections::HashMap;

pub fn parse_rodata(source: &str) -> HashMap<RiscvAddress, RiscvAddress> {
    let addrs: Vec<_> = source
        .lines()
        .skip_while(|line| !line.contains(".rodata"))
        .skip(3)
        .map(|line| line.trim())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            lazy_static! {
                static ref TARGET: Regex = Regex::new(r"(.+):\s+(.+?)\s+").unwrap();
            }
            let caps = TARGET.captures(line).unwrap();
            (caps[1].to_string(), caps[2].to_string())
        })
        .collect();
    if addrs.is_empty() {
        return HashMap::new();
    }
    let mut addrs_iter = addrs.iter();
    let mut jump_targets = Vec::new();
    while let (Some(lh), Some(hh)) = (addrs_iter.next(), addrs_iter.next()) {
        let addr = RiscvAddress::new(&lh.0);
        let target = RiscvAddress::new(&(hh.1.clone() + &lh.1));
        jump_targets.push((addr, target));
    }
    jump_targets.into_iter().collect()
}

pub fn parse_text(source: &str) -> Vec<RiscvInstruction> {
    let lines: Vec<_> = source
        .lines()
        .skip_while(|line| !line.contains(".text"))
        .skip(1)
        .map(|line| line.trim())
        .take_while(|line| !line.starts_with("Disassembly"))
        .collect();
    let mut label = None;
    let mut insts = Vec::new();
    for line in lines {
        if let Some(inst) = parse_line(line, &mut label) {
            insts.push(inst);
        }
    }
    insts
}

fn parse_line(line: &str, label: &mut Option<String>) -> Option<RiscvInstruction> {
    lazy_static! {
        static ref LABEL: Regex = Regex::new(r"[[:xdigit:]]+ <(\S+)>:").unwrap();
    }
    match line {
        "" | "..." => {
            *label = None;
            None
        }
        line if LABEL.is_match(line) => {
            let caps = LABEL.captures(line).unwrap();
            *label = Some(caps[1].to_string());
            None
        }
        line => {
            let lb = label.take();
            let inst = RiscvInstruction::new(line, lb);
            Some(inst)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::build_test;
    use crate::riscv_isa::{
        RiscvAddress, RiscvImmediate,
        RiscvInstruction::{self, *},
        RiscvRegister::*,
    };
    use std::collections::HashMap;

    #[test]
    fn basic() {
        let source = "

            examples/test:     file format elf64-littleriscv


            Disassembly of section .text:
            
            00000000000104c6 <main>:
                103ea:	6549                	lui	a0,0x12
                103ea:	6549                	lui	a0,0x12 <deregister_tm_clones+0x1c>
                ...
            
            Disassembly of section .rodata:
            
            0000000000010594 <.rodata>:
                10594:	04ba                	slli	s1,s1,0xe
                10596:	0001                	nop
                10598:	047e                	slli	s0,s0,0x1f
                1059a:	0001                	nop
            
        ";

        let jump_targets = super::parse_rodata("");
        assert!(jump_targets.is_empty());

        let jump_targets = super::parse_rodata(source);
        let mut expected: HashMap<RiscvAddress, RiscvAddress> = HashMap::new();
        expected.insert(0x10594.into(), 0x000104ba.into());
        expected.insert(0x10598.into(), 0x0001047e.into());
        assert_eq!(jump_targets, expected);

        let insts = super::parse_text(source);
        let expected = vec![Lui {
            label: Some(String::from("main")),
            address: 0x103ea.into(),
            rd: A0,
            imm: RiscvImmediate::new("0x12"),
            comment: None,
        },Lui {
            label: None,
            address: 0x103ea.into(),
            rd: A0,
            imm: RiscvImmediate::new("0x12"),
            comment: Some(String::from("<deregister_tm_clones+0x1c>")),
        }];
        assert_eq!(insts, expected);
    }

    build_test! {
        // Registers: 32 tests
        reg_1("flw	ft0,-20(zero)", Flw { rd: Ft0, imm: (-20).into(), rs1: Zero }),
        reg_2("flw	ft1,-20(ra)", Flw { rd: Ft1, imm: (-20).into(), rs1: Ra }),
        reg_3("flw	ft2,-20(sp)", Flw { rd: Ft2, imm: (-20).into(), rs1: Sp }),
        reg_4("flw	ft3,-20(gp)", Flw { rd: Ft3, imm: (-20).into(), rs1: Gp }),
        reg_5("flw	ft4,-20(tp)", Flw { rd: Ft4, imm: (-20).into(), rs1: Tp }),
        reg_6("flw	ft5,-20(t0)", Flw { rd: Ft5, imm: (-20).into(), rs1: T0 }),
        reg_7("flw	ft6,-20(t1)", Flw { rd: Ft6, imm: (-20).into(), rs1: T1 }),
        reg_8("flw	ft7,-20(t2)", Flw { rd: Ft7, imm: (-20).into(), rs1: T2 }),
        reg_9("flw	fs0,-20(s0)", Flw { rd: Fs0, imm: (-20).into(), rs1: S0 }),
        reg_10("flw	fs1,-20(s1)", Flw { rd: Fs1, imm: (-20).into(), rs1: S1 }),
        reg_11("flw	fa0,-20(a0)", Flw { rd: Fa0, imm: (-20).into(), rs1: A0 }),
        reg_12("flw	fa1,-20(a1)", Flw { rd: Fa1, imm: (-20).into(), rs1: A1 }),
        reg_13("flw	fa2,-20(a2)", Flw { rd: Fa2, imm: (-20).into(), rs1: A2 }),
        reg_14("flw	fa3,-20(a3)", Flw { rd: Fa3, imm: (-20).into(), rs1: A3 }),
        reg_15("flw	fa4,-20(a4)", Flw { rd: Fa4, imm: (-20).into(), rs1: A4 }),
        reg_16("flw	fa5,-20(a5)", Flw { rd: Fa5, imm: (-20).into(), rs1: A5 }),
        reg_17("flw	fa6,-20(a6)", Flw { rd: Fa6, imm: (-20).into(), rs1: A6 }),
        reg_18("flw	fa7,-20(a7)", Flw { rd: Fa7, imm: (-20).into(), rs1: A7 }),
        reg_19("flw	fs2,-20(s2)", Flw { rd: Fs2, imm: (-20).into(), rs1: S2 }),
        reg_20("flw	fs3,-20(s3)", Flw { rd: Fs3, imm: (-20).into(), rs1: S3 }),
        reg_21("flw	fs4,-20(s4)", Flw { rd: Fs4, imm: (-20).into(), rs1: S4 }),
        reg_22("flw	fs5,-20(s5)", Flw { rd: Fs5, imm: (-20).into(), rs1: S5 }),
        reg_23("flw	fs6,-20(s6)", Flw { rd: Fs6, imm: (-20).into(), rs1: S6 }),
        reg_24("flw	fs7,-20(s7)", Flw { rd: Fs7, imm: (-20).into(), rs1: S7 }),
        reg_25("flw	fs8,-20(s8)", Flw { rd: Fs8, imm: (-20).into(), rs1: S8 }),
        reg_26("flw	fs9,-20(s9)", Flw { rd: Fs9, imm: (-20).into(), rs1: S9 }),
        reg_27("flw	fs10,-20(s10)", Flw { rd: Fs10, imm: (-20).into(), rs1: S10 }),
        reg_28("flw	fs11,-20(s11)", Flw { rd: Fs11, imm: (-20).into(), rs1: S11 }),
        reg_29("flw	ft8,-20(t3)", Flw { rd: Ft8, imm: (-20).into(), rs1: T3 }),
        reg_30("flw	ft9,-20(t4)", Flw { rd: Ft9, imm: (-20).into(), rs1: T4 }),
        reg_31("flw	ft10,-20(t5)", Flw { rd: Ft10, imm: (-20).into(), rs1: T5 }),
        reg_32("flw	ft11,-20(t6)", Flw { rd: Ft11, imm: (-20).into(), rs1: T6 }),

        // RV32I: 43 tests
        lui("lui	a0,0x12", Lui { rd: A0, imm: 0x12.into() }),
        auipc("auipc	a0,0x0", Auipc { rd: A0, imm: 0x0.into() }),
        jal("jal	ra,103de", Jal { rd: Ra, addr: 0x103de.into() }),
        jalr("jalr	t1,1(t0)", Jalr { rd: T1, imm: 1.into(), rs1: T0 }),
        jalr_imm_rs1("jalr	1(t0)", Jalr { rd: Ra, imm: 1.into(), rs1: T0 }),
        jalr_rd_rs1("jalr	t1,t0", Jalr { rd: T1, imm: 0.into(), rs1: T0 }),
        jalr_rs1("jalr	t0", Jalr { rd: Ra, imm: 0.into(), rs1: T0 }),
        beq("beq	a4,a5,10406", Beq { rs1: A4, rs2: A5, addr: 0x10406.into() }),
        bne("bne	a4,a5,10406", Bne { rs1: A4, rs2: A5, addr: 0x10406.into() }),
        blt("blt	a4,a5,10406", Blt { rs1: A4, rs2: A5, addr: 0x10406.into() }),
        bge("bge	a4,a5,10406", Bge { rs1: A4, rs2: A5, addr: 0x10406.into() }),
        bltu("bltu	a4,a5,10406", Bltu { rs1: A4, rs2: A5, addr: 0x10406.into() }),
        bgeu("bgeu	a4,a5,10406", Bgeu { rs1: A4, rs2: A5, addr: 0x10406.into() }),
        lb("lb	a5,-20(s0)", Lb { rd: A5, imm: (-20).into(), rs1: S0 }),
        lh("lh	a5,-20(s0)", Lh { rd: A5, imm: (-20).into(), rs1: S0 }),
        lw("lw	a5,-20(s0)", Lw { rd: A5, imm: (-20).into(), rs1: S0 }),
        lbu("lbu	a5,-20(s0)", Lbu { rd: A5, imm: (-20).into(), rs1: S0 }),
        lhu("lhu	a5,-20(s0)", Lhu { rd: A5, imm: (-20).into(), rs1: S0 }),
        sb("sb	a5,-2000(gp)", Sb { rs2: A5, imm: (-2000).into(), rs1: Gp }),
        sh("sh	a5,-2000(gp)", Sh { rs2: A5, imm: (-2000).into(), rs1: Gp }),
        sw("sw	a5,-2000(gp)", Sw { rs2: A5, imm: (-2000).into(), rs1: Gp }),
        addi("addi	a2,sp,8", Addi { rd: A2, rs1: Sp, imm: 8.into() }),
        slti("slti	t0,t1,0", Slti { rd: T0, rs1: T1, imm: 0.into() }),
        sltiu("sltiu	t0,t1,0", Sltiu { rd: T0, rs1: T1, imm: 0.into() }),
        xori("xori	t0,t1,0", Xori { rd: T0, rs1: T1, imm: 0.into() }),
        ori("ori	t0,t1,0", Ori { rd: T0, rs1: T1, imm: 0.into() }),
        andi("andi	t0,t1,0", Andi { rd: T0, rs1: T1, imm: 0.into() }),
        slli("slli	a4,a5,0x2", Slli { rd: A4, rs1: A5, imm: 0x2.into() }),
        srli("srli	a5,a1,0x3f", Srli { rd: A5, rs1: A1, imm: 0x3f.into() }),
        srai("srai	a5,a1,0x3", Srai { rd: A5, rs1: A1, imm: 0x3.into() }),
        add("add	t0,t1,t2", Add { rd: T0, rs1: T1, rs2: T2 }),
        sub("sub	t0,t1,t2", Sub { rd: T0, rs1: T1, rs2: T2 }),
        sll("sll	t0,t1,t2", Sll { rd: T0, rs1: T1, rs2: T2 }),
        slt("slt	t0,t1,t2", Slt { rd: T0, rs1: T1, rs2: T2 }),
        sltu("sltu	t0,t1,t2", Sltu { rd: T0, rs1: T1, rs2: T2 }),
        xor("xor	t0,t1,t2", Xor { rd: T0, rs1: T1, rs2: T2 }),
        srl("srl	t0,t1,t2", Srl { rd: T0, rs1: T1, rs2: T2 }),
        sra("sra	t0,t1,t2", Sra { rd: T0, rs1: T1, rs2: T2 }),
        or("or	t0,t1,t2", Or { rd: T0, rs1: T1, rs2: T2 }),
        and("and	t0,t1,t2", And { rd: T0, rs1: T1, rs2: T2 }),
        fence("fence", Fence {}),
        ecall("ecall", Ecall {}),
        ebreak("ebreak", Ebreak {}),

        // RV64I: 12 tests
        lwu("lwu	a5,-20(s0)", Lwu { rd: A5, imm: (-20).into(), rs1: S0 }),
        ld("ld	a1,0(sp)", Ld { rd: A1, imm: 0.into(), rs1: Sp }),
        sd("sd	s0,0(sp)", Sd { rs2: S0, imm: 0.into(), rs1: Sp }),
        addiw("addiw	t0,t1,1", Addiw { rd: T0, rs1: T1, imm: 1.into() }),
        slliw("slliw	a4,a5,0x2", Slliw { rd: A4, rs1: A5, imm: 0x2.into() }),
        srliw("srliw	a4,a5,0x2", Srliw { rd: A4, rs1: A5, imm: 0x2.into() }),
        sraiw("sraiw	a4,a5,0x2", Sraiw { rd: A4, rs1: A5, imm: 0x2.into() }),
        addw("addw	t0,t1,t2", Addw { rd: T0, rs1: T1, rs2: T2 }),
        subw("subw	t0,t1,t2", Subw { rd: T0, rs1: T1, rs2: T2 }),
        sllw("sllw	t0,t1,t2", Sllw { rd: T0, rs1: T1, rs2: T2 }),
        srlw("srlw	t0,t1,t2", Srlw { rd: T0, rs1: T1, rs2: T2 }),
        sraw("sraw	t0,t1,t2", Sraw { rd: T0, rs1: T1, rs2: T2 }),

        // RV32F
        flw("flw	fa5,-20(s0)", Flw { rd: Fa5, imm: (-20).into(), rs1: S0 }),
    }
}
