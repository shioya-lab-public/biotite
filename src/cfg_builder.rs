use crate::cfg::{BasicBlock, Cfg, RiscvFunction};
use crate::riscv_isa::{RiscvAddress, RiscvInstruction};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::mem;

pub struct CfgBuilder {
    instructions: Vec<RiscvInstruction>,
    indirect_targets: HashMap<RiscvAddress, RiscvAddress>,
    functions: HashSet<String>,
    cfg: Cfg,
}

impl CfgBuilder {
    pub fn new(
        instructions: Vec<RiscvInstruction>,
        indirect_targets: HashMap<RiscvAddress, RiscvAddress>,
    ) -> Self {
        CfgBuilder {
            instructions,
            indirect_targets,
            functions: HashSet::new(),
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
        let (basic_blocks, indirect_targets) = self.build_basic_blocks(index);
        let func = RiscvFunction {
            name: name.to_string(),
            basic_blocks,
            indirect_targets,
        };
        self.cfg.push(func);
    }

    fn build_basic_blocks(
        &mut self,
        index: usize,
    ) -> (Vec<BasicBlock>, HashMap<RiscvAddress, usize>) {
        // Start indexes for basic blocks.
        let mut starts = vec![index];

        // Find basic blocks that are delimited by various jump instructions
        // and store their continue and jump targets.
        let mut targets = HashMap::new();
        let mut idx = index;
        while let Some(inst) = self.instructions.get(idx) {
            // Stop when we enter the next function.
            if idx != index && inst.label().is_some() {
                break;
            }

            use RiscvInstruction::*;

            match inst {
                Jal { comment, .. } | Jalr { comment, .. } => {
                    lazy_static! {
                        static ref FUNCTION: Regex = Regex::new(r"<(.+)>").unwrap();
                    }
                    let caps = FUNCTION.captures(comment.as_ref().unwrap()).unwrap();
                    let name = caps[1].to_string();
                    if !self.functions.contains(&name) {
                        self.build_function(&name);
                    }
                }
                Beq { addr, .. }
                | Bne { addr, .. }
                | Blt { addr, .. }
                | Bge { addr, .. }
                | Bltu { addr, .. }
                | Bgeu { addr, .. }
                | Beqz { addr, .. }
                | Bnez { addr, .. }
                | Blez { addr, .. }
                | Bgez { addr, .. }
                | Bltz { addr, .. }
                | Bgtz { addr, .. } => {
                    let index = self.address_to_index(addr);
                    targets.insert(idx + 1, (Some(idx + 1), Some(index)));
                    starts.extend(vec![idx + 1, index]);
                }
                J { addr, .. } => {
                    let index = self.address_to_index(addr);
                    targets.insert(idx + 1, (None, Some(index)));
                    starts.extend(vec![idx + 1, index]);
                }
                Jr { .. } | Ret { .. } => {
                    targets.insert(idx + 1, (None, None));
                    starts.push(idx + 1);
                }
                _ => {}
            }
            idx += 1;
        }

        // Find basic blocks that are delimited by indirect jump targets.
        let indirect_targets: HashMap<_, _> = self
            .indirect_targets
            .iter()
            .map(|(addr, target)| (addr.clone(), self.address_to_index(target)))
            .filter(|(_, i)| &index <= i && i < &idx)
            .collect();
        starts.extend(indirect_targets.values());
        starts.sort_unstable();
        starts.dedup();
        let indirect_targets: HashMap<_, _> = indirect_targets
            .into_iter()
            .map(|(addr, index)| (addr, self.find_basic_block_index(&starts, &index)))
            .collect();

        // Build basic blocks.
        let mut blocks = Vec::new();
        let mut start_iter = starts.iter();
        let mut start = *start_iter.next().unwrap();
        let mut end = start;
        for s in start_iter {
            start = end;
            end = *s;
            let block = match targets.get(&end) {
                Some((continue_target, jump_target)) => BasicBlock {
                    instructions: self.instructions[start..end].to_vec(),
                    continue_target: continue_target
                        .map(|i| self.find_basic_block_index(&starts, &i)),
                    jump_target: jump_target.map(|i| self.find_basic_block_index(&starts, &i)),
                },
                None => BasicBlock {
                    instructions: self.instructions[start..end].to_vec(),
                    continue_target: Some(self.find_basic_block_index(&starts, &end)),
                    jump_target: None,
                },
            };
            blocks.push(block);
        }

        (blocks, indirect_targets)
    }

    fn address_to_index(&self, address: &RiscvAddress) -> usize {
        self.instructions
            .iter()
            .position(|inst| inst.address() == address)
            .unwrap()
    }

    fn find_basic_block_index(&self, starts: &[usize], start: &usize) -> usize {
        starts.iter().position(|s| s == start).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::CfgBuilder;
    use crate::cfg::*;
    use crate::riscv_isa::RiscvInstruction::*;
    use crate::riscv_isa::RiscvRegister::*;
    use crate::riscv_parser;
    use std::collections::HashMap;

    #[test]
    fn minimal() {
        let source = "
            Disassembly of section .text:

            00000000000105c4 <main>:
                105c4:	8082                	ret
        ";
        let indirect_targets = riscv_parser::parse_rodata(source);
        let riscv_insts = riscv_parser::parse_text(source);
        let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
        let expected = vec![RiscvFunction {
            name: String::from("main"),
            basic_blocks: vec![BasicBlock {
                instructions: vec![Ret {
                    label: Some(String::from("main")),
                    address: 0x105c4.into(),
                    comment: None,
                }],
                continue_target: None,
                jump_target: None,
            }],
            indirect_targets: HashMap::new(),
        }];
        assert_eq!(cfg, expected);
    }

    #[test]
    fn functions() {
        let source = "
            Disassembly of section .text:

            0000000000010506 <f>:
                1051c:	8082                	ret
            
            000000000001051e <main>:
                1052a:	ff8080e7          	jalr	-8(ra) # 1051e <main>
                10530:	fdbff0ef          	jal	ra,104e0 <f>
                10534:	8082                	ret
        ";
        let indirect_targets = riscv_parser::parse_rodata(source);
        let riscv_insts = riscv_parser::parse_text(source);
        let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
        let expected = vec![
            RiscvFunction {
                name: String::from("f"),
                basic_blocks: vec![BasicBlock {
                    instructions: vec![Ret {
                        label: Some(String::from("f")),
                        address: 0x1051c.into(),
                        comment: None,
                    }],
                    continue_target: None,
                    jump_target: None,
                }],
                indirect_targets: HashMap::new(),
            },
            RiscvFunction {
                name: String::from("main"),
                basic_blocks: vec![BasicBlock {
                    instructions: vec![
                        Jalr {
                            label: Some(String::from("main")),
                            address: 0x1052a.into(),
                            rd: Ra,
                            imm: (-8).into(),
                            rs1: Ra,
                            comment: Some(String::from("# 1051e <main>")),
                        },
                        Jal {
                            label: None,
                            address: 0x10530.into(),
                            rd: Ra,
                            addr: 0x104e0.into(),
                            comment: Some(String::from("<f>")),
                        },
                        Ret {
                            label: None,
                            address: 0x10534.into(),
                            comment: None,
                        },
                    ],
                    continue_target: None,
                    jump_target: None,
                }],
                indirect_targets: HashMap::new(),
            },
        ];
        assert_eq!(cfg, expected);
    }

    #[test]
    fn branches() {
        let source = "
            Disassembly of section .text:

            00000000000104f8 <main>:
                104f8:	fe528de3          	beq	t0,t0,104f8 <main>
                10502:	fe529be3          	bne	t0,t0,104f8 <main>
                10506:	fe52c9e3          	blt	t0,t0,104f8 <main>
                1050a:	fe52d7e3          	bge	t0,t0,104f8 <main>
                1050e:	fe52e5e3          	bltu	t0,t0,104f8 <main>
                10512:	fe52f3e3          	bgeu	t0,t0,104f8 <main>
                10516:	fe0281e3          	beqz	t0,104f8 <main>
                1051a:	fc029fe3          	bnez	t0,104f8 <main>
                1051e:	fc505de3          	blez	t0,104f8 <main>
                10522:	fc02dbe3          	bgez	t0,104f8 <main>
                10526:	fc02c9e3          	bltz	t0,104f8 <main>
                1052a:	fc5047e3          	bgtz	t0,104f8 <main>
                10536:	8082                	ret
        ";
        let indirect_targets = riscv_parser::parse_rodata(source);
        let riscv_insts = riscv_parser::parse_text(source);
        let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
        let expected = vec![RiscvFunction {
            name: String::from("main"),
            basic_blocks: vec![
                BasicBlock {
                    instructions: vec![Beq {
                        label: Some(String::from("main")),
                        address: 0x104f8.into(),
                        rs1: T0,
                        rs2: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(1),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bne {
                        label: None,
                        address: 0x10502.into(),
                        rs1: T0,
                        rs2: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(2),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Blt {
                        label: None,
                        address: 0x10506.into(),
                        rs1: T0,
                        rs2: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(3),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bge {
                        label: None,
                        address: 0x1050a.into(),
                        rs1: T0,
                        rs2: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(4),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bltu {
                        label: None,
                        address: 0x1050e.into(),
                        rs1: T0,
                        rs2: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(5),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bgeu {
                        label: None,
                        address: 0x10512.into(),
                        rs1: T0,
                        rs2: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(6),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Beqz {
                        label: None,
                        address: 0x10516.into(),
                        rs1: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(7),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bnez {
                        label: None,
                        address: 0x1051a.into(),
                        rs1: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(8),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Blez {
                        label: None,
                        address: 0x1051e.into(),
                        rs1: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(9),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bgez {
                        label: None,
                        address: 0x10522.into(),
                        rs1: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(10),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bltz {
                        label: None,
                        address: 0x10526.into(),
                        rs1: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(11),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Bgtz {
                        label: None,
                        address: 0x1052a.into(),
                        rs1: T0,
                        addr: 0x104f8.into(),
                        comment: Some(String::from("<main>")),
                    }],
                    continue_target: Some(12),
                    jump_target: Some(0),
                },
                BasicBlock {
                    instructions: vec![Ret {
                        label: None,
                        address: 0x10536.into(),
                        comment: None,
                    }],
                    continue_target: None,
                    jump_target: None,
                },
            ],
            indirect_targets: HashMap::new(),
        }];
        assert_eq!(cfg, expected);
    }

    #[test]
    fn indirect_jumps() {
        let source = "
            Disassembly of section .text:

            00000000000104e0 <f>:
                104e0:	8782                	jr	a5
                1050e:	fec42783          	lw	a5,-20(s0)
                10512:	2785                	addiw	a5,a5,1
                10514:	fef42623          	sw	a5,-20(s0)
                10518:	a80d                	j	1054a <f+0x6a>
                1053e:	fec42783          	lw	a5,-20(s0)
                10542:	2795                	addiw	a5,a5,5
                10544:	fef42623          	sw	a5,-20(s0)
                10548:	0001                	nop
                1054a:	8082                	ret
            
            0000000000010556 <main>:
                10556:	f81ff0ef          	jal	ra,104e0 <f>
                1056e:	8082                	ret
            
            Disassembly of section .rodata:
            
            00000000000105cc <.rodata>:
                105cc:	054a                	slli	a0,a0,0x12
                105ce:	0001                	nop
                105d0:	050e                	slli	a0,a0,0x3
                105d2:	0001                	nop
                105e0:	053e                	slli	a0,a0,0xf
                105e2:	0001                	nop
        ";
        let indirect_targets = riscv_parser::parse_rodata(source);
        let riscv_insts = riscv_parser::parse_text(source);
        let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
        let expected = vec![
            RiscvFunction {
                name: String::from("f"),
                basic_blocks: vec![
                    BasicBlock {
                        instructions: vec![Jr {
                            label: Some(String::from("f")),
                            address: 0x104e0.into(),
                            rs1: A5,
                            comment: None,
                        }],
                        continue_target: None,
                        jump_target: None,
                    },
                    BasicBlock {
                        instructions: vec![
                            Lw {
                                label: None,
                                address: 0x1050e.into(),
                                rd: A5,
                                imm: (-20).into(),
                                rs1: S0,
                                comment: None,
                            },
                            Addiw {
                                label: None,
                                address: 0x10512.into(),
                                rd: A5,
                                rs1: A5,
                                imm: 1.into(),
                                comment: None,
                            },
                            Sw {
                                label: None,
                                address: 0x10514.into(),
                                rs2: A5,
                                imm: (-20).into(),
                                rs1: S0,
                                comment: None,
                            },
                            J {
                                label: None,
                                address: 0x10518.into(),
                                addr: 0x1054a.into(),
                                comment: Some(String::from("<f+0x6a>")),
                            },
                        ],
                        continue_target: None,
                        jump_target: Some(3),
                    },
                    BasicBlock {
                        instructions: vec![
                            Lw {
                                label: None,
                                address: 0x1053e.into(),
                                rd: A5,
                                imm: (-20).into(),
                                rs1: S0,
                                comment: None,
                            },
                            Addiw {
                                label: None,
                                address: 0x10542.into(),
                                rd: A5,
                                rs1: A5,
                                imm: 5.into(),
                                comment: None,
                            },
                            Sw {
                                label: None,
                                address: 0x10544.into(),
                                rs2: A5,
                                imm: (-20).into(),
                                rs1: S0,
                                comment: None,
                            },
                            Nop {
                                label: None,
                                address: 0x10548.into(),
                                comment: None,
                            },
                        ],
                        continue_target: Some(3),
                        jump_target: None,
                    },
                    BasicBlock {
                        instructions: vec![Ret {
                            label: None,
                            address: 0x1054a.into(),
                            comment: None,
                        }],
                        continue_target: None,
                        jump_target: None,
                    },
                ],
                indirect_targets: vec![
                    (0x105cc.into(), 3),
                    (0x105d0.into(), 1),
                    (0x105e0.into(), 2),
                ]
                .into_iter()
                .collect(),
            },
            RiscvFunction {
                name: String::from("main"),
                basic_blocks: vec![BasicBlock {
                    instructions: vec![
                        Jal {
                            label: Some(String::from("main")),
                            address: 0x10556.into(),
                            rd: Ra,
                            addr: 0x104e0.into(),
                            comment: Some(String::from("<f>")),
                        },
                        Ret {
                            label: None,
                            address: 0x1056e.into(),
                            comment: None,
                        },
                    ],
                    continue_target: None,
                    jump_target: None,
                }],
                indirect_targets: HashMap::new(),
            },
        ];
        assert_eq!(cfg, expected);
    }
}
