use crate::cfg::{BasicBlock, Cfg, Function};
use crate::riscv_isa::{RiscvAddress, RiscvInstruction};
use std::mem;

pub struct CfgBuilder {
    instructions: Vec<RiscvInstruction>,
    cfg: Cfg,
}

impl CfgBuilder {
    pub fn new() -> Self {
        CfgBuilder {
            instructions: Vec::new(),
            cfg: Cfg::new(),
        }
    }

    pub fn run(
        &mut self,
        instructions: Vec<RiscvInstruction>,
        potential_targets: Vec<RiscvAddress>,
    ) -> Cfg {
        self.instructions = instructions;
        self.cfg = Cfg::new();
        let index = self
            .instructions
            .iter()
            .position(|inst| inst.label() == &Some(String::from("main")))
            .unwrap();
        self.build_function(index, &potential_targets);
        mem::take(&mut self.cfg)
    }

    fn build_function(&mut self, index: usize, potential_targets: &Vec<RiscvAddress>) {
        let inst = &self.instructions[index];
        let name = inst.label().clone().unwrap();
        let address = inst.address().clone();
        let (basic_blocks, potential_targets) = self.build_basic_blocks(index, potential_targets);
        let func = Function {
            name,
            address,
            basic_blocks,
            potential_targets,
        };
        self.cfg.push(func);
    }

    fn build_basic_blocks(
        &mut self,
        index: usize,
        potential_targets: &Vec<RiscvAddress>,
    ) -> (Vec<BasicBlock>, Vec<usize>) {
        use RiscvInstruction::*;

        let mut targets = Vec::new();
        let mut crt = index;

        while let Some(inst) = self.instructions.get(crt) {
            // Stop when we find the next function.
            if crt != index && inst.label().is_some() {
                break;
            }

            // Find jump targets.
            match inst {
                Beq { addr, .. }
                | Bge { addr, .. }
                | Bgeu { addr, .. }
                | Blt { addr, .. }
                | Bltu { addr, .. }
                | Bne { addr, .. }
                | Beqz { addr, .. }
                | Bnez { addr, .. }
                | J { addr, .. } => {
                    let index = self.address_to_index(addr);
                    targets.push((crt + 1, index));
                }
                Ret { .. } => targets.push((crt + 1, 0)),
                // Function Calls
                Jal { addr, .. } => {
                    let index = self.address_to_index(addr);
                    self.build_function(index, potential_targets);
                }
                // Indirect function calls (`Jalr(rd, rs1, imm)`) will be handled by the translator.
                Jr { .. } => targets.push((crt + 1, 0)),
                _ => {}
            }

            crt += 1;
        }

        // Select potential targets that lays within this functions.
        let potential_targets: Vec<_> = potential_targets
            .iter()
            .map(|addr| self.address_to_index(addr))
            .skip_while(|i| i < &index)
            .take_while(|i| i < &crt)
            .collect();

        // Find basic blocks.
        let mut heads = vec![index];
        for (continue_target, jump_target) in targets.clone() {
            heads.push(continue_target);
            heads.push(jump_target);
        }
        heads.extend(
            potential_targets
                .iter()
                .map(|addr| self.address_to_index(addr)),
        );
        heads.sort();
        heads.dedup();
        heads.pop();
        heads.remove(0);

        // Build basic blocks.
        let mut blocks = Vec::new();
        let mut head_index = 0;
        let mut target_index = 0;
        while head_index < heads.len() {
            let head = heads[head_index];
            let next_head = heads[head_index];
            let next_jump = targets[target_index].0;
            if next_head < next_jump {
                let block = BasicBlock {
                    instructions: self.instructions[head..next_head].to_vec(),
                    continue_target: head_index + 1,
                    jump_target: 0,
                };
                blocks.push(block);
                head_index += 1;
            } else {
                let block = BasicBlock {
                    instructions: self.instructions[head..next_jump].to_vec(),
                    continue_target: self.find_basic_block_index(&heads, &next_jump),
                    jump_target: self.find_basic_block_index(&heads, &targets[target_index].1),
                };
                blocks.push(block);
                head_index += 1;
                target_index += 1;
            }
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
        for (i, head) in heads.iter().enumerate() {
            if index == head {
                return i;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::CfgBuilder;
    use crate::riscv_isa::RiscvInstruction::*;
    use crate::riscv_isa::RiscvRegister::*;
    use crate::riscv_parser;

    #[test]
    fn build_cfg() {
        let source = "
        Disassembly of section .text:

        0000000000010450 <f>:
            10450:	1141                	addi	sp,sp,-16
            10452:	e422                	sd	s0,8(sp)
            10454:	0800                	addi	s0,sp,16
            10456:	0001                	nop
            10458:	6422                	ld	s0,8(sp)
            1045a:	0141                	addi	sp,sp,16
            1045c:	8082                	ret
        
        000000000001045e <main>:
            1045e:	1141                	addi	sp,sp,-16
            10460:	e406                	sd	ra,8(sp)
            10462:	e022                	sd	s0,0(sp)
            10464:	0800                	addi	s0,sp,16
            10466:	febff0ef          	jal	ra,10450 <f>
            1046a:	4781                	li	a5,0
            1046c:	853e                	mv	a0,a5
            1046e:	60a2                	ld	ra,8(sp)
            10470:	6402                	ld	s0,0(sp)
            10472:	0141                	addi	sp,sp,16
            10474:	8082                	ret
        ";
        let insts = riscv_parser::parse_text(source.to_string());
        // let mut cfg_builder= CfgBuilder::new();
        // let cfg = cfg_builder.run(insts);
        // let expected = vec![

        // ];
        // assert_eq!(cfg, expected);
    }
}
