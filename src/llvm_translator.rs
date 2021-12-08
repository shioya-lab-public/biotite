use crate::llvm_isa::{CodeBlock, Instruction, InstructionBlock, Program, Type, Value};
use crate::llvm_macro::*;
use crate::riscv_isa::{
    Abi, Address, CodeBlock as RiscvCodeBlock, Immediate, Instruction as RiscvInstruction,
    Program as RiscvProgram, Register,
};

pub struct Translator {
    abi: Abi,
    entry: Address,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            abi: Abi::default(),
            entry: Address(0x0),
        }
    }

    pub fn run(&mut self, rv_program: RiscvProgram) -> Program {
        self.abi = rv_program.abi;
        self.entry = Address(0x0);

        let code_blocks = rv_program
            .code_blocks
            .into_iter()
            .map(|b| self.translate_code_block(b))
            .collect();

        Program {
            abi: self.abi,
            entry: self.entry,
            data_blocks: rv_program.data_blocks,
            code_blocks,
        }
    }

    fn translate_code_block(&mut self, rv_code_block: RiscvCodeBlock) -> CodeBlock {
        if let "_start" = rv_code_block.symbol.as_str() {
            self.entry = rv_code_block.address;
        }
        let instruction_blocks = rv_code_block
            .instructions
            .into_iter()
            .map(|i| self.translate_instruction(i))
            .collect();
        CodeBlock {
            section: rv_code_block.section,
            symbol: rv_code_block.symbol,
            address: rv_code_block.address,
            instruction_blocks,
        }
    }

    fn translate_instruction(&mut self, rv_inst: RiscvInstruction) -> InstructionBlock {
        use Instruction::*;
        use RiscvInstruction as RI;

        let insts = match &rv_inst {
            // RV32I
            RI::Lui { address, rd, imm } => build_instructions! { address, self.abi,
                Shl { rslt: _0, ty: _i, op1: imm, op2: imm_12 },
                Store { ty: _i, val: _0, ptr: rd },
            },
            RI::Auipc { address, rd, imm } => build_instructions! { address, self.abi,
                Shl { rslt: _0, ty: _i, op1: imm, op2: imm_12 },
                Add { rslt: _1, ty: _i, op1: _0, op2: address },
                Store { ty: _i, val: _1, ptr: rd },
            },

            RI::Addi {
                address,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                Store { ty: _i, val: _1, ptr: rd },
            },

            RI::J { address, addr } => build_instructions! { address, self.abi,
                UnconBr { addr: addr },
            },

            _ => todo!(),
        };

        let insts = insts
            .into_iter()
            .filter_map(|inst| match inst {
                Load {
                    rslt,
                    ty,
                    ptr: Value::Register(Register::Zero),
                } => Some(Add {
                    rslt,
                    ty,
                    op1: Value::Immediate(Immediate(0)),
                    op2: Value::Immediate(Immediate(0)),
                }),
                Store {
                    ptr: Value::Register(Register::Zero),
                    ..
                } => None,
                inst => Some(inst),
            })
            .collect();

        InstructionBlock {
            riscv_instruction: rv_inst,
            instructions: insts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Translator;
    use crate::llvm_isa::{CodeBlock, Instruction, InstructionBlock, Type, Value};
    use crate::riscv_isa::{
        Abi, Address, CodeBlock as RiscvCodeBlock, DataBlock, Immediate,
        Instruction as RiscvInstruction, Program as RiscvProgram, Register,
    };
    use std::fs;

    macro_rules! build_tests {
        ( $( $func:ident ( $rv_inst:expr, $( $inst:literal, )* ), )* ) => {
            $(
                #[test]
                fn $func() {
                    use RiscvInstruction as RI;

                    let rv_program = RiscvProgram {
                        abi: Abi::Lp64d,
                        data_blocks: Vec::new(),
                        code_blocks: vec![RiscvCodeBlock {
                            section: String::from(".text"),
                            symbol: String::from("_start"),
                            address: Address(0x0),
                            instructions: vec![$rv_inst],
                        }],
                    };
                    let program = Translator::new().run(rv_program);
                    let inst_strs: Vec<_> = program.code_blocks[0].instruction_blocks[0]
                        .instructions
                        .iter()
                        .map(|i| i.to_string())
                        .collect();
                    assert_eq!(inst_strs, vec![$($inst, )*]);
                }
            )*
        };
    }

    #[test]
    fn find_entry() {
        let rv_program = RiscvProgram {
            abi: Abi::default(),
            data_blocks: Vec::new(),
            code_blocks: vec![RiscvCodeBlock {
                section: String::from(".text"),
                symbol: String::from("_start"),
                address: Address(0x4),
                instructions: Vec::new(),
            }],
        };
        let program = Translator::new().run(rv_program);
        assert_eq!(program.entry, Address(0x4));
    }

    #[test]
    fn enforce_zero() {
        use Instruction::*;
        use RiscvInstruction as RI;

        let rv_program = RiscvProgram {
            abi: Abi::Lp64d,
            data_blocks: Vec::new(),
            code_blocks: vec![RiscvCodeBlock {
                section: String::from(".text"),
                symbol: String::from("_start"),
                address: Address(0x0),
                instructions: vec![RI::Addi {
                    address: Address(0x0),
                    rd: Register::Zero,
                    rs1: Register::Zero,
                    imm: Immediate(0),
                }],
            }],
        };
        let program = Translator::new().run(rv_program);
        assert_eq!(
            program.code_blocks,
            vec![CodeBlock {
                section: String::from(".text"),
                symbol: String::from("_start"),
                address: Address(0x0),
                instruction_blocks: vec![InstructionBlock {
                    riscv_instruction: RI::Addi {
                        address: Address(0x0),
                        rd: Register::Zero,
                        rs1: Register::Zero,
                        imm: Immediate(0)
                    },
                    instructions: vec![
                        Add {
                            rslt: Value::Temp(Address(0x0), 0),
                            ty: Type::I64,
                            op1: Value::Immediate(Immediate(0)),
                            op2: Value::Immediate(Immediate(0))
                        },
                        Add {
                            rslt: Value::Temp(Address(0x0), 1),
                            ty: Type::I64,
                            op1: Value::Temp(Address(0x0), 0),
                            op2: Value::Immediate(Immediate(0))
                        },
                    ],
                }],
            }]
        );
    }

    #[test]
    fn format() {
        use RiscvInstruction as RI;

        let abis = vec![
            Abi::Ilp32,
            Abi::Ilp32f,
            Abi::Ilp32d,
            Abi::Lp64,
            Abi::Lp64f,
            Abi::Lp64d,
        ];
        for abi in abis {
            let rv_program = RiscvProgram {
                abi,
                data_blocks: vec![DataBlock {
                    section: String::from(".data"),
                    symbol: String::from(".data"),
                    address: Address(0x0),
                    bytes: vec![0x3, 0x2, 0x1, 0x0],
                }],
                code_blocks: vec![RiscvCodeBlock {
                    section: String::from(".text"),
                    symbol: String::from("_start"),
                    address: Address(0x0),
                    instructions: vec![RI::J {
                        address: Address(0x0),
                        addr: Address(0x0),
                    }],
                }],
            };
            let program_str = format!("{}", Translator::new().run(rv_program));
            let ref_program_str = fs::read_to_string(format!("./tests/{}.ref.ll", abi))
                .expect("Cannot open the reference output file");
            assert_eq!(program_str, ref_program_str);
        }
    }

    build_tests! {
        lui(RI::Lui { address: Address(0x0), rd: Register::T0, imm: Immediate(4) },
            "%t_0_0 = shl i64 4, 12",
            "store i64 %t_0_0, i64* %t0",
        ),
    }
}
