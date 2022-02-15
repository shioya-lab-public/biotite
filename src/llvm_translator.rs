use crate::llvm_isa::{
    CodeBlock, Condition, Instruction, InstructionBlock, Ordering, Program, Type, Value,
};
use crate::llvm_macro::*;
use crate::riscv_isa::{
    Abi, Address, CodeBlock as RiscvCodeBlock, DataBlock, Immediate,
    Instruction as RiscvInstruction, Program as RiscvProgram, Raw, Register,
};
use std::collections::{HashMap, HashSet};
use std::mem;

pub struct Translator {
    abi: Abi,
    entry: Address,
    data_blocks: Vec<DataBlock>,
    targets: Vec<Address>,
    sp: Address,
    fp: Address,
    stack: HashMap<Address, HashSet<Type>>,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            abi: Abi::default(),
            entry: Address(0x0),
            data_blocks: Vec::new(),
            targets: Vec::new(),
            sp: Address(10_000000),
            fp: Address(10_000000),
            stack: HashMap::new(),
        }
    }

    pub fn run(&mut self, rv_program: RiscvProgram) -> Program {
        self.abi = rv_program.abi;
        self.entry = Address(0x0);
        self.data_blocks = rv_program.data_blocks;
        self.targets.clear();
        self.sp = Address(10_000000);
        self.fp = Address(10_000000);

        let stack = DataBlock {
            section: String::from(".stack"),
            symbol: String::from("stack"),
            address: Address(0),
            bytes: vec![0;1024],
        };
        self.data_blocks.push(stack);

        for code_block in rv_program.code_blocks.iter() {
            self.targets.push(code_block.address);
        }
        self.targets.push(Address(0x0));

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

    #[allow(unused_variables)]
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
            RI::Jalr {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => {
                let default = Value::Address(Address(0x1));
                let targets = self
                    .targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                build_instructions! { address, raw, self.abi,
                    Store { ty: _i, val: next_pc, ptr: rd },
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Switch { ty: _i, val: _1, dflt: default, tgts: targets },
                }
            }
            RI::ImplicitJalr {
                address,
                raw,
                imm,
                rs1,
            } => {
                let default = Value::Address(Address(0x1));
                let targets = self
                    .targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                let rs2 = &Register::Ra;
                build_instructions! { address, raw, self.abi,
                    Store { ty: _i, val: next_pc, ptr: rs2 },
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Switch { ty: _i, val: _1, dflt: default, tgts: targets },
                }
            }
            RI::Beq {
                address,
                raw,
                rs1,
                rs2,
                addr,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: eq, ty: _i, op1: _0, op2: _1 },
                ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
            },
            RI::Bne {
                address,
                raw,
                rs1,
                rs2,
                addr,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: ne, ty: _i, op1: _0, op2: _1 },
                ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
            },
            RI::Blt {
                address,
                raw,
                rs1,
                rs2,
                addr,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: slt, ty: _i, op1: _0, op2: _1 },
                ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
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
            RI::Bgeu {
                address,
                raw,
                rs1,
                rs2,
                addr,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: uge, ty: _i, op1: _0, op2: _1 },
                ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
            },
            RI::Lb {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     build_instructions! { address, raw, self.abi,
                //         Loadstack { rslt: _0, ty: _i8, stk: stk },
                //         Sext { rslt: _1, ty: _i8, val: _0, ty2: _i },
                //         Store { ty: _i, val: _1, ptr: rd },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i8 },
                    Load { rslt: _4, ty: _i8, ptr: _3 },
                    Sext { rslt: _5, ty: _i8, val: _4, ty2: _i },
                    Store { ty: _i, val: _5, ptr: rd },
                },
            },
            RI::Lh {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     build_instructions! { address, raw, self.abi,
                //         Loadstack { rslt: _0, ty: _i16, stk: stk },
                //         Sext { rslt: _1, ty: _i16, val: _0, ty2: _i },
                //         Store { ty: _i, val: _1, ptr: rd },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i16 },
                    Load { rslt: _4, ty: _i16, ptr: _3 },
                    Sext { rslt: _5, ty: _i16, val: _4, ty2: _i },
                    Store { ty: _i, val: _5, ptr: rd },
                },
            },
            RI::Lw {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     build_instructions! { address, raw, self.abi,
                //         Loadstack { rslt: _0, ty: _i32, stk: stk },
                //         Sext { rslt: _1, ty: _i32, val: _0, ty2: _i },
                //         Store { ty: _i, val: _1, ptr: rd },
                //     }
                // }
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
            RI::Lbu {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     build_instructions! { address, raw, self.abi,
                //         Loadstack { rslt: _0, ty: _i8, stk: stk },
                //         Zext { rslt: _1, ty: _i8, val: _0, ty2: _i },
                //         Store { ty: _i, val: _1, ptr: rd },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i8 },
                    Load { rslt: _4, ty: _i8, ptr: _3 },
                    Zext { rslt: _5, ty: _i8, val: _4, ty2: _i },
                    Store { ty: _i, val: _5, ptr: rd },
                },
            },
            RI::Lhu {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     build_instructions! { address, raw, self.abi,
                //         Loadstack { rslt: _0, ty: _i16, stk: stk },
                //         Zext { rslt: _1, ty: _i16, val: _0, ty2: _i },
                //         Store { ty: _i, val: _1, ptr: rd },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i16 },
                    Load { rslt: _4, ty: _i16, ptr: _3 },
                    Zext { rslt: _5, ty: _i16, val: _4, ty2: _i },
                    Store { ty: _i, val: _5, ptr: rd },
                },
            },
            RI::Sb {
                address,
                raw,
                rs2,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     self.stack.entry(stk).or_default().insert(Type::I8);
                //     build_instructions! { address, raw, self.abi,
                //         Load { rslt: _0, ty: _i, ptr: rs2 },
                //         Trunc { rslt: _1, ty: _i, val: _0, ty2: _i8 },
                //         Storestack { ty: _i8, val: _1, stk: stk },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs2 },
                    Trunc { rslt: _1, ty: _i, val: _0, ty2: _i8 },
                    Load { rslt: _2, ty: _i, ptr: rs1 },
                    Add { rslt: _3, ty: _i, op1: _2, op2: imm },
                    Getdataptr { rslt: _4, ty: _i, addr: _3 },
                    Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i8 },
                    Store { ty: _i8, val: _1, ptr: _5 },
                },
            },
            RI::Sh {
                address,
                raw,
                rs2,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     self.stack.entry(stk).or_default().insert(Type::I16);
                //     build_instructions! { address, raw, self.abi,
                //         Load { rslt: _0, ty: _i, ptr: rs2 },
                //         Trunc { rslt: _1, ty: _i, val: _0, ty2: _i16 },
                //         Storestack { ty: _i16, val: _1, stk: stk },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs2 },
                    Trunc { rslt: _1, ty: _i, val: _0, ty2: _i16 },
                    Load { rslt: _2, ty: _i, ptr: rs1 },
                    Add { rslt: _3, ty: _i, op1: _2, op2: imm },
                    Getdataptr { rslt: _4, ty: _i, addr: _3 },
                    Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i16 },
                    Store { ty: _i16, val: _1, ptr: _5 },
                },
            },
            RI::Sw {
                address,
                raw,
                rs2,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     self.stack.entry(stk).or_default().insert(Type::I32);
                //     build_instructions! { address, raw, self.abi,
                //         Load { rslt: _0, ty: _i, ptr: rs2 },
                //         Trunc { rslt: _1, ty: _i, val: _0, ty2: _i32 },
                //         Storestack { ty: _i32, val: _1, stk: stk },
                //     }
                // }
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
            RI::Addi {
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
            RI::Slti {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Icmp { rslt: _1, cond: slt, ty: _i, op1: _0, op2: imm },
                Zext { rslt: _2, ty: _i1, val: _1, ty2: _i },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Sltiu {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Icmp { rslt: _1, cond: ult, ty: _i, op1: _0, op2: imm },
                Zext { rslt: _2, ty: _i1, val: _1, ty2: _i },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Xori {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Xor { rslt: _1, ty: _i, op1: _0, op2: imm },
                Store { ty: _i, val: _1, ptr: rd },
            },
            RI::Ori {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Or { rslt: _1, ty: _i, op1: _0, op2: imm },
                Store { ty: _i, val: _1, ptr: rd },
            },
            RI::Andi {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                And { rslt: _1, ty: _i, op1: _0, op2: imm },
                Store { ty: _i, val: _1, ptr: rd },
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
            RI::Srli {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Lshr { rslt: _1, ty: _i, op1: _0, op2: imm },
                Store { ty: _i, val: _1, ptr: rd },
            },
            RI::Srai {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Ashr { rslt: _1, ty: _i, op1: _0, op2: imm },
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
            RI::Sub {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Sub { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Sll {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Shl { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Slt {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: slt, ty: _i, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i },
                Store { ty: _i, val: _3, ptr: rd },
            },
            RI::Sltu {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Icmp { rslt: _2, cond: ult, ty: _i, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i },
                Store { ty: _i, val: _3, ptr: rd },
            },
            RI::Xor {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Xor { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Srl {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Lshr { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Sra {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Ashr { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Or {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Or { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::And {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                And { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Fence { address, raw, .. } => build_instructions! { address, raw, self.abi,
                Fence { ord: monotonic },
            },
            RI::Ecall { address, raw, .. } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: a7 },
                Load { rslt: _1, ty: _i, ptr: a0 },
                Load { rslt: _2, ty: _i, ptr: a1 },
                Load { rslt: _3, ty: _i, ptr: a2 },
                Load { rslt: _4, ty: _i, ptr: a3 },
                Load { rslt: _5, ty: _i, ptr: a4 },
                Load { rslt: _6, ty: _i, ptr: a5 },
                Syscall { rslt: _7, ty: _i, nr: _0, arg1: _1, arg2: _2, arg3: _3, arg4: _4, arg5: _5, arg6: _6 },
                Store { ty: _i, val: _7, ptr: a0 },
            },
            RI::Ebreak { .. } => panic!("`ebreak` is not implemented"),

            // RV64I
            RI::Lwu {
                address,
                raw,
                rd,
                imm,
                rs1,
            } => match rs1 {
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     build_instructions! { address, raw, self.abi,
                //         Loadstack { rslt: _0, ty: _i32, stk: stk },
                //         Zext { rslt: _1, ty: _i32, val: _0, ty2: _i },
                //         Store { ty: _i, val: _1, ptr: rd },
                //     }
                // }
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
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     build_instructions! { address, raw, self.abi,
                //         Loadstack { rslt: _0, ty: _i64, stk: stk },
                //         Zext { rslt: _1, ty: _i64, val: _0, ty2: _i },
                //         Store { ty: _i, val: _1, ptr: rd },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Getdataptr { rslt: _2, ty: _i, addr: _1 },
                    Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i64 },
                    Load { rslt: _4, ty: _i64, ptr: _3 },
                    Zext { rslt: _5, ty: _i64, val: _4, ty2: _i },
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
                // Register::Sp | Register::S0 => {
                //     let Address(addr) = match rs1 {
                //         Register::Sp => self.sp,
                //         Register::S0 => self.fp,
                //         _ => unreachable!(),
                //     };
                //     let Immediate(imm) = *imm;
                //     let stk = if imm >= 0 {
                //         Address(addr + imm as u64)
                //     } else {
                //         Address(addr - (-imm) as u64)
                //     };
                //     self.stack.entry(stk).or_default().insert(Type::I64);
                //     build_instructions! { address, raw, self.abi,
                //         Load { rslt: _0, ty: _i, ptr: rs2 },
                //         Trunc { rslt: _1, ty: _i, val: _0, ty2: _i64 },
                //         Storestack { ty: _i64, val: _1, stk: stk },
                //     }
                // }
                _ => build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs2 },
                    Trunc { rslt: _1, ty: _i, val: _0, ty2: _i64 },
                    Load { rslt: _2, ty: _i, ptr: rs1 },
                    Add { rslt: _3, ty: _i, op1: _2, op2: imm },
                    Getdataptr { rslt: _4, ty: _i, addr: _3 },
                    Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i64 },
                    Store { ty: _i64, val: _1, ptr: _5 },
                },
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
                Trunc { rslt: _2, ty: _i, val: _1, ty2: _i32 },
                Sext { rslt: _3, ty: _i32, val: _2, ty2: _i },
                Store { ty: _i, val: _3, ptr: rd },
            },
            RI::Slliw {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Shl { rslt: _1, ty: _i, op1: _0, op2: imm },
                Trunc { rslt: _2, ty: _i, val: _1, ty2: _i32 },
                Sext { rslt: _3, ty: _i32, val: _2, ty2: _i },
                Store { ty: _i, val: _3, ptr: rd },
            },
            RI::Srliw {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Lshr { rslt: _1, ty: _i, op1: _0, op2: imm },
                Trunc { rslt: _2, ty: _i, val: _1, ty2: _i32 },
                Sext { rslt: _3, ty: _i32, val: _2, ty2: _i },
                Store { ty: _i, val: _3, ptr: rd },
            },
            RI::Sraiw {
                address,
                raw,
                rd,
                rs1,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Ashr { rslt: _1, ty: _i, op1: _0, op2: imm },
                Trunc { rslt: _2, ty: _i, val: _1, ty2: _i32 },
                Sext { rslt: _3, ty: _i32, val: _2, ty2: _i },
                Store { ty: _i, val: _3, ptr: rd },
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
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },
            RI::Subw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Sub { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },
            RI::Sllw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Shl { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },
            RI::Srlw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Lshr { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },
            RI::Sraw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Ashr { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },

            // Pseudoinstructions
            RI::Nop { .. } => Vec::new(),
            RI::Li {
                address,
                raw,
                rd,
                imm,
            } => build_instructions! { address, raw, self.abi,
                Store { ty: _i, val: imm, ptr: rd },
            },
            RI::Mv {
                address,
                raw,
                rd,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Store { ty: _i, val: _0, ptr: rd },
            },
            RI::Not {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(-1);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Xor { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Store { ty: _i, val: _1, ptr: rd },
                }
            }
            RI::Neg {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Sub { rslt: _1, ty: _i, op1: imm, op2: _0 },
                    Store { ty: _i, val: _1, ptr: rd },
                }
            }
            RI::Negw {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Sub { rslt: _1, ty: _i, op1: imm, op2: _0 },
                    Trunc { rslt: _2, ty: _i, val: _1, ty2: _i32 },
                    Sext { rslt: _3, ty: _i32, val: _2, ty2: _i },
                    Store { ty: _i, val: _3, ptr: rd },
                }
            }
            RI::SextW {
                address,
                raw,
                rd,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Store { ty: _i, val: _0, ptr: rd },
            },
            RI::Seqz {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: eq, ty: _i, op1: _0, op2: imm },
                    Zext { rslt: _2, ty: _i1, val: _1, ty2: _i },
                    Store { ty: _i, val: _2, ptr: rd },
                }
            }
            RI::Snez {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: ne, ty: _i, op1: _0, op2: imm },
                    Zext { rslt: _2, ty: _i1, val: _1, ty2: _i },
                    Store { ty: _i, val: _2, ptr: rd },
                }
            }
            RI::Sltz {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: slt, ty: _i, op1: _0, op2: imm },
                    Zext { rslt: _2, ty: _i1, val: _1, ty2: _i },
                    Store { ty: _i, val: _2, ptr: rd },
                }
            }
            RI::Sgtz {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: sgt, ty: _i, op1: _0, op2: imm },
                    Zext { rslt: _2, ty: _i1, val: _1, ty2: _i },
                    Store { ty: _i, val: _2, ptr: rd },
                }
            }

            RI::Beqz {
                address,
                raw,
                rs1,
                addr,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: eq, ty: _i, op1: _0, op2: imm },
                    ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
                }
            }
            RI::Bnez {
                address,
                raw,
                rs1,
                addr,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: ne, ty: _i, op1: _0, op2: imm },
                    ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
                }
            }
            RI::Blez {
                address,
                raw,
                rs1,
                addr,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: sle, ty: _i, op1: _0, op2: imm },
                    ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
                }
            }
            RI::Bgez {
                address,
                raw,
                rs1,
                addr,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: sge, ty: _i, op1: _0, op2: imm },
                    ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
                }
            }
            RI::Bltz {
                address,
                raw,
                rs1,
                addr,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: slt, ty: _i, op1: _0, op2: imm },
                    ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
                }
            }
            RI::Bgtz {
                address,
                raw,
                rs1,
                addr,
            } => {
                let imm = &Immediate(0);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Icmp { rslt: _1, cond: sgt, ty: _i, op1: _0, op2: imm },
                    ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
                }
            }

            RI::J { address, raw, addr } => build_instructions! { address, raw, self.abi,
                UnconBr { addr: addr },
            },
            RI::Jr { address, raw, rs1 } => {
                let default = Value::Address(Address(0x1));
                let targets = self
                    .targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Switch { ty: _i, val: _0, dflt: default, tgts: targets },
                }
            }
            RI::PseudoJalr { address, raw, rs1 } => {
                let default = Value::Address(Address(0x1));
                let targets = self
                    .targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                let imm = &Immediate(0);
                let rs2 = &Register::Ra;
                build_instructions! { address, raw, self.abi,
                    Store { ty: _i, val: next_pc, ptr: rs2 },
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Switch { ty: _i, val: _1, dflt: default, tgts: targets },
                }
            }
            RI::Ret { address, raw } => {
                let default = Value::Address(Address(0x1));
                let targets = self
                    .targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                let rs1 = &Register::Ra;
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Switch { ty: _i, val: _0, dflt: default, tgts: targets },
                }
            }

            RI::PseudoFence { address, raw } => build_instructions! { address, raw, self.abi,
                Fence { ord: monotonic },
            },

            // Misc
            RI::Unimp { address, .. } => Vec::new(), // panic!("Encounter `unimp` at `{}`", address),
            RI::Unknown { address, .. } => Vec::new(), // panic!("Encounter `unknown` at `{}`", address),

            // Ad Hoc
            RI::Mul {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Mul { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Mulw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Mul { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },
            RI::Divw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Sdiv { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },

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
    #[ignore]
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
