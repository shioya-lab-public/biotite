use crate::cfg::{Cfg, Function, BasicBlock};
use crate::riscv_isa::{RiscvInstruction, RiscvAddress};
use std::mem;

pub struct CfgBuilder {
    instructions: Vec<RiscvInstruction>,
    links: Vec<(bool, Option<usize>, Option<usize>)>,
    cfg: Cfg,
}

impl CfgBuilder {
    pub fn new() -> Self {
        CfgBuilder {
            instructions: Vec::new(),
            links: Vec::new(),
            cfg: Cfg::new(),
        }
    }

    pub fn run(&mut self, instructions: Vec<RiscvInstruction>) -> Cfg {
        self.instructions = instructions;
        self.links = vec![(false, None, None); self.instructions.len()];
        self.cfg = Cfg::new();
        let index = self.instructions.iter().position(|inst| inst.label() == &Some(String::from("main"))).unwrap();
        self.build_function(index);
        mem::take(&mut self.cfg)
    }

    fn build_function(&mut self, index: usize) {
        let inst = &self.instructions[index];
        let name = inst.label().clone().unwrap();
        let address = *inst.address();
        self.find_basic_blocks(index);
        let basic_blocks = self.build_basic_blocks(index);
        let func = Function {
            name,
            address,
            basic_blocks,
        };
        self.cfg.push(func);
    }

    fn find_basic_blocks(&mut self, mut index: usize) {
        use RiscvInstruction::*;
        
        loop {
            let inst = &self.instructions[index];
            match inst {
                Beq { addr, .. }|
                Bge{ addr, ..}|
                Bgeu{ addr, ..}|
                Blt{ addr, ..}|
                Bltu{ addr, ..}|
                Bne{ addr, ..} =>  {
                    self.links[index].1 = Some(self.address_to_index(inst.address())+1);
                    let alt_target = self.address_to_index(addr);
                    self.links[index].2 = Some(alt_target);
                    if !self.links[alt_target].0 {
                        self.find_basic_blocks(self.address_to_index(addr));
                    } else if self.links[alt_target].1.is_none() && self.links[alt_target].2.is_none() {
                        let mut index = alt_target - 1;
                        while self.links[index].1.is_none() {
                            index -= 1;
                        }
                        self.links[alt_target] = self.links[index];
                        self.links[index].1 = Some(alt_target);
                        self.links[index].2 = None;
                    }
                }
                // Jal(rd, addr),
                // Jalr(rd, rs1, imm),
                // Beqz(rs1, addr),
                // Bnez(rs1, addr),
                // J(addr),
                // Jr(rs1),
                // Ret(),
                _ => {}
            }
            index += 1;
        }
    }

    fn build_basic_blocks(&mut self, index: usize) -> Vec<BasicBlock> {
        
    }

    fn address_to_index(&self, address: &RiscvAddress) -> usize {
        self.instructions.iter().position(|inst| inst.address() == address).expect(&format!("Unknown address `{}`", address))
    }
}
