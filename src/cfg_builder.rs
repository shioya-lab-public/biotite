use crate::cfg::{BasicBlock, Cfg, RiscvFunction};
use crate::riscv_isa::{RiscvAddress, RiscvInstruction, FUNCTION};
use std::collections::HashSet;
use std::mem;

pub struct CfgBuilder {
    functions: HashSet<String>,
    instructions: Vec<RiscvInstruction>,
    potential_targets: Vec<RiscvAddress>,
    cfg: Cfg,
}

impl CfgBuilder {
    pub fn new(instructions: Vec<RiscvInstruction>, potential_targets: Vec<RiscvAddress>) -> Self {
        CfgBuilder {
            functions: HashSet::new(),
            instructions,
            potential_targets,
            cfg: Cfg::new(),
        }
    }

    pub fn run(&mut self) -> Cfg {
        self.build_function("main");
        mem::take(&mut self.cfg)
    }

    fn build_function(&mut self, name: &str) {
        self.functions.insert(name.to_string());
        let index = self
            .instructions
            .iter()
            .position(|inst| inst.label() == &Some(name.to_string()))
            .unwrap();
        let (basic_blocks, potential_targets) = self.build_basic_blocks(index);
        let func = RiscvFunction {
            name: name.to_string(),
            basic_blocks,
            potential_targets,
        };
        self.cfg.push(func);
    }

    fn build_basic_blocks(&mut self, index: usize) -> (Vec<BasicBlock>, Vec<usize>) {
        use RiscvInstruction::*;

        // Store heads for basic blocks.
        let mut heads = vec![index];

        // Find continue and jump targets.
        let mut targets = Vec::new();
        let mut idx = index;
        while let Some(inst) = self.instructions.get(idx) {
            // Stop when we find the next function.
            if idx != index && inst.label().is_some() {
                break;
            }

            match inst {
                Beq { addr, .. }
                | Bge { addr, .. }
                | Bgeu { addr, .. }
                | Blt { addr, .. }
                | Bltu { addr, .. }
                | Bne { addr, .. }
                | Beqz { addr, .. }
                | Bnez { addr, .. } => {
                    let index = self.address_to_index(addr);
                    targets.push((idx + 1, Some(idx + 1), Some(index)));
                    heads.extend(vec![idx + 1, index]);
                }
                J { addr, .. } => {
                    let index = self.address_to_index(addr);
                    targets.push((idx + 1, None, Some(index)));
                    heads.extend(vec![idx + 1, index]);
                }
                Jr { .. } | Ret { .. } => {
                    targets.push((idx + 1, None, None));
                    heads.push(idx + 1);
                }
                Jal { comment, .. } | Jalr { comment, .. } => {
                    let caps = FUNCTION.captures(comment.as_ref().unwrap()).unwrap();
                    let name = caps[1].to_string();
                    if !self.functions.contains(&name) {
                        self.build_function(&name);
                    }
                }
                _ => {}
            }
            idx += 1;
        }

        // Find potential targets that lay within this function.
        let potential_targets: Vec<_> = self
            .potential_targets
            .iter()
            .map(|addr| self.address_to_index(addr))
            .filter(|i| &index <= i && i < &idx)
            .collect();

        // Find heads for basic blocks.
        heads.extend(potential_targets.clone());
        heads.sort();
        heads.dedup();
        heads.pop(); // Remove the `idx + 1` target for the final `ret`.

        // Build basic blocks.
        let mut blocks = Vec::new();
        let mut head_index = 0;
        let mut target_index = 0;
        while head_index < heads.len() {
            let head = heads[head_index];
            let next_head = heads.get(head_index + 1).cloned();
            let next_jump = targets[target_index].0;
            if matches!(next_head, Some(next_head) if next_head < next_jump) {
                let block = BasicBlock {
                    instructions: self.instructions[head..next_head.unwrap()].to_vec(),
                    continue_target: Some(self.find_basic_block_index(&heads, &next_head.unwrap())),
                    jump_target: None,
                };
                blocks.push(block);
            } else {
                let continue_target = targets[target_index]
                    .1
                    .map(|i| self.find_basic_block_index(&heads, &i));
                let jump_target = targets[target_index]
                    .2
                    .map(|i| self.find_basic_block_index(&heads, &i));
                let block = BasicBlock {
                    instructions: self.instructions[head..next_jump].to_vec(),
                    continue_target,
                    jump_target,
                };
                blocks.push(block);
                target_index += 1;
            }
            head_index += 1;
        }

        (blocks, potential_targets)
    }

    fn address_to_index(&self, address: &RiscvAddress) -> usize {
        self.instructions
            .iter()
            .position(|inst| inst.address() == address)
            .expect(&format!("Unknown address `{}`", address))
    }

    fn find_basic_block_index(&self, heads: &Vec<usize>, index: &usize) -> usize {
        heads.iter().position(|head| head == index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::CfgBuilder;
    use crate::cfg::*;
    use crate::riscv_isa::RiscvInstruction::*;
    use crate::riscv_isa::RiscvRegister::*;
    use crate::riscv_parser;

    #[test]
    fn build_cfg() {
        let source = "
Disassembly of section .text:

0000000000010450 <s>:
   10484:	8782                	jr	a5
   10486:	0001                	nop
   1048e:	8082                	ret

0000000000010490 <f>:
   10490:	1141                	addi	sp,sp,-16
   10496:	feb50de3          	beq	a0,a1,10490 <f>
   1049a:	feb55be3          	bge	a0,a1,10490 <f>
   1049e:	feb579e3          	bgeu	a0,a1,10490 <f>
   104a2:	feb547e3          	blt	a0,a1,10490 <f>
   104a6:	feb565e3          	bltu	a0,a1,10490 <f>
   104aa:	feb513e3          	bne	a0,a1,10490 <f>
   104ae:	d16d                	beqz	a0,10490 <f>
   104b0:	f165                	bnez	a0,10490 <f>
   104b2:	bff9                	j	10490 <f>

00000000000104bc <main>:
   104c8:	f89ff0ef          	jal	ra,10450 <s>
   104cc:	00000097          	auipc	ra,0x0
   104d0:	fbe080e7          	jalr	-66(ra) # 104b8 <f>
   104da:	8082                	ret

Disassembly of section .rodata:

0000000000010538 <.rodata>:
   10538:	0486                	slli	s1,s1,0x1
   1053a:	0001                	nop
        ";
        let potential_targets = riscv_parser::parse_rodata(source);
        let riscv_insts = riscv_parser::parse_text(source);
        let cfg = CfgBuilder::new(riscv_insts, potential_targets).run();
        let expected = vec![
            RiscvFunction {
                name: String::from("s"),
                basic_blocks: vec![
                    BasicBlock {
                        instructions: vec![Jr {
                            label: Some(String::from("s")),
                            address: 66692,
                            rs1: A5,
                            comment: None,
                        }],
                        continue_target: None,
                        jump_target: None,
                    },
                    BasicBlock {
                        instructions: vec![
                            Nop {
                                label: None,
                                address: 66694,
                                comment: None,
                            },
                            Ret {
                                label: None,
                                address: 66702,
                                comment: None,
                            },
                        ],
                        continue_target: None,
                        jump_target: None,
                    },
                ],
                potential_targets: vec![1],
            },
            RiscvFunction {
                name: String::from("f"),
                basic_blocks: vec![
                    BasicBlock {
                        instructions: vec![
                            Addi {
                                label: Some(String::from("f")),
                                address: 66704,
                                rd: Sp,
                                rs1: Sp,
                                imm: -16,
                                comment: None,
                            },
                            Beq {
                                label: None,
                                address: 66710,
                                rs1: A0,
                                rs2: A1,
                                addr: 66704,
                                comment: Some(String::from(" <f>")),
                            },
                        ],
                        continue_target: Some(1),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![Bge {
                            label: None,
                            address: 66714,
                            rs1: A0,
                            rs2: A1,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: Some(2),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![Bgeu {
                            label: None,
                            address: 66718,
                            rs1: A0,
                            rs2: A1,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: Some(3),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![Blt {
                            label: None,
                            address: 66722,
                            rs1: A0,
                            rs2: A1,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: Some(4),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![Bltu {
                            label: None,
                            address: 66726,
                            rs1: A0,
                            rs2: A1,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: Some(5),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![Bne {
                            label: None,
                            address: 66730,
                            rs1: A0,
                            rs2: A1,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: Some(6),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![Beqz {
                            label: None,
                            address: 66734,
                            rs1: A0,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: Some(7),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![Bnez {
                            label: None,
                            address: 66736,
                            rs1: A0,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: Some(8),
                        jump_target: Some(0),
                    },
                    BasicBlock {
                        instructions: vec![J {
                            label: None,
                            address: 66738,
                            addr: 66704,
                            comment: Some(String::from(" <f>")),
                        }],
                        continue_target: None,
                        jump_target: Some(0),
                    },
                ],
                potential_targets: vec![],
            },
            RiscvFunction {
                name: String::from("main"),
                basic_blocks: vec![BasicBlock {
                    instructions: vec![
                        Jal {
                            label: Some(String::from("main")),
                            address: 66760,
                            rd: Ra,
                            addr: 66640,
                            comment: Some(String::from(" <s>")),
                        },
                        Auipc {
                            label: None,
                            address: 66764,
                            rd: Ra,
                            imm: 0,
                            comment: None,
                        },
                        Jalr {
                            label: None,
                            address: 66768,
                            rd: Ra,
                            rs1: Ra,
                            imm: -66,
                            comment: Some(String::from(" # 104b8 <f>")),
                        },
                        Ret {
                            label: None,
                            address: 66778,
                            comment: None,
                        },
                    ],
                    continue_target: None,
                    jump_target: None,
                }],
                potential_targets: vec![],
            },
        ];
        assert_eq!(cfg, expected);
    }
}
