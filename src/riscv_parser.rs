use crate::riscv_isa::{riscv_regex, RiscvAddress, RiscvInstruction};
use crate::{addr, build_instruction, imm, rd, rs1, rs2};
use regex::Regex;

lazy_static! {
    static ref TARGET: Regex = Regex::new(r":\s+(.+?)\s+").unwrap();
}

pub fn parse_rodata(source: &str) -> Vec<RiscvAddress> {
    let addrs: Vec<_> = source
        .lines()
        .skip_while(|line| !line.contains(".rodata"))
        .skip(3)
        .map(|line| line.trim())
        .take_while(|line| !line.is_empty())
        .map(|line| TARGET.captures(line).unwrap()[1].to_string())
        .collect();
    if addrs.is_empty() {
        return Vec::new();
    }
    let mut addrs_iter = addrs.iter();
    let mut potential_targets = Vec::new();
    while let (Some(lh), Some(hh)) = (addrs_iter.next(), addrs_iter.next()) {
        let s = format!("{}{}", hh, lh);
        let target = RiscvAddress::from_str_radix(&s, 16).unwrap();
        potential_targets.push(target);
    }
    potential_targets.sort();
    potential_targets.dedup();
    potential_targets
}

pub fn parse_text(source: String) -> Vec<RiscvInstruction> {
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
    match line {
        "" | "..." => {
            *label = None;
            None
        }
        line if riscv_regex::LABEL.is_match(line) => {
            let caps = riscv_regex::LABEL.captures(line).unwrap();
            *label = Some(caps["label"].to_string());
            None
        }
        line => {
            let lb = label.take();
            let inst = build_instruction(line, lb);
            Some(inst)
        }
    }
}

fn build_instruction(line: &str, label: Option<String>) -> RiscvInstruction {
    use RiscvInstruction::*;

    build_instruction! {
        line, label,

        // RV64I
        riscv_regex::ADD => Add(rd, rs1, rs2),
        riscv_regex::ADDW => Addw(rd, rs1, rs2),
        riscv_regex::ADDI => Addi(rd, rs1, imm),
        riscv_regex::ADDIW => Addiw(rd, rs1, imm),
        riscv_regex::AND => And(rd, rs1, rs2),
        riscv_regex::ANDI => Andi(rd, rs1, imm),
        riscv_regex::AUIPC => Auipc(rd, imm),
        riscv_regex::BEQ => Beq(rs1, rs2, addr),
        riscv_regex::BGE => Bge(rs1, rs2, addr),
        riscv_regex::BGE => Bge(rs1, rs2, addr),
        riscv_regex::BGEU => Bgeu(rs1, rs2, addr),
        riscv_regex::BLT => Blt(rs1, rs2, addr),
        riscv_regex::BLTU => Bltu(rs1, rs2, addr),
        riscv_regex::BNE => Bne(rs1, rs2, addr),
        riscv_regex::EBREAK => Ebreak(),
        riscv_regex::ECALL => Ecall(),
        riscv_regex::JAL => Jal(rd, addr),
        riscv_regex::JALR => Jalr(rd, rs1, imm),
        riscv_regex::JALR_IMPLICIT => Jalr(rd, rs1, imm),
        riscv_regex::JALR_MORE_IMPLICIT => Jalr(rd, rs1, imm),
        riscv_regex::LB => Lb(rd, rs1, imm),
        riscv_regex::LBU => Lbu(rd, rs1, imm),
        riscv_regex::LD => Ld(rd, rs1, imm),
        riscv_regex::LH => Lh(rd, rs1, imm),
        riscv_regex::LHU => Lhu(rd, rs1, imm),
        riscv_regex::LUI => Lui(rd, imm),
        riscv_regex::LW => Lw(rd, rs1, imm),
        riscv_regex::LWU => Lwu(rd, rs1, imm),
        riscv_regex::OR => Or(rd, rs1, rs2),
        riscv_regex::ORI => Ori(rd, rs1, imm),
        riscv_regex::SB => Sb(rs1, imm, rs2),
        riscv_regex::SD => Sd(rs1, imm, rs2),
        riscv_regex::SH => Sh(rs1, imm, rs2),
        riscv_regex::SLL => Sll(rd, rs1, rs2),
        riscv_regex::SLLW => Sllw(rd, rs1, rs2),
        riscv_regex::SLLI => Slli(rd, rs1, imm),
        riscv_regex::SLLIW => Slliw(rd, rs1, imm),
        riscv_regex::SLT => Slt(rd, rs1, rs2),
        riscv_regex::SLTI => Slti(rd, rs1, imm),
        riscv_regex::SLTIU => Sltiu(rd, rs1, imm),
        riscv_regex::SLTU => Sltu(rd, rs1, rs2),
        riscv_regex::SRA => Sra(rd, rs1, rs2),
        riscv_regex::SRAW => Sraw(rd, rs1, rs2),
        riscv_regex::SRAI => Srai(rd, rs1, imm),
        riscv_regex::SRAIW => Sraiw(rd, rs1, imm),
        riscv_regex::SRL => Srl(rd, rs1, rs2),
        riscv_regex::SRLW => Srlw(rd, rs1, rs2),
        riscv_regex::SRLI => Srli(rd, rs1, imm),
        riscv_regex::SRLIW => Srliw(rd, rs1, imm),
        riscv_regex::SUB => Sub(rd, rs1, rs2),
        riscv_regex::SUBW => Subw(rd, rs1, rs2),
        riscv_regex::SW => Sw(rs1, imm, rs2),
        riscv_regex::XOR => Xor(rd, rs1, rs2),
        riscv_regex::XORI => Xori(rd, rs1, imm),

        // Pseudo
        riscv_regex::BEQZ => Beqz(rs1, addr),
        riscv_regex::BNEZ => Bnez(rs1, addr),
        riscv_regex::J => J(addr),
        riscv_regex::JR => Jr(rs1),
        riscv_regex::LI => Li(rd, imm),
        riscv_regex::MV => Mv(rd, rs1),
        riscv_regex::NEG => Neg(rd, rs1),
        riscv_regex::NOP => Nop(),
        riscv_regex::NOT => Not(rd, rs1),
        riscv_regex::RET => Ret(),
        riscv_regex::SEQZ => Seqz(rd, rs1),
        riscv_regex::SNEZ => Snez(rd, rs1),
    }
}

#[cfg(test)]
mod tests {
    use crate::build_test;
    use crate::riscv_isa::RiscvInstruction::*;
    use crate::riscv_isa::RiscvRegister::*;

    #[test]
    fn minimal() {
        let source = "
Disassembly of section .text:

00000000000104bc <main>:
   104da:	8082                	ret
        ";
        let potential_targets = super::parse_rodata(source);
        assert_eq!(potential_targets, Vec::new());
        let insts = super::parse_text(source.to_string());
        let expected = vec![Ret {
            label: Some(String::from("main")),
            address: 66778,
            comment: None,
        }];
        assert_eq!(insts, expected);
    }

    #[test]
    fn potential_targets() {
        let source = "

examples/test:     file format elf64-littleriscv


Disassembly of section .plt:

0000000000010380 <_PROCEDURE_LINKAGE_TABLE_>:
   10380:	97 23 00 00 33 03 c3 41 03 be 03 c8 13 03 43 fd     .#..3..A......C.
   10390:	93 82 03 c8 13 53 13 00 83 b2 82 00 67 00 0e 00     .....S......g...

00000000000103a0 <__libc_start_main@plt>:
   103a0:	00002e17          	auipc	t3,0x2
   103a4:	c70e3e03          	ld	t3,-912(t3) # 12010 <__libc_start_main@GLIBC_2.27>
   103a8:	000e0367          	jalr	t1,t3
   103ac:	00000013          	nop

Disassembly of section .text:

00000000000103de <load_gp>:
   103de:	00002197          	auipc	gp,0x2
   103e2:	42218193          	addi	gp,gp,1058 # 12800 <__global_pointer$>
   103e6:	8082                	ret
	...

0000000000010650 <__libc_csu_fini>:
   10650:	8082                	ret

Disassembly of section .rodata:

0000000000010538 <.rodata>:
   10538:	0486                	slli	s1,s1,0x1
   1053a:	0001                	nop
   10538:	0486                	slli	s1,s1,0x1
   1053a:	0001                	nop
        ";
        let potential_targets = super::parse_rodata(source);
        let expected = vec![66694];
        assert_eq!(potential_targets, expected);
    }

    #[test]
    fn label() {
        let source = "

examples/test:     file format elf64-littleriscv


Disassembly of section .plt:

0000000000010380 <_PROCEDURE_LINKAGE_TABLE_>:
   10380:	97 23 00 00 33 03 c3 41 03 be 03 c8 13 03 43 fd     .#..3..A......C.
   10390:	93 82 03 c8 13 53 13 00 83 b2 82 00 67 00 0e 00     .....S......g...

00000000000103a0 <__libc_start_main@plt>:
   103a0:	00002e17          	auipc	t3,0x2
   103a4:	c70e3e03          	ld	t3,-912(t3) # 12010 <__libc_start_main@GLIBC_2.27>
   103a8:	000e0367          	jalr	t1,t3
   103ac:	00000013          	nop

Disassembly of section .text:

00000000000103de <load_gp>:
   103de:	00002197          	auipc	gp,0x2
   103e2:	42218193          	addi	gp,gp,1058 # 12800 <__global_pointer$>
   103e6:	8082                	ret
	...

0000000000010650 <__libc_csu_fini>:
   10650:	8082                	ret

Disassembly of section .rodata:

0000000000010538 <.rodata>:
   10538:	0486                	slli	s1,s1,0x1
   1053a:	0001                	nop
   10538:	0486                	slli	s1,s1,0x1
   1053a:	0001                	nop
        ";
        let insts = super::parse_text(source.to_string());
        let expected = vec![
            Auipc {
                label: Some(String::from("load_gp")),
                address: 66526,
                rd: Gp,
                imm: 2,
                comment: None,
            },
            Addi {
                label: None,
                address: 66530,
                rd: Gp,
                rs1: Gp,
                imm: 1058,
                comment: Some(String::from(" # 12800 <__global_pointer$>")),
            },
            Ret {
                label: None,
                address: 66534,
                comment: None,
            },
            Ret {
                label: Some(String::from("__libc_csu_fini")),
                address: 67152,
                comment: None,
            },
        ];
        assert_eq!(insts, expected);
    }

    build_test! {
        // RV64I
        add(
            "10456:	00b502b3          	add	t0,a0,a1",
            Add {
                address: 66646,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        addw(
            "1045a:	00b502bb          	addw	t0,a0,a1",
            Addw {
                address: 66650,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        addi(
            "1045e:	00150293          	addi	t0,a0,1",
            Addi {
                address: 66654,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        addiw(
            "10462:	0015029b          	addiw	t0,a0,1",
            Addiw {
                address: 66658,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        and(
            "10466:	00b572b3          	and	t0,a0,a1",
            And {
                address: 66662,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        andi(
            "1046a:	00157293          	andi	t0,a0,1",
            Andi {
                address: 66666,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        auipc(
            "1046e:	00001297          	auipc	t0,0x1",
            Auipc {
                address: 66670,
                rd: T0,
                imm: 1,
            }
        ),
        beq(
            "10472:	16b50b63          	beq	a0,a1,105e8 <main>",
            Beq {
                address: 66674,
                rs1: A0,
                rs2: A1,
                addr: 67048,
            }
        ),
        bge(
            "10476:	16b55963          	bge	a0,a1,105e8 <main>",
            Bge {
                address: 66678,
                rs1: A0,
                rs2: A1,
                addr: 67048,
            }
        ),
        bgeu(
            "1047a:	16b57763          	bgeu	a0,a1,105e8 <main>",
            Bgeu {
                address: 66682,
                rs1: A0,
                rs2: A1,
                addr: 67048,
            }
        ),
        blt(
            "1047e:	16b54563          	blt	a0,a1,105e8 <main>",
            Blt {
                address: 66686,
                rs1: A0,
                rs2: A1,
                addr: 67048,
            }
        ),
        bltu(
            "10482:	16b56363          	bltu	a0,a1,105e8 <main>",
            Bltu {
                address: 66690,
                rs1: A0,
                rs2: A1,
                addr: 67048,
            }
        ),
        bne(
            "10486:	16b51163          	bne	a0,a1,105e8 <main>",
            Bne {
                address: 66694,
                rs1: A0,
                rs2: A1,
                addr: 67048,
            }
        ),
        ebreak(
            "1048a:	9002                	ebreak",
            Ebreak {
                address: 66698,
            }
        ),
        ecall(
            "1048c:	00000073          	ecall",
            Ecall {
                address: 66700,
            }
        ),
        jal(
            "10490:	158000ef          	jal	ra,105e8 <main>",
            Jal {
                address: 66704,
                rd: Ra,
                addr: 67048,
            }
        ),
        jalr_implicit(
            "10494:	001500e7          	jalr	1(a0)",
            Jalr {
                address: 66708,
                rd: Ra,
                rs1: A0,
                imm: 1,
            }
        ),
        jalr_more_implicit(
            "104b4:   9782                    jalr    a5",
            Jalr {
                address: 66740,
                rd: Ra,
                rs1: A5,
                imm: 0,
            }
        ),
        jalr(
            "10456:	001502e7          	jalr	t0,1(a0)",
            Jalr {
                address: 66646,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        lb(
            "10498:	00150283          	lb	t0,1(a0)",
            Lb {
                address: 66712,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        lbu(
            "1049c:	00154283          	lbu	t0,1(a0)",
            Lbu {
                address: 66716,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        ld(
            "104a0:	00153283          	ld	t0,1(a0)",
            Ld {
                address: 66720,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        lh(
            "104a4:	00151283          	lh	t0,1(a0)",
            Lh {
                address: 66724,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        lhu(
            "104a8:	00155283          	lhu	t0,1(a0)",
            Lhu {
                address: 66728,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        lui(
            "104ac:	6285                	lui	t0,0x1",
            Lui {
                address: 66732,
                rd: T0,
                imm: 1,
            }
        ),
        lw(
            "104ae:	00152283          	lw	t0,1(a0)",
            Lw {
                address: 66734,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        lwu(
            "104b2:	00156283          	lwu	t0,1(a0)",
            Lwu {
                address: 66738,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        or(
            "104b6:	00b562b3          	or	t0,a0,a1",
            Or {
                address: 66742,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        ori(
            "104ba:	00156293          	ori	t0,a0,1",
            Ori {
                address: 66746,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        sb(
            "104be:	00b500a3          	sb	a1,1(a0)",
            Sb {
                address: 66750,
                rs1: A0,
                imm: 1,
                rs2: A1,
            }
        ),
        sd(
            "104c2:	00b530a3          	sd	a1,1(a0)",
            Sd {
                address: 66754,
                rs1: A0,
                imm: 1,
                rs2: A1,
            }
        ),
        sh(
            "104c6:	00b510a3          	sh	a1,1(a0)",
            Sh {
                address: 66758,
                rs1: A0,
                imm: 1,
                rs2: A1,
            }
        ),
        sll(
            "104ca:	00b512b3          	sll	t0,a0,a1",
            Sll {
                address: 66762,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        sllw(
            "104ce:	00b512bb          	sllw	t0,a0,a1",
            Sllw {
                address: 66766,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        slli(
            "104d2:	00151293          	slli	t0,a0,0x1",
            Slli {
                address: 66770,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        slliw(
            "104d6:	0015129b          	slliw	t0,a0,0x1",
            Slliw {
                address: 66774,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        slt(
            "104da:	00b522b3          	slt	t0,a0,a1",
            Slt {
                address: 66778,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        slti(
            "104de:	00152293          	slti	t0,a0,1",
            Slti {
                address: 66782,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        sltiu(
            "104e2:	00253293          	sltiu	t0,a0,2",
            Sltiu {
                address: 66786,
                rd: T0,
                rs1: A0,
                imm: 2,
            }
        ),
        sltu(
            "104e6:	00b532b3          	sltu	t0,a0,a1",
            Sltu {
                address: 66790,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        sra(
            "104ea:	40b552b3          	sra	t0,a0,a1",
            Sra {
                address: 66794,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        sraw(
            "104ee:	40b552bb          	sraw	t0,a0,a1",
            Sraw {
                address: 66798,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        srai(
            "104f2:	40155293          	srai	t0,a0,0x1",
            Srai {
                address: 66802,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        sraiw(
            "104f6:	4015529b          	sraiw	t0,a0,0x1",
            Sraiw {
                address: 66806,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        srl(
            "104fa:	00b552b3          	srl	t0,a0,a1",
            Srl {
                address: 66810,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        srlw(
            "104fe:	00b552bb          	srlw	t0,a0,a1",
            Srlw {
                address: 66814,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        srli(
            "10502:	00155293          	srli	t0,a0,0x1",
            Srli {
                address: 66818,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        srliw(
            "10506:	0015529b          	srliw	t0,a0,0x1",
            Srliw {
                address: 66822,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),
        sub(
            "1050a:	40b502b3          	sub	t0,a0,a1",
            Sub {
                address: 66826,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        subw(
            "1050e:	40b502bb          	subw	t0,a0,a1",
            Subw {
                address: 66830,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        sw(
            "10512:	00b520a3          	sw	a1,1(a0)",
            Sw {
                address: 66834,
                rs1: A0,
                imm: 1,
                rs2: A1,
            }
        ),
        xor(
            "10516:	00b542b3          	xor	t0,a0,a1",
            Xor {
                address: 66838,
                rd: T0,
                rs1: A0,
                rs2: A1,
            }
        ),
        xori(
            "1051a:	00154293          	xori	t0,a0,1",
            Xori {
                address: 66842,
                rd: T0,
                rs1: A0,
                imm: 1,
            }
        ),

        // Pseudo
        beqz(
            "1052c:	cd55                	beqz	a0,105e8 <main>",
            Beqz {
                address: 66860,
                rs1: A0,
                addr: 67048,
            }
        ),
        bnez(
            "1052e:	ed4d                	bnez	a0,105e8 <main>",
            Bnez {
                address: 66862,
                rs1: A0,
                addr: 67048,
            }
        ),
        j(
            "10530:	a865                	j	105e8 <main>",
            J {
                address: 66864,
                addr: 67048,
            }
        ),
        jr(
            "10532:	8502                	jr	a0",
            Jr {
                address: 66866,
                rs1: A0,
            }
        ),
        li(
            "1053c:	4285                	li	t0,1",
            Li {
                address: 66876,
                rd: T0,
                imm: 1,
            }
        ),
        mv(
            "1053e:	82aa                	mv	t0,a0",
            Mv {
                address: 66878,
                rd: T0,
                rs1: A0,
            }
        ),
        neg(
            "10540:	40a002b3          	neg	t0,a0",
            Neg {
                address: 66880,
                rd: T0,
                rs1: A0,
            }
        ),
        nop(
            "10544:	0001                	nop",
            Nop {
                address: 66884,
            }
        ),
        not(
            "10546:	fff54293          	not	t0,a0",
            Not {
                address: 66886,
                rd: T0,
                rs1: A0,
            }
        ),
        ret(
            "1054a:	8082                	ret",
            Ret {
                address: 66890,
            }
        ),
        seqz(
            "1054c:	00153293          	seqz	t0,a0",
            Seqz {
                address: 66892,
                rd: T0,
                rs1: A0,
            }
        ),
        snez(
            "10550:	00a032b3          	snez	t0,a0",
            Snez {
                address: 66896,
                rd: T0,
                rs1: A0,
            }
        ),

        // Register
        zero(
            "10562:	000002b3          	add	t0,zero,zero",
            Add {
                address: 66914,
                rd: T0,
                rs1: Zero,
                rs2: Zero,
            }
        ),
        ra(
            "10566:	001082b3          	add	t0,ra,ra",
            Add {
                address: 66918,
                rd: T0,
                rs1: Ra,
                rs2: Ra,
            }
        ),
        sp(
            "1056a:	002102b3          	add	t0,sp,sp",
            Add {
                address: 66922,
                rd: T0,
                rs1: Sp,
                rs2: Sp,
            }
        ),
        gp(
            "1056e:	003182b3          	add	t0,gp,gp",
            Add {
                address: 66926,
                rd: T0,
                rs1: Gp,
                rs2: Gp,
            }
        ),
        tp(
            "10572:	004202b3          	add	t0,tp,tp",
            Add {
                address: 66930,
                rd: T0,
                rs1: Tp,
                rs2: Tp,
            }
        ),
        t0(
            "10576:	9296                	add	t0,t0,t0",
            Add {
                address: 66934,
                rd: T0,
                rs1: T0,
                rs2: T0,
            }
        ),
        t1(
            "10578:	006302b3          	add	t0,t1,t1",
            Add {
                address: 66936,
                rd: T0,
                rs1: T1,
                rs2: T1,
            }
        ),
        t2(
            "1057c:	007382b3          	add	t0,t2,t2",
            Add {
                address: 66940,
                rd: T0,
                rs1: T2,
                rs2: T2,
            }
        ),
        s0(
            "10580:	008402b3          	add	t0,s0,s0",
            Add {
                address: 66944,
                rd: T0,
                rs1: S0,
                rs2: S0,
            }
        ),
        s1(
            "10584:	009482b3          	add	t0,s1,s1",
            Add {
                address: 66948,
                rd: T0,
                rs1: S1,
                rs2: S1,
            }
        ),
        a0(
            "10588:	00a502b3          	add	t0,a0,a0",
            Add {
                address: 66952,
                rd: T0,
                rs1: A0,
                rs2: A0,
            }
        ),
        a1(
            "1058c:	00b582b3          	add	t0,a1,a1",
            Add {
                address: 66956,
                rd: T0,
                rs1: A1,
                rs2: A1,
            }
        ),
        a2(
            "10590:	00c602b3          	add	t0,a2,a2",
            Add {
                address: 66960,
                rd: T0,
                rs1: A2,
                rs2: A2,
            }
        ),
        a3(
            "10594:	00d682b3          	add	t0,a3,a3",
            Add {
                address: 66964,
                rd: T0,
                rs1: A3,
                rs2: A3,
            }
        ),
        a4(
            "10598:	00e702b3          	add	t0,a4,a4",
            Add {
                address: 66968,
                rd: T0,
                rs1: A4,
                rs2: A4,
            }
        ),
        a5(
            "1059c:	00f782b3          	add	t0,a5,a5",
            Add {
                address: 66972,
                rd: T0,
                rs1: A5,
                rs2: A5,
            }
        ),
        a6(
            "105a0:	010802b3          	add	t0,a6,a6",
            Add {
                address: 66976,
                rd: T0,
                rs1: A6,
                rs2: A6,
            }
        ),
        a7(
            "105a4:	011882b3          	add	t0,a7,a7",
            Add {
                address: 66980,
                rd: T0,
                rs1: A7,
                rs2: A7,
            }
        ),
        s2(
            "105a8:	012902b3          	add	t0,s2,s2",
            Add {
                address: 66984,
                rd: T0,
                rs1: S2,
                rs2: S2,
            }
        ),
        s3(
            "105ac:	013982b3          	add	t0,s3,s3",
            Add {
                address: 66988,
                rd: T0,
                rs1: S3,
                rs2: S3,
            }
        ),
        s4(
            "105b0:	014a02b3          	add	t0,s4,s4",
            Add {
                address: 66992,
                rd: T0,
                rs1: S4,
                rs2: S4,
            }
        ),
        s5(
            "105b4:	015a82b3          	add	t0,s5,s5",
            Add {
                address: 66996,
                rd: T0,
                rs1: S5,
                rs2: S5,
            }
        ),
        s6(
            "105b8:	016b02b3          	add	t0,s6,s6",
            Add {
                address: 67000,
                rd: T0,
                rs1: S6,
                rs2: S6,
            }
        ),
        s7(
            "105bc:	017b82b3          	add	t0,s7,s7",
            Add {
                address: 67004,
                rd: T0,
                rs1: S7,
                rs2: S7,
            }
        ),
        s8(
            "105c0:	018c02b3          	add	t0,s8,s8",
            Add {
                address: 67008,
                rd: T0,
                rs1: S8,
                rs2: S8,
            }
        ),
        s9(
            "105c4:	019c82b3          	add	t0,s9,s9",
            Add {
                address: 67012,
                rd: T0,
                rs1: S9,
                rs2: S9,
            }
        ),
        s10(
            "105c8:	01ad02b3          	add	t0,s10,s10",
            Add {
                address: 67016,
                rd: T0,
                rs1: S10,
                rs2: S10,
            }
        ),
        s11(
            "105cc:	01bd82b3          	add	t0,s11,s11",
            Add {
                address: 67020,
                rd: T0,
                rs1: S11,
                rs2: S11,
            }
        ),
        t3(
            "105d0:	01ce02b3          	add	t0,t3,t3",
            Add {
                address: 67024,
                rd: T0,
                rs1: T3,
                rs2: T3,
            }
        ),
        t4(
            "105d4:	01de82b3          	add	t0,t4,t4",
            Add {
                address: 67028,
                rd: T0,
                rs1: T4,
                rs2: T4,
            }
        ),
        t5(
            "105d8:	01ef02b3          	add	t0,t5,t5",
            Add {
                address: 67032,
                rd: T0,
                rs1: T5,
                rs2: T5,
            }
        ),
        t6(
            "105dc:	01ff82b3          	add	t0,t6,t6",
            Add {
                address: 67036,
                rd: T0,
                rs1: T6,
                rs2: T6,
            }
        ),
    }
}
