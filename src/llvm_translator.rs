use crate::llvm_isa::{
    CodeBlock, Condition, FPCondition, Instruction, InstructionBlock, Ordering, Program, Type,
    Value,
};
use crate::llvm_macro::*;
use crate::riscv_isa::{
    Addr, CodeBlock as RiscvCodeBlock, DataBlock, Imm, Inst as RiscvInstruction,
    Program as RiscvProgram, Reg,
};
use std::collections::{HashMap, HashSet};
use std::mem;

pub fn run(rv_prog: RiscvProgram, src_funcs: &HashMap<Addr, String>) -> Program {
    let mut end = 0;
    for data_block in &rv_prog.data_blocks {
        let Addr(head) = data_block.address;
        let len = data_block.bytes.len();
        if head as usize + len > end {
            end = head as usize + len;
        }
    }
    let mut bytes = vec![0; end];
    for data_block in &rv_prog.data_blocks {
        let Addr(head) = data_block.address;
        for (i, byte) in data_block.bytes.iter().enumerate() {
            bytes[head as usize + i] = *byte;
        }
    }
    let data_blocks = vec![DataBlock {
        address: Addr(0),
        section: String::new(),
        symbol: String::new(),
        bytes,
    }];

    let mut targets: Vec<_> = Vec::new();
    for block in &rv_prog.code_blocks {
        for inst in &block.instructions {
            targets.push(inst.address());
        }
    }
    targets.sort();
    targets.dedup();

    // let code_blocks = rv_program
    //     .code_blocks
    //     .into_iter()
    //     .map(|b| self.translate_code_block(b))
    //     .collect();
    let mut func_targets = HashSet::new();
    for func in &rv_prog.code_blocks {
        let addr = func.instructions[0].address();
        func_targets.insert(addr);
    }

    let mut addr = Addr(0);
    let mut funcs = Vec::new();
    for func in rv_prog.code_blocks {
        if !func.symbol.is_empty() {
            addr = func.address;
        }
        let func = translate_code_block(func, addr, &func_targets);
        funcs.push(func);
    }

    Program {
        entry: self.entry,
        data_blocks: mem::take(&mut self.data_blocks),
        functions: funcs,
        targets,
        parsed_funcs,
        parsed_irs: Vec::new(),
    }
}

fn translate_code_block(
    &mut self,
    rv_code_block: RiscvCodeBlock,
    func: Address,
    func_targets: &HashSet<Address>,
) -> Vec<CodeBlock> {
    if let "_start" = rv_code_block.symbol.as_str() {
        self.entry = rv_code_block.address;
    }

    rv_code_block
        .instructions
        .into_iter()
        .map(|i| {
            let address = i.address();
            let b = self.translate_instruction(i, func, func_targets);
            CodeBlock {
                section: rv_code_block.section.clone(),
                symbol: rv_code_block.symbol.clone(),
                address,
                instruction_blocks: vec![b],
            }
        })
        .collect()
}

#[allow(unused_variables)]
fn translate_instruction(
    &mut self,
    rv_inst: RiscvInstruction,
    func: Address,
    func_targets: &HashSet<Address>,
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
        } => build_instructions! { address, raw,
            Shl { rslt: _0, ty: _i32, op1: imm, op2: imm_12 },
            Sext { rslt: _1, ty: _i32, val: _0, ty2: _i64 },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Auipc {
            address,
            raw,
            rd,
            imm,
        } => build_instructions! { address, raw,
            Shl { rslt: _0, ty: _i64, op1: imm, op2: imm_12 },
            Add { rslt: _1, ty: _i64, op1: _0, op2: address },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Jal {
            address,
            raw,
            rd,
            addr,
        } => build_instructions! { address, raw,
            Store { ty: _i64, val: next_pc, ptr: rd },
            Ret { ty: _i64, val: addr },
            // Call { addr: addr },
            // UnconBr { addr: next_pc },
        },
        RI::Jalr {
            address,
            raw,
            rd,
            imm,
            rs1,
        } => {
            // let default = Value::Address(Address(0x1));
            // let targets: Vec<_>= self
            //     .inter_targets
            //     .clone()
            //     .into_iter()
            //     .map(Value::Address)
            //     .collect();
            //     // if targets.len() >= 300 {
            //     //     println!("jalr {:?}", address);
            //     // }
            build_instructions! { address, raw,
                Store { ty: _i64, val: next_pc, ptr: rd },
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                // Switch { ty: _i64, val: _1, dflt: default, tgts: targets },
            Ret { ty: _i64, val: _1 },
            }
        }
        RI::ImplicitJalr {
            address,
            raw,
            imm,
            rs1,
        } => {
            // let default = Value::Address(Address(0x1));
            // let targets: Vec<_> = self
            //     .inter_targets
            //     .clone()
            //     .into_iter()
            //     .map(Value::Address)
            //     .collect();
            // if targets.len() >= 300 {
            //     println!("ImplicitJalr {:?}", address);
            // }
            let rs2 = &Register::Ra;
            build_instructions! { address, raw,
                Store { ty: _i64, val: next_pc, ptr: rs2 },
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                // Switch { ty: _i64, val: _1, dflt: default, tgts: targets },
            Ret { ty: _i64, val: _1 },
            }
        }
        RI::Beq {
            address,
            raw,
            rs1,
            rs2,
            addr,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: eq, ty: _i64, op1: _0, op2: _1 },
            ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
        },
        RI::Bne {
            address,
            raw,
            rs1,
            rs2,
            addr,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: ne, ty: _i64, op1: _0, op2: _1 },
            ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
        },
        RI::Blt {
            address,
            raw,
            rs1,
            rs2,
            addr,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: slt, ty: _i64, op1: _0, op2: _1 },
            ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
        },
        RI::Bge {
            address,
            raw,
            rs1,
            rs2,
            addr,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: sge, ty: _i64, op1: _0, op2: _1 },
            ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
        },
        RI::Bltu {
            address,
            raw,
            rs1,
            rs2,
            addr,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: ult, ty: _i64, op1: _0, op2: _1 },
            ConBr { cond: _2, iftrue: addr, iffalse: next_pc },
        },
        RI::Bgeu {
            address,
            raw,
            rs1,
            rs2,
            addr,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: uge, ty: _i64, op1: _0, op2: _1 },
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
            //     build_instructions! { address, raw,
            //         Loadstack { rslt: _0, ty: _i8, stk: stk },
            //         Sext { rslt: _1, ty: _i8, val: _0, ty2: _i64 },
            //         Store { ty: _i64, val: _1, ptr: rd },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i64, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i8 },
                Load { rslt: _4, ty: _i8, ptr: _3 },
                Sext { rslt: _5, ty: _i8, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _5, ptr: rd },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Loadstack { rslt: _0, ty: _i16, stk: stk },
            //         Sext { rslt: _1, ty: _i16, val: _0, ty2: _i64 },
            //         Store { ty: _i64, val: _1, ptr: rd },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i64, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i16 },
                Load { rslt: _4, ty: _i16, ptr: _3 },
                Sext { rslt: _5, ty: _i16, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _5, ptr: rd },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Loadstack { rslt: _0, ty: _i32, stk: stk },
            //         Sext { rslt: _1, ty: _i32, val: _0, ty2: _i64 },
            //         Store { ty: _i64, val: _1, ptr: rd },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i64, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i32 },
                Load { rslt: _4, ty: _i32, ptr: _3 },
                Sext { rslt: _5, ty: _i32, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _5, ptr: rd },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Loadstack { rslt: _0, ty: _i8, stk: stk },
            //         Zext { rslt: _1, ty: _i8, val: _0, ty2: _i64 },
            //         Store { ty: _i64, val: _1, ptr: rd },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i64, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i8 },
                Load { rslt: _4, ty: _i8, ptr: _3 },
                Zext { rslt: _5, ty: _i8, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _5, ptr: rd },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Loadstack { rslt: _0, ty: _i16, stk: stk },
            //         Zext { rslt: _1, ty: _i16, val: _0, ty2: _i64 },
            //         Store { ty: _i64, val: _1, ptr: rd },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i64, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i16 },
                Load { rslt: _4, ty: _i16, ptr: _3 },
                Zext { rslt: _5, ty: _i16, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _5, ptr: rd },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Load { rslt: _0, ty: _i64, ptr: rs2 },
            //         Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i8 },
            //         Storestack { ty: _i8, val: _1, stk: stk },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs2 },
                Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i8 },
                Load { rslt: _2, ty: _i64, ptr: rs1 },
                Add { rslt: _3, ty: _i64, op1: _2, op2: imm },
                Getdataptr { rslt: _4, ty: _i64, addr: _3 },
                Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i8 },
                Store { ty: _i8, val: _1, ptr: _5 },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Load { rslt: _0, ty: _i64, ptr: rs2 },
            //         Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i16 },
            //         Storestack { ty: _i16, val: _1, stk: stk },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs2 },
                Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i16 },
                Load { rslt: _2, ty: _i64, ptr: rs1 },
                Add { rslt: _3, ty: _i64, op1: _2, op2: imm },
                Getdataptr { rslt: _4, ty: _i64, addr: _3 },
                Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i16 },
                Store { ty: _i16, val: _1, ptr: _5 },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Load { rslt: _0, ty: _i64, ptr: rs2 },
            //         Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i32 },
            //         Storestack { ty: _i32, val: _1, stk: stk },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs2 },
                Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i32 },
                Load { rslt: _2, ty: _i64, ptr: rs1 },
                Add { rslt: _3, ty: _i64, op1: _2, op2: imm },
                Getdataptr { rslt: _4, ty: _i64, addr: _3 },
                Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i32 },
                Store { ty: _i32, val: _1, ptr: _5 },
                UnconBr { addr: next_pc },
            },
        },
        RI::Addi {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Slti {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Icmp { rslt: _1, cond: slt, ty: _i64, op1: _0, op2: imm },
            Zext { rslt: _2, ty: _i1, val: _1, ty2: _i64 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sltiu {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Icmp { rslt: _1, cond: ult, ty: _i64, op1: _0, op2: imm },
            Zext { rslt: _2, ty: _i1, val: _1, ty2: _i64 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Xori {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Xor { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Ori {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Or { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Andi {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            And { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Slli {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Shl { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Srli {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Lshr { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Srai {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Ashr { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Add {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Add { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sub {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Sub { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sll {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Shl { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Slt {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: slt, ty: _i64, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sltu {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Icmp { rslt: _2, cond: ult, ty: _i64, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Xor {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Xor { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Srl {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Lshr { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sra {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Ashr { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Or {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Or { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::And {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            And { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Fence { address, raw, .. } => build_instructions! { address, raw,
            Fence { ord: acq_rel },
            UnconBr { addr: next_pc },
        },
        RI::Ecall { address, raw, .. } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: a7 },
            Load { rslt: _1, ty: _i64, ptr: a0 },
            Load { rslt: _2, ty: _i64, ptr: a1 },
            Load { rslt: _3, ty: _i64, ptr: a2 },
            Load { rslt: _4, ty: _i64, ptr: a3 },
            Load { rslt: _5, ty: _i64, ptr: a4 },
            Load { rslt: _6, ty: _i64, ptr: a5 },
            Syscall { rslt: _7, ty: _i64, nr: _0, arg1: _1, arg2: _2, arg3: _3, arg4: _4, arg5: _5, arg6: _6 },
            Store { ty: _i64, val: _7, ptr: a0 },
            UnconBr { addr: next_pc },
        },
        RI::Ebreak { address, raw, .. } => build_instructions! { address, raw,
            Unreachable { addr: address },
            // UnconBr { addr: next_pc },
        }, //panic!("`ebreak` is not implemented"),

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
            //     build_instructions! { address, raw,
            //         Loadstack { rslt: _0, ty: _i32, stk: stk },
            //         Zext { rslt: _1, ty: _i32, val: _0, ty2: _i64 },
            //         Store { ty: _i64, val: _1, ptr: rd },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i64, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i32 },
                Load { rslt: _4, ty: _i32, ptr: _3 },
                Zext { rslt: _5, ty: _i32, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _5, ptr: rd },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Loadstack { rslt: _0, ty: _i64, stk: stk },
            //         Zext { rslt: _1, ty: _i64, val: _0, ty2: _i64 },
            //         Store { ty: _i64, val: _1, ptr: rd },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Getdataptr { rslt: _2, ty: _i64, addr: _1 },
                Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i64 },
                Load { rslt: _4, ty: _i64, ptr: _3 },
                Sext { rslt: _5, ty: _i64, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _5, ptr: rd },
                UnconBr { addr: next_pc },
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
            //     build_instructions! { address, raw,
            //         Load { rslt: _0, ty: _i64, ptr: rs2 },
            //         Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i64 },
            //         Storestack { ty: _i64, val: _1, stk: stk },
            //     }
            // }
            _ => build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs2 },
                Trunc { rslt: _1, ty: _i64, val: _0, ty2: _i64 },
                Load { rslt: _2, ty: _i64, ptr: rs1 },
                Add { rslt: _3, ty: _i64, op1: _2, op2: imm },
                Getdataptr { rslt: _4, ty: _i64, addr: _3 },
                Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i64 },
                Store { ty: _i64, val: _1, ptr: _5 },
                UnconBr { addr: next_pc },
            },
        },
        RI::Addiw {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Trunc { rslt: _2, ty: _i64, val: _1, ty2: _i32 },
            Sext { rslt: _3, ty: _i32, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Slliw {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Shl { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Trunc { rslt: _2, ty: _i64, val: _1, ty2: _i32 },
            Sext { rslt: _3, ty: _i32, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Srliw {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Lshr { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Trunc { rslt: _2, ty: _i64, val: _1, ty2: _i32 },
            Sext { rslt: _3, ty: _i32, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sraiw {
            address,
            raw,
            rd,
            rs1,
            imm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Ashr { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Trunc { rslt: _2, ty: _i64, val: _1, ty2: _i32 },
            Sext { rslt: _3, ty: _i32, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Addw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Add { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Subw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Sub { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sllw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Shl { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Srlw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Lshr { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Sraw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Ashr { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },

        // Pseudoinstructions
        RI::Nop { address, raw } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Li {
            address,
            raw,
            rd,
            imm,
        } => build_instructions! { address, raw,
            Store { ty: _i64, val: imm, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Mv {
            address,
            raw,
            rd,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Store { ty: _i64, val: _0, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Not {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(-1);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Xor { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Store { ty: _i64, val: _1, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }
        RI::Neg {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(0);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Sub { rslt: _1, ty: _i64, op1: imm, op2: _0 },
                Store { ty: _i64, val: _1, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }
        RI::Negw {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(0);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Sub { rslt: _1, ty: _i64, op1: imm, op2: _0 },
                Trunc { rslt: _2, ty: _i64, val: _1, ty2: _i32 },
                Sext { rslt: _3, ty: _i32, val: _2, ty2: _i64 },
                Store { ty: _i64, val: _3, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }
        RI::SextW {
            address,
            raw,
            rd,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Store { ty: _i64, val: _0, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Seqz {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(0);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: eq, ty: _i64, op1: _0, op2: imm },
                Zext { rslt: _2, ty: _i1, val: _1, ty2: _i64 },
                Store { ty: _i64, val: _2, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }
        RI::Snez {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(0);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: ne, ty: _i64, op1: _0, op2: imm },
                Zext { rslt: _2, ty: _i1, val: _1, ty2: _i64 },
                Store { ty: _i64, val: _2, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }
        RI::Sltz {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(0);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: slt, ty: _i64, op1: _0, op2: imm },
                Zext { rslt: _2, ty: _i1, val: _1, ty2: _i64 },
                Store { ty: _i64, val: _2, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }
        RI::Sgtz {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(0);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: sgt, ty: _i64, op1: _0, op2: imm },
                Zext { rslt: _2, ty: _i1, val: _1, ty2: _i64 },
                Store { ty: _i64, val: _2, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }

        RI::Beqz {
            address,
            raw,
            rs1,
            addr,
        } => {
            let imm = &Immediate(0);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: eq, ty: _i64, op1: _0, op2: imm },
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
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: ne, ty: _i64, op1: _0, op2: imm },
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
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: sle, ty: _i64, op1: _0, op2: imm },
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
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: sge, ty: _i64, op1: _0, op2: imm },
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
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: slt, ty: _i64, op1: _0, op2: imm },
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
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Icmp { rslt: _1, cond: sgt, ty: _i64, op1: _0, op2: imm },
                ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
            }
        }

        // RI::J { address, raw, addr } => build_instructions! { address, raw,
        //     Ret { ty: _i64, val: addr },
        // },
        RI::J { address, raw, addr } => {
            if func_targets.contains(&addr) {
                build_instructions! { address, raw,
                    // Call { addr: addr },
                    Ret{ty: _i64, val: addr},
                }
            } else {
                build_instructions! { address, raw,
                    UnconBr { addr: addr },
                    // Ret{ty: _i64, val: addr},
                }
            }
        }
        RI::Jr { address, raw, rs1 } => {
            // let default = Value::Address(Address(0x1));
            // let targets: Vec<_> = self
            //     .intra_targets.get(&func).cloned().unwrap_or_default()
            //     .into_iter()
            //     .map(Value::Address)
            //     // .filter_map(|addr| if &addr > address {Some(Value::Address(addr))} else {None})
            //     .collect();
            //     // if targets.len() >= 300 {
            //     //     println!("jr {:?} - {}", address, targets.len());
            //     // }
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                // Switch { ty: _i64, val: _0, dflt: default, tgts: targets },
            Ret { ty: _i64, val: _0 },
            }
        }
        RI::OffsetJr {
            address,
            raw,
            imm,
            rs1,
        } => {
            // let default = Value::Address(Address(0x1));
            // let targets: Vec<_> = self
            //     .intra_targets.get(&func).cloned().unwrap_or_default()
            //     .into_iter()
            //     .map(Value::Address)
            //     .collect();
            //     // if targets.len() >= 300 {
            //     //     println!("offsetjr {:?}", address);
            //     // }
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                // Switch { ty: _i64, val: _1, dflt: default, tgts: targets },
            Ret { ty: _i64, val: _1 },
            }
        }
        RI::PseudoJalr { address, raw, rs1 } => {
            // let default = Value::Address(Address(0x1));
            // let targets: Vec<_> = self
            //     .inter_targets
            //     .clone()
            //     .into_iter()
            //     .map(Value::Address)
            //     .collect();

            //     // if targets.len() >= 300 {
            //     //     println!("PseudoJalr {:?}", address);
            //     // }
            let imm = &Immediate(0);
            let rs2 = &Register::Ra;
            build_instructions! { address, raw,
                Store { ty: _i64, val: next_pc, ptr: rs2 },
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
                // Switch { ty: _i64, val: _1, dflt: default, tgts: targets },
            Ret { ty: _i64, val: _1 },
            }
        }
        RI::Ret { address, raw } => {
            // let default = Value::Address(Address(0x1));
            // let mut targets: Vec<_> = self
            //     .ret_targets.get(&func).cloned().unwrap_or_default();
            // let rel_targets: Vec<_> = self
            //     .rel_ret_targets
            //     .clone();
            // targets.extend(rel_targets);

            // self.inherit(&func, &mut targets);

            // targets.sort_unstable();
            // targets.dedup();
            // let targets:Vec<_> =     targets.into_iter()
            //     .map(Value::Address)
            //     .collect();

            let rs1 = &Register::Ra;
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                // Switch { ty: _i64, val: _0, dflt: default, tgts: targets },
            Ret { ty: _i64, val: _0 },
            }
        }

        RI::PseudoFence { address, raw } => build_instructions! { address, raw,
            Fence { ord: acq_rel },
            UnconBr { addr: next_pc },
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
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Mul { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Mulw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Mul { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Divw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Sdiv { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Divu {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Udiv { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Divuw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Udiv { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Div {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Sdiv { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Remu {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Urem { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Remw {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Srem { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty: _i64, val: _2, ty2: _i32 },
            Sext { rslt: _4, ty: _i32, val: _3, ty2: _i64 },
            Store { ty: _i64, val: _4, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Rem {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Load { rslt: _1, ty: _i64, ptr: rs2 },
            Srem { rslt: _2, ty: _i64, op1: _0, op2: _1 },
            Store { ty: _i64, val: _2, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::Mulhu {
            address,
            raw,
            rd,
            rs1,
            rs2,
        } => {
            let imm = &Immediate(64);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Zext { rslt: _1, ty: _i64, val: _0, ty2: _i128 },
                Load { rslt: _2, ty: _i64, ptr: rs2 },
                Zext { rslt: _3, ty: _i64, val: _2, ty2: _i128 },
                Mul { rslt: _4, ty: _i128, op1: _1, op2: _3 },
                Lshr { rslt: _5, ty: _i128, op1: _4, op2: imm },
                Trunc { rslt: _6, ty: _i128, val: _5, ty2: _i64 },
                Store { ty: _i64, val: _6, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }

        RI::Fld {
            address,
            raw,
            frd,
            imm,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Getdataptr { rslt: _2, ty: _i64, addr: _1 },
            Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i64 },
            Load { rslt: _4, ty: _i64, ptr: _3 },
            Bitcast { rslt: _5, ty: _i64, val: _4, ty2: _d },
            Store { ty: _d, val: _5, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::Fsd {
            address,
            raw,
            frs2,
            imm,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs2 },
            Bitcast { rslt: _1, ty: _d, val: _0, ty2: _i64 },
            Load { rslt: _2, ty: _i64, ptr: rs1 },
            Add { rslt: _3, ty: _i64, op1: _2, op2: imm },
            Getdataptr { rslt: _4, ty: _i64, addr: _3 },
            Bitcast { rslt: _5, ty: _i8, val: _4, ty2: _i64 },
            Store { ty: _i64, val: _1, ptr: _5 },
            UnconBr { addr: next_pc },
        },
        RI::Flw {
            address,
            raw,
            frd,
            imm,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Add { rslt: _1, ty: _i64, op1: _0, op2: imm },
            Getdataptr { rslt: _2, ty: _i64, addr: _1 },
            Bitcast { rslt: _3, ty: _i8, val: _2, ty2: _i32 },
            Load { rslt: _4, ty: _i32, ptr: _3 },
            Bitcast { rslt: _5, ty: _i32, val: _4, ty2: _f },
            Fpext { rslt: _6, ty: _f, val: _5, ty2: _d },
            Store { ty: _d, val: _6, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::Fsw {
            address,
            raw,
            frs2,
            imm,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs2 },
            Fptrunc { rslt: _1, ty: _d, val: _0, ty2: _f },
            Bitcast { rslt: _2, ty: _f, val: _1, ty2: _i32 },
            Load { rslt: _3, ty: _i64, ptr: rs1 },
            Add { rslt: _4, ty: _i64, op1: _3, op2: imm },
            Getdataptr { rslt: _5, ty: _i64, addr: _4 },
            Bitcast { rslt: _6, ty: _i8, val: _5, ty2: _i32 },
            Store { ty: _i32, val: _2, ptr: _6 },
            UnconBr { addr: next_pc },
        },
        RI::FmvXD {
            address,
            raw,
            rd,
            frs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Bitcast { rslt: _1, ty: _d, val: _0, ty2: _i64 },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FmvDX {
            address,
            raw,
            frd,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Bitcast { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FmvXW {
            address,
            raw,
            rd,
            frs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Bitcast { rslt: _1, ty: _d, val: _0, ty2: _i64 },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FmvWX {
            address,
            raw,
            frd,
            rs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Bitcast { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FmulD {
            address,
            raw,
            frd,
            frs1,
            frs2,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fmul { rslt: _2, ty: _d, op1: _0, op2: _1 },
            Store { ty: _d, val: _2, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FsubD {
            address,
            raw,
            frd,
            frs1,
            frs2,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fsub { rslt: _2, ty: _d, op1: _0, op2: _1 },
            Store { ty: _d, val: _2, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FaddD {
            address,
            raw,
            frd,
            frs1,
            frs2,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fadd { rslt: _2, ty: _d, op1: _0, op2: _1 },
            Store { ty: _d, val: _2, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FnegD {
            address,
            raw,
            frd,
            frs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Fneg { rslt: _1, ty: _d, op1: _0 },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FmaddD {
            address,
            raw,
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Load { rslt: _2, ty: _d, ptr: frs3 },
            Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
            Fadd { rslt: _4, ty: _d, op1: _3, op2: _2 },
            Store { ty: _d, val: _4, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FmsubD {
            address,
            raw,
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Load { rslt: _2, ty: _d, ptr: frs3 },
            Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
            Fsub { rslt: _4, ty: _d, op1: _3, op2: _2 },
            Store { ty: _d, val: _4, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FnmsubD {
            address,
            raw,
            frd,
            frs1,
            frs2,
            frs3,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Load { rslt: _2, ty: _d, ptr: frs3 },
            Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
            Fneg { rslt: _4, ty: _d, op1: _3 },
            Fadd { rslt: _5, ty: _d, op1: _4, op2: _2 },
            Store { ty: _d, val: _5, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FmulS {
            address,
            raw,
            frd,
            frs1,
            frs2,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fmul { rslt: _2, ty: _d, op1: _0, op2: _1 },
            Store { ty: _d, val: _2, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FdivD {
            address,
            raw,
            frd,
            frs1,
            frs2,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fdiv { rslt: _2, ty: _d, op1: _0, op2: _1 },
            Store { ty: _d, val: _2, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FdivS {
            address,
            raw,
            frd,
            frs1,
            frs2,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fdiv { rslt: _2, ty: _d, op1: _0, op2: _1 },
            Store { ty: _d, val: _2, ptr: frd },
            UnconBr { addr: next_pc },
        },

        RI::FcvtSL {
            address,
            raw,
            frd,
            rs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Sitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtDWu {
            address,
            raw,
            frd,
            rs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Uitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtDLu {
            address,
            raw,
            frd,
            rs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Uitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtDW {
            address,
            raw,
            frd,
            rs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Sitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtDL {
            address,
            raw,
            frd,
            rs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Sitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtWD {
            address,
            raw,
            rd,
            frs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Fptosi { rslt: _1, ty: _d, val: _0, ty2: _i64 },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtLD {
            address,
            raw,
            rd,
            frs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Fptosi { rslt: _1, ty: _d, val: _0, ty2: _i64 },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtWuD {
            address,
            raw,
            rd,
            frs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Fptoui { rslt: _1, ty: _d, val: _0, ty2: _i64 },
            Store { ty: _i64, val: _1, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtSW {
            address,
            raw,
            frd,
            rs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _i64, ptr: rs1 },
            Sitofp { rslt: _1, ty: _i64, val: _0, ty2: _d },
            Store { ty: _d, val: _1, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtDS {
            address,
            raw,
            frd,
            frs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Store { ty: _d, val: _0, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FcvtSD {
            address,
            raw,
            frd,
            frs1,
            rm,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Store { ty: _d, val: _0, ptr: frd },
            UnconBr { addr: next_pc },
        },

        RI::FmvD {
            address,
            raw,
            frd,
            frs1,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Store { ty: _d, val: _0, ptr: frd },
            UnconBr { addr: next_pc },
        },
        RI::FeqD {
            address,
            raw,
            rd,
            frs1,
            frs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: oeq, ty: _d, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FltD {
            address,
            raw,
            rd,
            frs1,
            frs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: olt, ty: _d, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FleD {
            address,
            raw,
            rd,
            frs1,
            frs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: ole, ty: _d, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FeqS {
            address,
            raw,
            rd,
            frs1,
            frs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: oeq, ty: _d, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FltS {
            address,
            raw,
            rd,
            frs1,
            frs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: olt, ty: _d, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },
        RI::FleS {
            address,
            raw,
            rd,
            frs1,
            frs2,
        } => build_instructions! { address, raw,
            Load { rslt: _0, ty: _d, ptr: frs1 },
            Load { rslt: _1, ty: _d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: ole, ty: _d, op1: _0, op2: _1 },
            Zext { rslt: _3, ty: _i1, val: _2, ty2: _i64 },
            Store { ty: _i64, val: _3, ptr: rd },
            UnconBr { addr: next_pc },
        },

        RI::Frrm { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Csrs { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Csrrw { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Csrr { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Csrsi { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Csrrsi { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Csrrci { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },
        RI::Csrrs { address, raw, .. } => build_instructions! { address, raw,
            UnconBr { addr: next_pc },
        },

        RI::ZextB {
            address,
            raw,
            rd,
            rs1,
        } => {
            let imm = &Immediate(255);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                And { rslt: _1, ty: _i64, op1: _0, op2: imm },
                Store { ty: _i64, val: _1, ptr: rd },
                UnconBr { addr: next_pc },
            }
        }

        RI::AmoswapD {
            address,
            raw,
            rd,
            rs2,
            rs1,
            ..
        } => {
            let imm = &Immediate(255);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Store { ty: _i64, val: _0, ptr: rd },
                Store { ty: _i64, val: _0, ptr: rs1 },
                UnconBr { addr: next_pc },
            }
        }
        RI::AmoaddD {
            address,
            raw,
            rd,
            rs2,
            rs1,
            ..
        }
        | RI::AmoaddW {
            address,
            raw,
            rd,
            rs2,
            rs1,
            ..
        } => {
            let imm = &Immediate(255);
            build_instructions! { address, raw,
                Load { rslt: _0, ty: _i64, ptr: rs1 },
                Store { ty: _i64, val: _0, ptr: rd },
                Load { rslt: _1, ty: _i64, ptr: rs2 },
                Add { rslt: _2, ty: _i64, op1: _0, op2: _1 },
                Store { ty: _i64, val: _2, ptr: rs1 },
                UnconBr { addr: next_pc },
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
