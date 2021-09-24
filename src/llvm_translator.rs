use crate::cfg::{BasicBlock, Cfg, RiscvFunction};
use crate::llvm_isa::{LlvmFunction, LlvmInstruction, LlvmType, LlvmValue, Program};
use crate::riscv_isa::{RiscvAddress, RiscvInstruction};
use std::collections::HashMap;
use std::mem;

pub struct LlvmTranslator {
    cfg: Cfg,
    statics: HashMap<String, String>,
    temp: usize,
}

impl LlvmTranslator {
    pub fn new(cfg: Cfg, statics: HashMap<String, String>) -> Self {
        LlvmTranslator {
            cfg,
            statics,
            temp: 0,
        }
    }

    pub fn run(&mut self) -> Program {
        let functions = mem::take(&mut self.cfg)
            .into_iter()
            .map(|f| self.translate_function(f))
            .collect();
        Program {
            statics: mem::take(&mut self.statics),
            functions,
        }
    }

    fn get_temps(&mut self, n: usize) -> Vec<LlvmValue> {
        let temps = (self.temp..self.temp + n).map(|t| t.into()).collect();
        self.temp += n;
        temps
    }

    fn translate_function(
        &mut self,
        RiscvFunction {
            name,
            basic_blocks,
            indirect_targets,
        }: RiscvFunction,
    ) -> LlvmFunction {
        self.temp = 0;
        let mut body: Vec<_> = basic_blocks
            .into_iter()
            .enumerate()
            .map(|(i, block)| {
                let mut insts = vec![LlvmInstruction::Label(format!("L{}", i))];
                insts.extend(self.translate_basic_block(block, &indirect_targets));
                insts
            })
            .flatten()
            .collect();
        body.insert(0, LlvmInstruction::UnconBr(String::from("L0")));
        body.insert(0, LlvmInstruction::Label(String::from("Entry")));
        LlvmFunction { name, body }
    }

    fn translate_basic_block(
        &mut self,
        BasicBlock {
            instructions,
            continue_target,
            jump_target,
        }: BasicBlock,
        indirect_targets: &HashMap<RiscvAddress, usize>,
    ) -> Vec<LlvmInstruction> {
        use LlvmInstruction as LI;
        use RiscvInstruction as RI;

        instructions
            .into_iter()
            .map(|inst| match inst {
                // RV32I
                RI::Lui { rd, imm, .. } => {
                    let temps = self.get_temps(1);
                    vec![
                        LI::Shl {
                            result: temps[0].clone(),
                            ty: LlvmType::I64,
                            op1: imm.into(),
                            op2: 12_i64.into(),
                        },
                        LI::Store {
                            ty: LlvmType::I64,
                            value: temps[0].clone(),
                            pointer: rd.into(),
                        },
                    ]
                }
                RI::Auipc { .. } => Vec::new(),

                _ => vec![],
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::LlvmTranslator;
    use crate::cfg_builder::CfgBuilder;
    use crate::llvm_isa::{LlvmFunction, LlvmInstruction::*, LlvmType, Program};
    use crate::riscv_isa::RiscvRegister::*;
    use crate::riscv_parser;
    use std::collections::HashMap;

    macro_rules! build_test {
        ( $( $func:ident ( $source:literal $(, $inst:expr )* ), )* ) => {
            $(
                #[test]
                fn $func() {
                    let source = format!("
                        Disassembly of section .text:
                        0000000000010556 <main>:
                            10556:	1141                	{}
                            10598:	8082                	ret
                    ", $source);
                    let indirect_targets = riscv_parser::parse_rodata(&source);
                    let mut statics = riscv_parser::parse_sdata(&source);
                    statics.extend(riscv_parser::parse_sbss(&source));
                    let rv_insts = riscv_parser::parse_text(&source);
                    let cfg = CfgBuilder::new(rv_insts, indirect_targets).run();
                    let ll_program = LlvmTranslator::new(cfg, statics).run();
                    let expected = Program{
                        statics: HashMap::new(),
                        functions: vec![LlvmFunction {
                            name: String::from("main"),
                            body: vec![
                                Label(String::from("Entry")),
                                UnconBr(String::from("L0")),
                                Label(String::from("L0")),
                                $(
                                    $inst,
                                )*
                            ],
                        }]
                    };
                    assert_eq!(ll_program, expected);
                }
            )*
        };
    }

    build_test! {
        lui("lui	a0,0x12",
            Shl {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                op1: 0x12_i64.into(),
                op2: 12_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 0_usize.into(),
                pointer: A0.into(),
            }
        ),
        auipc("auipc	a0,0x0"),
    }
}
