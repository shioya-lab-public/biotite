use crate::cfg::{BasicBlock, Cfg, RiscvFunction};
use crate::llvm_isa::{LlvmCondition, LlvmFunction, LlvmInstruction, LlvmType, Program};
use crate::riscv_isa::{RiscvAddress, RiscvInstruction, RiscvRegister, FUNCTION};
use std::collections::HashMap;

pub fn translate(cfg: Cfg) -> Program {
    cfg.into_iter().map(translate_function).collect()
}

fn translate_function(function: RiscvFunction) -> LlvmFunction {
    let RiscvFunction {
        name,
        basic_blocks,
        potential_targets,
    } = function;
    let mut body: Vec<_> = basic_blocks
        .into_iter()
        .enumerate()
        .map(|(i, block)| translate_basic_block(i, block, &potential_targets))
        .flatten()
        .collect();
    body.insert(0, LlvmInstruction::DirectBr(String::from("L0")));
    body.insert(0, LlvmInstruction::Label(String::from("Entry")));
    LlvmFunction { name, body }
}

fn translate_basic_block(
    index: usize,
    basic_block: BasicBlock,
    potential_targets: &HashMap<RiscvAddress, usize>,
) -> Vec<LlvmInstruction> {
    let mut llvm_insts = vec![LlvmInstruction::Label(format!("L{}", index))];
    let BasicBlock {
        instructions,
        continue_target,
        jump_target,
    } = basic_block;
    for riscv_inst in instructions {
        let insts = match riscv_inst {
            // RV64I
            RiscvInstruction::Add { rd, rs1, rs2, .. }
            | RiscvInstruction::Addw { rd, rs1, rs2, .. } => vec![LlvmInstruction::Add {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Addi { rd, rs1, imm, .. }
            | RiscvInstruction::Addiw { rd, rs1, imm, .. } => vec![LlvmInstruction::Addi {
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::And { rd, rs1, rs2, .. } => vec![LlvmInstruction::And {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Andi { rd, rs1, imm, .. } => vec![LlvmInstruction::Andi {
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Auipc { .. } => Vec::new(),
            RiscvInstruction::Beq { rs1, rs2, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Eq,
                    op1: rs1,
                    op2: rs2,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::Bge { rs1, rs2, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Sge,
                    op1: rs1,
                    op2: rs2,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::Bgeu { rs1, rs2, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Uge,
                    op1: rs1,
                    op2: rs2,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::Blt { rs1, rs2, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Slt,
                    op1: rs1,
                    op2: rs2,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::Bltu { rs1, rs2, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Ult,
                    op1: rs1,
                    op2: rs2,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::Bne { rs1, rs2, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Ne,
                    op1: rs1,
                    op2: rs2,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::Ebreak { .. } => todo!(),
            RiscvInstruction::Ecall { .. } => todo!(),
            RiscvInstruction::Jal { comment, .. } | RiscvInstruction::Jalr { comment, .. } => {
                let caps = FUNCTION.captures(comment.as_ref().unwrap()).unwrap();
                let name = caps[1].to_string();
                vec![LlvmInstruction::Call(name)]
            }
            RiscvInstruction::Lb { rd, rs1, imm, .. } => vec![LlvmInstruction::Load {
                ty: LlvmType::I8,
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Lbu { rd, rs1, imm, .. } => vec![LlvmInstruction::Load {
                ty: LlvmType::U8,
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Ld { rd, rs1, imm, .. } => vec![LlvmInstruction::Load {
                ty: LlvmType::I64,
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Lh { rd, rs1, imm, .. } => vec![LlvmInstruction::Load {
                ty: LlvmType::I16,
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Lhu { rd, rs1, imm, .. } => vec![LlvmInstruction::Load {
                ty: LlvmType::U16,
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Lui { rd, imm, .. } => vec![LlvmInstruction::Shli12 {
                result: rd,
                op1: imm,
            }],
            RiscvInstruction::Lw { rd, rs1, imm, .. } => vec![LlvmInstruction::Load {
                ty: LlvmType::I32,
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Lwu { rd, rs1, imm, .. } => vec![LlvmInstruction::Load {
                ty: LlvmType::U32,
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Or { rd, rs1, rs2, .. } => vec![LlvmInstruction::Or {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Ori { rd, rs1, imm, .. } => vec![LlvmInstruction::Ori {
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Sb { rs1, imm, rs2, .. } => vec![LlvmInstruction::Save {
                ty: LlvmType::I8,
                op1: rs1,
                op2: imm,
                source: rs2,
            }],
            RiscvInstruction::Sd { rs1, imm, rs2, .. } => vec![LlvmInstruction::Save {
                ty: LlvmType::I64,
                op1: rs1,
                op2: imm,
                source: rs2,
            }],
            RiscvInstruction::Sh { rs1, imm, rs2, .. } => vec![LlvmInstruction::Save {
                ty: LlvmType::I16,
                op1: rs1,
                op2: imm,
                source: rs2,
            }],
            RiscvInstruction::Sll { rd, rs1, rs2, .. }
            | RiscvInstruction::Sllw { rd, rs1, rs2, .. } => vec![LlvmInstruction::Shl {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Slli { rd, rs1, imm, .. }
            | RiscvInstruction::Slliw { rd, rs1, imm, .. } => vec![LlvmInstruction::Shli {
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Slt { .. }
            | RiscvInstruction::Slti { .. }
            | RiscvInstruction::Sltiu { .. }
            | RiscvInstruction::Sltu { .. } => todo!(),
            RiscvInstruction::Sra { rd, rs1, rs2, .. }
            | RiscvInstruction::Sraw { rd, rs1, rs2, .. } => vec![LlvmInstruction::Ashr {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Srai { rd, rs1, imm, .. }
            | RiscvInstruction::Sraiw { rd, rs1, imm, .. } => vec![LlvmInstruction::Ashri {
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Srl { rd, rs1, rs2, .. }
            | RiscvInstruction::Srlw { rd, rs1, rs2, .. } => vec![LlvmInstruction::Lshr {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Srli { rd, rs1, imm, .. }
            | RiscvInstruction::Srliw { rd, rs1, imm, .. } => vec![LlvmInstruction::Lshri {
                result: rd,
                op1: rs1,
                op2: imm,
            }],
            RiscvInstruction::Sub { rd, rs1, rs2, .. }
            | RiscvInstruction::Subw { rd, rs1, rs2, .. } => vec![LlvmInstruction::Sub {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Sw { rs1, imm, rs2, .. } => vec![LlvmInstruction::Save {
                ty: LlvmType::I32,
                op1: rs1,
                op2: imm,
                source: rs2,
            }],
            RiscvInstruction::Xor { rd, rs1, rs2, .. } => vec![LlvmInstruction::Xor {
                result: rd,
                op1: rs1,
                op2: rs2,
            }],
            RiscvInstruction::Xori { rd, rs1, imm, .. } => vec![LlvmInstruction::Xori {
                result: rd,
                op1: rs1,
                op2: imm,
            }],

            // Pseudo
            RiscvInstruction::Beqz { rs1, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Eq,
                    op1: rs1,
                    op2: RiscvRegister::Zero,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::Bnez { rs1, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Ne,
                    op1: rs1,
                    op2: RiscvRegister::Zero,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
            RiscvInstruction::J { addr, .. } => {
                let target = if addr == 0 {
                    continue_target.unwrap()
                } else {
                    jump_target.unwrap()
                };
                vec![LlvmInstruction::DirectBr(format!("L{}", target))]
            }
            RiscvInstruction::Jr { rs1, .. } => vec![LlvmInstruction::Switch {
                register: rs1,
                targets: potential_targets.clone(),
            }],
            RiscvInstruction::Li { rd, imm, .. } => vec![LlvmInstruction::Addi {
                result: rd,
                op1: RiscvRegister::Zero,
                op2: imm,
            }],
            RiscvInstruction::Mv { rd, rs1, .. } => vec![LlvmInstruction::Add {
                result: rd,
                op1: RiscvRegister::Zero,
                op2: rs1,
            }],
            RiscvInstruction::Neg { rd, rs1, .. } => vec![LlvmInstruction::Sub {
                result: rd,
                op1: RiscvRegister::Zero,
                op2: rs1,
            }],
            RiscvInstruction::Nop { .. } => Vec::new(),
            RiscvInstruction::Not { rd, rs1, .. } => vec![LlvmInstruction::Xori {
                result: rd,
                op1: rs1,
                op2: -1,
            }],
            RiscvInstruction::Ret { .. } => vec![LlvmInstruction::Ret],
            RiscvInstruction::Seqz { .. } => todo!(),
            RiscvInstruction::Snez { .. } => todo!(),

            // Misc
            RiscvInstruction::SextW { rd, rs1, .. } => vec![LlvmInstruction::Addi {
                result: rd,
                op1: rs1,
                op2: 0,
            }],
            RiscvInstruction::Blez { rs1, .. } => vec![
                LlvmInstruction::Icmp {
                    condition: LlvmCondition::Sle,
                    op1: rs1,
                    op2: RiscvRegister::Zero,
                },
                LlvmInstruction::Br {
                    iftrue: format!("L{}", jump_target.unwrap()),
                    iffalse: format!("L{}", continue_target.unwrap()),
                },
            ],
        };
        llvm_insts.extend(insts);
    }
    llvm_insts
}
