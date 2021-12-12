use crate::llvm_isa::{
    CodeBlock, Condition, FPType, Instruction, InstructionBlock, Program, Type, Value,
};
use crate::llvm_macro::*;
use crate::riscv_isa::{
    Abi, Address, CodeBlock as RiscvCodeBlock, DataBlock, Immediate,
    Instruction as RiscvInstruction, Program as RiscvProgram, Raw, Register,
};
use std::collections::HashMap;
use std::mem;

pub struct Translator {
    abi: Abi,
    entry: Address,
    data_blocks: Vec<DataBlock>,
    targets: Vec<Address>,
    sp: Address,
    fp: Address,
    stack: HashMap<Address, Vec<Type>>,
    fpstack: HashMap<Address, Vec<FPType>>,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            abi: Abi::default(),
            entry: Address(0x0),
            data_blocks: Vec::new(),
            targets: Vec::new(),
            sp: Address(u64::MAX),
            fp: Address(u64::MAX),
            stack: HashMap::new(),
            fpstack: HashMap::new(),
        }
    }

    pub fn run(&mut self, rv_program: RiscvProgram) -> Program {
        self.abi = rv_program.abi;
        self.entry = Address(0x0);
        self.data_blocks = rv_program.data_blocks;
        self.targets.clear();
        self.sp = Address(u64::MAX);
        self.fp = Address(u64::MAX);

        for code_block in rv_program.code_blocks.iter() {
            self.targets.push(code_block.address);
        }

        let code_blocks = rv_program
            .code_blocks
            .into_iter()
            .map(|b| self.translate_code_block(b))
            .collect();

        Program {
            abi: self.abi,
            entry: self.entry,
            data_blocks: mem::take(&mut self.data_blocks),
            code_blocks,
            stack: mem::take(&mut self.stack),
            fpstack: mem::take(&mut self.fpstack),
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
            RI::Lui {
                address,
                raw,
                rd,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Shl { rslt: _0, ty: _i, op1: imm, op2: imm_12 },
                Store { ty: _i, val: _0, ptr: rd },
            },
            RI::Auipc {
                address,
                raw,
                rd,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Shl { rslt: _0, ty: _i, op1: imm, op2: imm_12 },
                Add { rslt: _1, ty: _i, op1: _0, op2: address },
                Store { ty: _i, val: _1, ptr: rd },
            },
            RI::Jal {
                address,
                raw,
                rd,
                addr,
            } => build_instructions! { address, raw, self.abi,
                Store { ty: _i, val: next_pc, ptr: rd },
                UnconBr { addr: addr },
            },
            RI::Bge {
                address,
                raw,
                rs1,
                rs2,
                addr,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: sge, ty: _i, op1: _0, op2: _1 },
                ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
            },
            RI::Bltu {
                address,
                raw,
                rs1,
                rs2,
                addr,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: ult, ty: _i, op1: _0, op2: _1 },
                ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
            },
            RI::Lw {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                Register::Sp | Register::S0 => {
                    let stk = match rs1 {
                        Register::Sp => {
                            let Address(addr) = self.sp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        Register::S0 => {
                            let Address(addr) = self.fp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        _ => unreachable!(),
                    };
                    let ver = (self.stack[&stk].len() - 1) as i64;
                    build_instructions! { address, raw, self.abi,
                        Load { rslt: _0, ty: _i, ptr: rs1 },
                        Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                        Loadstack { rslt: _2, ty: _i32, stk: stk, ver: ver },
                        Sext { rslt: _3, ty: _i32, val: _2, ty2: _i },
                        Store { ty: _i, val: _3, ptr: rd },
                    }
                }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i32 },
                    Load { rslt: _4, ty: _i32, ptr: _3 },
                    Sext { rslt: _5, ty: _i32, val: _4, ty2: _i },
                    Store { ty: _i, val: _5, ptr: rd },
                },
            },
            RI::Sw {
                address,
                raw,
                rs2,
                imm,
                rs1,
            } => match rs1 {
                Register::Sp | Register::S0 => {
                    let stk = match rs1 {
                        Register::Sp => {
                            let Address(addr) = self.sp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        Register::S0 => {
                            let Address(addr) = self.fp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        _ => unreachable!(),
                    };
                    self.stack.entry(stk).or_default().push(Type::I32);
                    let ver = (self.stack[&stk].len() - 1) as i64;
                    build_instructions! { address, raw, self.abi,
                        Load { rslt: _0, ty: _i, ptr: rs2 },
                        Trunc { rslt: _1, ty: _i, val: _0, ty2: _i32 },
                        Storestack { ty: _i32, val: _1, stk: stk, ver: ver },
                    }
                }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs2 },
                    Trunc { rslt: _1, ty: _i, val: _0, ty2: _i32 },
                    Load { rslt: _2, ty: _i, ptr: rs1 },
                    Add { rslt: _3, ty: _i, op1: _2, op2: imm },
                    Getdataptr { rslt: _4, ty: _i, addr: _3 },
                    Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i32 },
                    Store { ty: _i32, val: _1, ptr: _5 },
                },
            },
            RI::Slli {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Shl { rslt: _1, ty: _i, op1: _0, op2: imm },
                Store { ty: _i, val: _1, ptr: rd },
            },
            RI::Add {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Add { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Addw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Add { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Addiw {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                Store { ty: _i, val: _1, ptr: rd },
            },

            // RV64I
            RI::Lwu {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                Register::Sp | Register::S0 => {
                    let stk = match rs1 {
                        Register::Sp => {
                            let Address(addr) = self.sp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        Register::S0 => {
                            let Address(addr) = self.fp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        _ => unreachable!(),
                    };
                    let ver = (self.stack[&stk].len() - 1) as i64;
                    build_instructions! { address, raw, self.abi,
                        Load { rslt: _0, ty: _i, ptr: rs1 },
                        Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                        Loadstack { rslt: _2, ty: _i32, stk: stk, ver: ver },
                        Zext { rslt: _3, ty: _i32, val: _2, ty2: _i },
                        Store { ty: _i, val: _3, ptr: rd },
                    }
                }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i32 },
                    Load { rslt: _4, ty: _i32, ptr: _3 },
                    Zext { rslt: _5, ty: _i32, val: _4, ty2: _i },
                    Store { ty: _i, val: _5, ptr: rd },
                },
            },
            RI::Ld {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                Register::Sp | Register::S0 => {
                    let stk = match rs1 {
                        Register::Sp => {
                            let Address(addr) = self.sp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        Register::S0 => {
                            let Address(addr) = self.fp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        _ => unreachable!(),
                    };
                    let ver = (self.stack[&stk].len() - 1) as i64;
                    build_instructions! { address, raw, self.abi,
                        Load { rslt: _0, ty: _i, ptr: rs1 },
                        Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                        Loadstack { rslt: _2, ty: _i64, stk: stk, ver: ver },
                        Store { ty: _i, val: _2, ptr: rd },
                    }
                }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i32 },
                    Load { rslt: _4, ty: _i64, ptr: _3 },
                    Store { ty: _i, val: _5, ptr: rd },
                },
            },
            RI::Sd {
                address,
                raw,
                rs2,
                imm,
                rs1,
            } => match rs1 {
                Register::Sp | Register::S0 => {
                    let stk = match rs1 {
                        Register::Sp => {
                            let Address(addr) = self.sp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        Register::S0 => {
                            let Address(addr) = self.fp;
                            let Immediate(imm) = *imm;
                            if imm >= 0 {
                                Address(addr + imm as u64)
                            } else {
                                Address(addr - (-imm) as u64)
                            }
                        }
                        _ => unreachable!(),
                    };
                    self.stack.entry(stk).or_default().push(Type::I64);
                    let ver = (self.stack[&stk].len() - 1) as i64;
                    build_instructions! { address, raw, self.abi,
                        Load { rslt: _0, ty: _i, ptr: rs2 },
                        Storestack { ty: _i64, val: _0, stk: stk, ver: ver },
                    }
                }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs2 },
                    Load { rslt: _1, ty: _i, ptr: rs1 },
                    Add { rslt: _2, ty: _i, op1: _1, op2: imm },
                    Getdataptr { rslt: _3, ty: _i, addr: _2 },
                    Bitcast { rslt: _4, ty: _i8, val: _3, ty2: _i64 },
                    Store { ty: _i64, val: _0, ptr: _4 },
                },
            },

            RI::Addi {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => {
                match (rd, rs1, imm) {
                    (Register::Sp, Register::Sp, Immediate(imm)) => {
                        let Address(sp) = self.sp;
                        let imm = *imm;
                        if imm >= 0 {
                            self.sp = Address(sp + imm as u64)
                        } else {
                            self.sp = Address(sp - (-imm) as u64)
                        }
                    }
                    (Register::S0, Register::Sp, Immediate(imm)) => {
                        let Address(sp) = self.sp;
                        let Address(fp) = self.fp;
                        let imm = *imm;
                        if imm >= 0 {
                            self.fp = Address(sp + imm as u64)
                        } else {
                            self.fp = Address(sp - (-imm) as u64)
                        }
                    }
                    _ => {}
                };
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Store { ty: _i, val: _1, ptr: rd },
                }
            }

            // Pseudoinstructions
            RI::Mv {
                address,
                raw,
                rd,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Store { ty: _i, val: _0, ptr: rd },
            },
            RI::SextW {
                address,
                raw,
                rd,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Store { ty: _i, val: _0, ptr: rd },
            },
            RI::Li {
                address,
                raw,
                rd,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Store { ty: _i, val: imm, ptr: rd },
            },

            RI::J { address, raw, addr } => build_instructions! { address, raw, self.abi,
                UnconBr { addr: addr },
            },
            RI::Nop { .. } => Vec::new(),
            RI::Ret { address, raw } => {
                let default = Value::Address(Address(0x1));
                let targets = self.targets.clone().into_iter().map(|t| Value::Address(t)).collect();
                build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: ra },
                Switch { ty: _i, val: _0, default: default, targets: targets },
            }}
            RI::Jr { address, raw, rs1 } => {
                let default = Value::Address(Address(0x1));
                let targets = self.targets.clone().into_iter().map(|t| Value::Address(t)).collect();
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Switch { ty: _i, val: _0, default: default, targets: targets },
                }
            }

            inst => todo!("{:?}", inst),
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
        Instruction as RiscvInstruction, Program as RiscvProgram, Raw, Register,
    };
    use std::fs;

    macro_rules! build_tests {
        ( $( $func:ident ( $rv_inst:ident { $( $field:ident: $value:expr ),* }, $( $inst:literal, )* ), )* ) => {
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
                            instructions: vec![
                                RI::$rv_inst {
                                    address: Address(0x0),
                                    raw: Raw::new(""),
                                    $(
                                        $field: $value,
                                    )*
                                },
                            ],
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
                    raw: Raw::new(""),
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
                        raw: Raw::new(""),
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
                        raw: Raw::new(""),
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
        lui(Lui { rd: Register::T0, imm: Immediate(4) },
            "%t_0_0 = shl i64 4, 12",
            "store i64 %t_0_0, i64* %t0",
        ),
    }
}
