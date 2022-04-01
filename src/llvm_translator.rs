use crate::llvm_isa::{
    CodeBlock, Condition, FPCondition, Instruction, InstructionBlock, Ordering, Program, Type,
    Value,
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
    intra_targets: HashMap<Address, Vec<Address>>,
    inter_targets: Vec<Address>,
    ret_targets: HashMap<Address, Vec<Address>>,
    rel_ret_targets: Vec<Address>,
    inherit_ret_targets: HashMap<Address, Vec<Address>>,
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
            intra_targets: HashMap::new(),
            inter_targets: Vec::new(),
            ret_targets: HashMap::new(),
            rel_ret_targets: Vec::new(),
            inherit_ret_targets: HashMap::new(),
            sp: Address(10_000000),
            fp: Address(10_000000),
            stack: HashMap::new(),
        }
    }

    pub fn run(&mut self, rv_program: RiscvProgram) -> Program {
        self.abi = rv_program.abi;
        self.entry = Address(0x0);
        self.data_blocks = rv_program.data_blocks;
        self.intra_targets.clear();
        self.inter_targets.clear();
        self.ret_targets.clear();
        self.rel_ret_targets.clear();
        self.inherit_ret_targets.clear();
        self.sp = Address(10_000000);
        self.fp = Address(10_000000);

        let stack = DataBlock {
            section: String::from(".stack"),
            symbol: String::from("stack"),
            address: Address(1),
            bytes: vec![0; 10240],
        };
        self.data_blocks.push(stack);

        let mut func = Address(0);
        let mut symbol = "";
        let mut last_j_addr = Address(0);
        for code_block in rv_program.code_blocks.iter() {
            // inherit ret targets
            if last_j_addr != Address(0) {
                self.inherit_ret_targets.entry(last_j_addr).or_default().push(func);
            }

            if !code_block.symbol.is_empty() {
                func = code_block.address;
                symbol = &code_block.symbol;
            }

            // intra (jr)
            self.intra_targets.entry(func).or_default().push(code_block.address);

            // inter (jalr, memset, _start)
            if !code_block.symbol.is_empty() || symbol == "memset" {
                self.inter_targets.push(code_block.address);
            }

            // ret (jal)
            if let RiscvInstruction::Jal { address, raw, addr, .. } = code_block.instructions.last().unwrap()
            {
                let Raw(raw) = raw;
                let Address(ad) = address;
                let ret = match raw.len() {
                    5 => Address(ad + 2),
                    9 => Address(ad + 4),
                    _ => unreachable!(),
                };
                self.ret_targets.entry(*addr).or_default().push(ret);
            }

            // rel_ret (jalr)
            if let RiscvInstruction::Jalr { address, raw, .. }
            | RiscvInstruction::ImplicitJalr { address, raw, .. }
            | RiscvInstruction::PseudoJalr { address, raw, .. } = code_block.instructions.last().unwrap()
            {
                let Raw(raw) = raw;
                let Address(addr) = address;
                let addr = match raw.len() {
                    5 => Address(addr + 2),
                    9 => Address(addr + 4),
                    _ => unreachable!(),
                };
                self.rel_ret_targets.push(addr);
            }

            // inherit ret targets
            if let RiscvInstruction::J { addr, .. }= code_block.instructions.last().unwrap() {
                last_j_addr = *addr;
            } else {
                last_j_addr = Address(0);
            }
        }

        // let code_blocks = rv_program
        //     .code_blocks
        //     .into_iter()
        //     .map(|b| self.translate_code_block(b))
        //     .collect();
        let mut func = Address(0);
        let mut code_blocks = Vec::new();
        for code_block in rv_program.code_blocks {
            if !code_block.symbol.is_empty() {
                func = code_block.address;
            }
            let code_block = self.translate_code_block(code_block, func);
            code_blocks.push(code_block);
        }

        Program {
            abi: self.abi,
            entry: self.entry,
            data_blocks: mem::take(&mut self.data_blocks),
            code_blocks,
            stack: mem::take(&mut self.stack),
        }
    }

    fn translate_code_block(&mut self, rv_code_block: RiscvCodeBlock, func: Address) -> CodeBlock {
        if let "_start" = rv_code_block.symbol.as_str() {
            self.entry = rv_code_block.address;
        }

        let instruction_blocks = rv_code_block
            .instructions
            .into_iter()
            .map(|i| self.translate_instruction(i, func))
            .collect();
        CodeBlock {
            section: rv_code_block.section,
            symbol: rv_code_block.symbol,
            address: rv_code_block.address,
            instruction_blocks,
        }
    }

    fn inherit(&self, func: &Address, targets: &mut Vec<Address>) {
        for f in self.inherit_ret_targets.get(func).cloned().unwrap_or_default() {
            self.inherit(&f, targets);
            let tgts = self.ret_targets.get(&f).cloned().unwrap_or_default();
            targets.extend(tgts);
        }
    }

    #[allow(unused_variables)]
    fn translate_instruction(
        &mut self,
        rv_inst: RiscvInstruction,
        func: Address,
    ) -> InstructionBlock {
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
                Shl { rslt: _0, ty: _i32, op1: imm, op2: imm_12 },
                Sext { rslt: _1, ty: _i32, val: _0, ty2: _i },
                Store { ty: _i, val: _1, ptr: rd },
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
                let targets: Vec<_>= self
                    .inter_targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                    if targets.len() >= 300 {
                        println!("jalr {:?}", address);
                    }
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
                let targets: Vec<_> = self
                    .inter_targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                    if targets.len() >= 300 {
                        println!("ImplicitJalr {:?}", address);
                    }
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
                    Sext { rslt: _5, ty: _i64, val: _4, ty2: _i },
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
                let targets: Vec<_> = self
                    .intra_targets.get(&func).cloned().unwrap_or_default()
                    .into_iter()
                    .map(Value::Address)
                    // .filter_map(|addr| if &addr > address {Some(Value::Address(addr))} else {None})
                    .collect();
                    if targets.len() >= 300 {
                        println!("jr {:?} - {}", address, targets.len());
                    }
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Switch { ty: _i, val: _0, dflt: default, tgts: targets },
                }
            }
            RI::OffsetJr {
                address,
                raw,
                imm,
                rs1,
            } => {
                let default = Value::Address(Address(0x1));
                let targets: Vec<_> = self
                    .intra_targets.get(&func).cloned().unwrap_or_default()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                    if targets.len() >= 300 {
                        println!("offsetjr {:?}", address);
                    }
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Switch { ty: _i, val: _1, dflt: default, tgts: targets },
                }
            }
            RI::PseudoJalr { address, raw, rs1 } => {
                let default = Value::Address(Address(0x1));
                let targets: Vec<_> = self
                    .inter_targets
                    .clone()
                    .into_iter()
                    .map(Value::Address)
                    .collect();
                
                    if targets.len() >= 300 {
                        println!("PseudoJalr {:?}", address);
                    }
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
                let mut targets: Vec<_> = self
                    .ret_targets.get(&func).cloned().unwrap_or_default();
                let rel_targets: Vec<_> = self
                    .rel_ret_targets
                    .clone();
                targets.extend(rel_targets);

                self.inherit(&func, &mut targets);
                
                targets.sort_unstable();
                targets.dedup();
                let targets:Vec<_> =     targets.into_iter()
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
            RI::Divu {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Udiv { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Divuw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Udiv { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Remu {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Urem { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Store { ty: _i, val: _2, ptr: rd },
            },
            RI::Remw {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Load { rslt: _1, ty: _i, ptr: rs2 },
                Srem { rslt: _2, ty: _i, op1: _0, op2: _1 },
                Trunc { rslt: _3, ty: _i, val: _2, ty2: _i32 },
                Sext { rslt: _4, ty: _i32, val: _3, ty2: _i },
                Store { ty: _i, val: _4, ptr: rd },
            },
            RI::Mulhu {
                address,
                raw,
                rd,
                rs1,
                rs2,
            } => {
                let imm = &Immediate(64);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    Zext { rslt: _1, ty: _i64, val: _0, ty2: _i128 },
                    Load { rslt: _2, ty: _i, ptr: rs2 },
                    Zext { rslt: _3, ty: _i64, val: _2, ty2: _i128 },
                    Mul { rslt: _4, ty: _i128, op1: _1, op2: _3 },
                    Lshr { rslt: _5, ty: _i128, op1: _4, op2: imm },
                    Trunc { rslt: _6, ty: _i128, val: _5, ty2: _i64 },
                    Store { ty: _i64, val: _6, ptr: rd },
                }
            }

            RI::Fld {
                address,
                raw,
                frd,
                imm,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i64 },
                Load { rslt: _4, ty: _i64, ptr: _3 },
                Bitcast { rslt: _5, ty: _i64, val: _4, ty2: _d },
                Store { ty: _d, val: _5, ptr: frd },
            },
            RI::Fsd {
                address,
                raw,
                frs2,
                imm,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs2 },
                Bitcast { rslt: _1, ty: _d, val: _0, ty2: _i64 },
                Load { rslt: _2, ty: _i, ptr: rs1 },
                Add { rslt: _3, ty: _i, op1: _2, op2: imm },
                Getdataptr { rslt: _4, ty: _i, addr: _3 },
                Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _1, ptr: _5 },
            },
            RI::Flw {
                address,
                raw,
                frd,
                imm,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i, ptr: rs1 },
                Add { rslt: _1, ty: _i, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i32 },
                Load { rslt: _4, ty: _i32, ptr: _3 },
                Bitcast { rslt: _5, ty: _i32, val: _4, ty2: _f },
                Fpext { rslt: _6, ty: _f, val: _5, ty2: _d },
                Store { ty: _d, val: _6, ptr: frd },
            },
            RI::Fsw {
                address,
                raw,
                frs2,
                imm,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs2 },
                Fptrunc { rslt: _1, ty: _d, val: _0, ty2: _f },
                Bitcast { rslt: _2, ty: _f, val: _1, ty2: _i32 },
                Load { rslt: _3, ty: _i, ptr: rs1 },
                Add { rslt: _4, ty: _i, op1: _3, op2: imm },
                Getdataptr { rslt: _5, ty: _i, addr: _4 },
                Bitcast { rslt: _6, ty: _i8, val: _5, ty2: _i32 },
                Store { ty: _i32, val: _2, ptr: _6 },
            },
            RI::FmvXD {
                address,
                raw,
                rd,
                frs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Bitcast { rslt: _1, ty: _d, val: _0, ty2: _i64 },
                Store { ty: _i64, val: _1, ptr: rd },
            },
            RI::FmvDX {
                address,
                raw,
                frd,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Bitcast { rslt: _1, ty: _i64, val: _0, ty2: _d },
                Store { ty: _d, val: _1, ptr: frd },
            },
            RI::FmvXW {
                address,
                raw,
                rd,
                frs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Bitcast { rslt: _1, ty: _d, val: _0, ty2: _i64 },
                Store { ty: _i64, val: _1, ptr: rd },
            },
            RI::FmvWX {
                address,
                raw,
                frd,
                rs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Bitcast { rslt: _1, ty: _i64, val: _0, ty2: _d },
                Store { ty: _d, val: _1, ptr: frd },
            },
            RI::FmulD {
                address,
                raw,
                frd,
                frs1,
                frs2,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fmul { rslt: _2, ty: _d, op1: _0, op2: _1 },
                Store { ty: _d, val: _2, ptr: frd },
            },
            RI::FsubD {
                address,
                raw,
                frd,
                frs1,
                frs2,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fsub { rslt: _2, ty: _d, op1: _0, op2: _1 },
                Store { ty: _d, val: _2, ptr: frd },
            },
            RI::FaddD {
                address,
                raw,
                frd,
                frs1,
                frs2,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fadd { rslt: _2, ty: _d, op1: _0, op2: _1 },
                Store { ty: _d, val: _2, ptr: frd },
            },
            RI::FnegD {
                address,
                raw,
                frd,
                frs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Fneg { rslt: _1, ty: _d, op1: _0 },
                Store { ty: _d, val: _1, ptr: frd },
            },
            RI::FmaddD {
                address,
                raw,
                frd,
                frs1,
                frs2,
                frs3,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Load { rslt: _2, ty: _d, ptr: frs3 },
                Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
                Fadd { rslt: _4, ty: _d, op1: _3, op2: _2 },
                Store { ty: _d, val: _4, ptr: frd },
            },
            RI::FmsubD {
                address,
                raw,
                frd,
                frs1,
                frs2,
                frs3,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Load { rslt: _2, ty: _d, ptr: frs3 },
                Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
                Fsub { rslt: _4, ty: _d, op1: _3, op2: _2 },
                Store { ty: _d, val: _4, ptr: frd },
            },
            RI::FnmsubD {
                address,
                raw,
                frd,
                frs1,
                frs2,
                frs3,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Load { rslt: _2, ty: _d, ptr: frs3 },
                Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
                Fneg { rslt: _4, ty: _d, op1: _3 },
                Fadd { rslt: _5, ty: _d, op1: _4, op2: _2 },
                Store { ty: _d, val: _5, ptr: frd },
            },
            RI::FmulS {
                address,
                raw,
                frd,
                frs1,
                frs2,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fmul { rslt: _2, ty: _d, op1: _0, op2: _1 },
                Store { ty: _d, val: _2, ptr: frd },
            },
            RI::FdivD {
                address,
                raw,
                frd,
                frs1,
                frs2,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fdiv { rslt: _2, ty: _d, op1: _0, op2: _1 },
                Store { ty: _d, val: _2, ptr: frd },
            },
            RI::FdivS {
                address,
                raw,
                frd,
                frs1,
                frs2,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fdiv { rslt: _2, ty: _d, op1: _0, op2: _1 },
                Store { ty: _d, val: _2, ptr: frd },
            },

            RI::FcvtSL {
                address,
                raw,
                frd,
                rs1,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Sitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
                Store { ty: _d, val: _1, ptr: frd },
            },
            RI::FcvtDWu {
                address,
                raw,
                frd,
                rs1,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Uitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
                Store { ty: _d, val: _1, ptr: frd },
            },
            RI::FcvtDW {
                address,
                raw,
                frd,
                rs1,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Sitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
                Store { ty: _d, val: _1, ptr: frd },
            },
            RI::FcvtWD {
                address,
                raw,
                rd,
                frs1,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Fptosi { rslt: _1, ty: _d, val: _0, ty2: _i64 },
                Store { ty: _i64, val: _1, ptr: rd },
            },
            RI::FcvtSW {
                address,
                raw,
                frd,
                rs1,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Sitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
                Store { ty: _d, val: _1, ptr: frd },
            },
            RI::FcvtDS {
                address,
                raw,
                frd,
                frs1,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Store { ty: _d, val: _0, ptr: frd },
            },
            RI::FcvtSD {
                address,
                raw,
                frd,
                frs1,
                rm,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Store { ty: _d, val: _0, ptr: frd },
            },

            RI::FmvD {
                address,
                raw,
                frd,
                frs1,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Store { ty: _d, val: _0, ptr: frd },
            },
            RI::FeqD {
                address,
                raw,
                rd,
                frs1,
                frs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fcmp { rslt: _2, fcond: oeq, ty: _d, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
                Store { ty: _i64, val: _3, ptr: rd },
            },
            RI::FltD {
                address,
                raw,
                rd,
                frs1,
                frs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fcmp { rslt: _2, fcond: olt, ty: _d, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
                Store { ty: _i64, val: _3, ptr: rd },
            },
            RI::FleD {
                address,
                raw,
                rd,
                frs1,
                frs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fcmp { rslt: _2, fcond: ole, ty: _d, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
                Store { ty: _i64, val: _3, ptr: rd },
            },
            RI::FeqS {
                address,
                raw,
                rd,
                frs1,
                frs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fcmp { rslt: _2, fcond: oeq, ty: _d, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
                Store { ty: _i64, val: _3, ptr: rd },
            },
            RI::FltS {
                address,
                raw,
                rd,
                frs1,
                frs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fcmp { rslt: _2, fcond: olt, ty: _d, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
                Store { ty: _i64, val: _3, ptr: rd },
            },
            RI::FleS {
                address,
                raw,
                rd,
                frs1,
                frs2,
            } => build_instructions! { address, raw, self.abi,
                Load { rslt: _0, ty: _d, ptr: frs1 },
                Load { rslt: _1, ty: _d, ptr: frs2 },
                Fcmp { rslt: _2, fcond: ole, ty: _d, op1: _0, op2: _1 },
                Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
                Store { ty: _i64, val: _3, ptr: rd },
            },

            RI::Frrm { .. } => vec![],
            RI::Csrs { .. } => vec![],
            RI::Csrrw { .. } => vec![],
            RI::Csrr { .. } => vec![],
            RI::Csrsi { .. } => vec![],
            RI::Csrrsi { .. } => vec![],
            RI::Csrrci { .. } => vec![],
            RI::Csrrs { .. } => vec![],

            RI::ZextB {
                address,
                raw,
                rd,
                rs1,
            } => {
                let imm = &Immediate(255);
                build_instructions! { address, raw, self.abi,
                    Load { rslt: _0, ty: _i, ptr: rs1 },
                    And { rslt: _1, ty: _i, op1: _0, op2: imm },
                    Store { ty: _i, val: _1, ptr: rd },
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
