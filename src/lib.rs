#[macro_use]
extern crate lazy_static;

mod llvm_isa;
mod riscv_isa;

use llvm_isa::LlvmInstruction;
use llvm_isa::LlvmType;
use llvm_isa::LlvmValue;
use riscv_isa::riscv_instruction_regex;
use riscv_isa::RiscvInstruction;
use riscv_isa::RiscvRegister;
use std::collections::HashMap;

pub fn run(riscv_source: &str) -> String {
    let riscv_insts = riscv_source
        .lines()
        .skip_while(|line| !line.contains("<main>:"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(build_riscv_instruction)
        .collect();
    let llvm_insts = lift_to_llvm(riscv_insts);
    format!(
        "define i64 @main() {{\n{}}}\n",
        llvm_insts
            .iter()
            .map(|inst| format!("    {}\n", inst))
            .reduce(|ll, inst| ll + &inst)
            .unwrap()
    )
}

fn build_riscv_instruction(line: &str) -> RiscvInstruction {
    match line {
        line if riscv_instruction_regex::ADDI.is_match(line) => {
            let caps = riscv_instruction_regex::ADDI.captures(line).unwrap();
            RiscvInstruction::Addi(
                build_riscv_register(&caps["rd"]),
                caps["imm"].parse().unwrap(),
                build_riscv_register(&caps["rs1"]),
            )
        }
        line if riscv_instruction_regex::SD.is_match(line) => {
            let caps = riscv_instruction_regex::SD.captures(line).unwrap();
            RiscvInstruction::Sd(
                build_riscv_register(&caps["rs2"]),
                caps["imm"].parse().unwrap(),
                build_riscv_register(&caps["rs1"]),
            )
        }
        line if riscv_instruction_regex::LI.is_match(line) => {
            let caps = riscv_instruction_regex::LI.captures(line).unwrap();
            RiscvInstruction::Li(
                build_riscv_register(&caps["rd"]),
                caps["imm"].parse().unwrap(),
            )
        }
        line if riscv_instruction_regex::MV.is_match(line) => {
            let caps = riscv_instruction_regex::MV.captures(line).unwrap();
            RiscvInstruction::Mv(
                build_riscv_register(&caps["rd"]),
                build_riscv_register(&caps["rs1"]),
            )
        }
        line if riscv_instruction_regex::LD.is_match(line) => {
            let caps = riscv_instruction_regex::LD.captures(line).unwrap();
            RiscvInstruction::Ld(
                build_riscv_register(&caps["rd"]),
                caps["imm"].parse().unwrap(),
                build_riscv_register(&caps["rs1"]),
            )
        }
        line if riscv_instruction_regex::RET.is_match(line) => RiscvInstruction::Ret,
        line => panic!("Unknown RISC-V instruction `{}`", line),
    }
}

fn build_riscv_register(reg: &str) -> RiscvRegister {
    match reg {
        "sp" => RiscvRegister::Sp,
        "s0" => RiscvRegister::S0,
        "a5" => RiscvRegister::A5,
        "a0" => RiscvRegister::A0,
        reg => panic!("Unknown RISC-V register `{}`", reg),
    }
}

fn lift_to_llvm(riscv_insts: Vec<RiscvInstruction>) -> Vec<LlvmInstruction> {
    let mut llvm_insts = Vec::new();
    let mut symbol_table: HashMap<RiscvRegister, (u64, LlvmType, LlvmValue)> = HashMap::new();
    for riscv_inst in riscv_insts {
        match riscv_inst {
            RiscvInstruction::Addi(_rd, _imm, _rs1) => {}
            RiscvInstruction::Sd(_rs2, _imm, _rs1) => {}
            RiscvInstruction::Li(rd, imm) => {
                let sub = symbol_table
                    .get(&rd)
                    .map(|(sub, _, _)| sub + 1)
                    .unwrap_or_default();
                symbol_table.insert(rd, (sub, LlvmType::I64, LlvmValue::Literal(imm)));
                llvm_insts.push(LlvmInstruction::Add(
                    LlvmValue::Register(rd, sub),
                    LlvmType::I64,
                    LlvmValue::Literal(imm),
                    LlvmValue::Literal(0),
                ))
            }
            RiscvInstruction::Mv(rd, rs1) => {
                let (rs1_sub, rs1_ty) = symbol_table
                    .get(&rs1)
                    .map(|(sub, ty, _)| (*sub, *ty))
                    .unwrap_or_else(|| panic!("Read the uninitialized register `{:?}`", rs1));
                let rd_sub = symbol_table
                    .get(&rd)
                    .map(|(sub, _, _)| sub + 1)
                    .unwrap_or_default();
                symbol_table.insert(rd, (rd_sub, rs1_ty, LlvmValue::Register(rs1, rs1_sub)));
                llvm_insts.push(LlvmInstruction::Add(
                    LlvmValue::Register(rd, rd_sub),
                    rs1_ty,
                    LlvmValue::Register(rs1, rs1_sub),
                    LlvmValue::Literal(0),
                ))
            }
            RiscvInstruction::Ld(_rd, _imm, _rs1) => {}
            RiscvInstruction::Ret => match symbol_table.get(&RiscvRegister::A0) {
                Some((_, ty, LlvmValue::Literal(value))) => {
                    llvm_insts.push(LlvmInstruction::Ret(*ty, LlvmValue::Literal(*value)))
                }
                Some((sub, ty, LlvmValue::Register(_, _))) => llvm_insts.push(
                    LlvmInstruction::Ret(*ty, LlvmValue::Register(RiscvRegister::A0, *sub)),
                ),
                None => panic!("`ret void` is not supported yet."),
            },
        }
    }
    llvm_insts
}
