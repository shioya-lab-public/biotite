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
        let addr = RiscvAddress::from_str_radix(&lh.0, 16).unwrap();
        let target = RiscvAddress::from_str_radix(&(hh.1.clone() + &lh.1), 16).unwrap();
        jump_targets.push((addr, target));
    }
    jump_targets.sort_unstable();
    jump_targets.dedup();
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
    use crate::riscv_isa::RiscvImmediate;
    use crate::riscv_isa::RiscvInstruction::{self, *};
    use crate::riscv_isa::RiscvRegister::*;
    use std::collections::HashMap;

    #[test]
    fn basic() {
        let source = "

            examples/test:     file format elf64-littleriscv


            Disassembly of section .text:
            
            00000000000104c6 <main>:
                103ea:	6549                	lui	a0,0x12
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
        let mut expected = HashMap::new();
        expected.insert(0x10594, 0x000104ba);
        expected.insert(0x10598, 0x0001047e);
        assert_eq!(jump_targets, expected);

        let insts = super::parse_text(source);
        let expected = vec![Lui {
            label: Some(String::from("main")),
            address: 0x103ea,
            rd: A0,
            imm: RiscvImmediate::new("0x12"),
            comment: None,
        }];
        assert_eq!(insts, expected);
    }

    build_test! {
        lui("lui	zero,0", Lui{rd: Zero, imm: 0.into()}),
        jalr("jalr	ra,0(a5)", Jalr{rd: Ra, rs1: A5, imm: 0.into()}),
        ecall("ecall", Ecall{}),
    }
}
