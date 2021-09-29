use crate::cfg::{BasicBlock, Cfg, RiscvFunction};
use crate::llvm_isa::{
    LlvmFunction, LlvmInstruction, LlvmIntCondition, LlvmOrdering, LlvmType, LlvmValue, Program,
};
use crate::riscv_isa::{RiscvAddress, RiscvImmediate, RiscvInstruction, RiscvRegister};
use regex::Regex;
use std::collections::HashMap;
use std::mem;

pub struct LlvmTranslator {
    cfg: Cfg,
    statics: HashMap<String, (String, LlvmType)>,
    temp: usize,
    lr_value: LlvmValue,
}

impl LlvmTranslator {
    pub fn new(cfg: Cfg, statics: HashMap<String, (String, LlvmType)>) -> Self {
        LlvmTranslator {
            cfg,
            statics,
            temp: 0,
            lr_value: 0_usize.into(),
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
                RI::Jal { comment, .. } | RI::Jalr { comment, .. } => {
                    lazy_static! {
                        static ref FUNCTION: Regex = Regex::new(r"<(.+)>").unwrap();
                    }
                    let caps = FUNCTION.captures(comment.as_ref().unwrap()).unwrap();
                    let name = caps[1].to_string();
                    vec![LI::Call(name)]
                }
                RI::Beq { rs1, rs2, .. } => self.build_branch(
                    rs1,
                    rs2,
                    LlvmIntCondition::Eq,
                    jump_target.unwrap(),
                    continue_target.unwrap(),
                ),
                RI::Bne { rs1, rs2, .. } => self.build_branch(
                    rs1,
                    rs2,
                    LlvmIntCondition::Ne,
                    jump_target.unwrap(),
                    continue_target.unwrap(),
                ),
                RI::Blt { rs1, rs2, .. } => self.build_branch(
                    rs1,
                    rs2,
                    LlvmIntCondition::Slt,
                    jump_target.unwrap(),
                    continue_target.unwrap(),
                ),
                RI::Bge { rs1, rs2, .. } => self.build_branch(
                    rs1,
                    rs2,
                    LlvmIntCondition::Sge,
                    jump_target.unwrap(),
                    continue_target.unwrap(),
                ),
                RI::Bltu { rs1, rs2, .. } => self.build_branch(
                    rs1,
                    rs2,
                    LlvmIntCondition::Ult,
                    jump_target.unwrap(),
                    continue_target.unwrap(),
                ),
                RI::Bgeu { rs1, rs2, .. } => self.build_branch(
                    rs1,
                    rs2,
                    LlvmIntCondition::Uge,
                    jump_target.unwrap(),
                    continue_target.unwrap(),
                ),
                RI::Lb {
                    rd,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_load(rd, imm, rs1, comment, LlvmType::I8, true),
                RI::Lh {
                    rd,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_load(rd, imm, rs1, comment, LlvmType::I16, true),
                RI::Lw {
                    rd,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_load(rd, imm, rs1, comment, LlvmType::I32, true),
                RI::Lbu {
                    rd,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_load(rd, imm, rs1, comment, LlvmType::I8, false),
                RI::Lhu {
                    rd,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_load(rd, imm, rs1, comment, LlvmType::I16, false),
                RI::Sb {
                    rs2,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_store(rs2, imm, rs1, comment, LlvmType::I8),
                RI::Sh {
                    rs2,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_store(rs2, imm, rs1, comment, LlvmType::I16),
                RI::Sw {
                    rs2,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_store(rs2, imm, rs1, comment, LlvmType::I32),
                RI::Addi { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Addi", false)
                }
                RI::Slti { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Slti", false)
                }
                RI::Sltiu { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Sltiu", false)
                }
                RI::Xori { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Xori", false)
                }
                RI::Ori { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Ori", false)
                }
                RI::Andi { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Andi", false)
                }
                RI::Slli { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Slli", false)
                }
                RI::Srli { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Srli", false)
                }
                RI::Srai { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Srai", false)
                }
                RI::Add { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Add", false),
                RI::Sub { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Sub", false),
                RI::Sll { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Sll", false),
                RI::Slt { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Slt", false),
                RI::Sltu { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Sltu", false),
                RI::Xor { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Xor", false),
                RI::Srl { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Srl", false),
                RI::Sra { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Sra", false),
                RI::Or { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Or", false),
                RI::And { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "And", false),
                RI::Fence { .. } => vec![LI::Fence(LlvmOrdering::AcqRel)],
                RI::Ecall { .. } => {
                    let temps = self.get_temps(8);
                    vec![
                        LI::Load {
                            result: temps[0].clone(),
                            ty: LlvmType::I64,
                            pointer: RiscvRegister::A7.into(),
                        },
                        LI::Load {
                            result: temps[1].clone(),
                            ty: LlvmType::I64,
                            pointer: RiscvRegister::A0.into(),
                        },
                        LI::Load {
                            result: temps[2].clone(),
                            ty: LlvmType::I64,
                            pointer: RiscvRegister::A1.into(),
                        },
                        LI::Load {
                            result: temps[3].clone(),
                            ty: LlvmType::I64,
                            pointer: RiscvRegister::A2.into(),
                        },
                        LI::Load {
                            result: temps[4].clone(),
                            ty: LlvmType::I64,
                            pointer: RiscvRegister::A3.into(),
                        },
                        LI::Load {
                            result: temps[5].clone(),
                            ty: LlvmType::I64,
                            pointer: RiscvRegister::A4.into(),
                        },
                        LI::Load {
                            result: temps[6].clone(),
                            ty: LlvmType::I64,
                            pointer: RiscvRegister::A5.into(),
                        },
                        LI::Syscall {
                            result: temps[7].clone(),
                            no: temps[0].clone(),
                            arg1: temps[1].clone(),
                            arg2: temps[2].clone(),
                            arg3: temps[3].clone(),
                            arg4: temps[4].clone(),
                            arg5: temps[5].clone(),
                            arg6: temps[6].clone(),
                        },
                        LI::Store {
                            ty: LlvmType::I64,
                            value: temps[7].clone(),
                            pointer: RiscvRegister::A0.into(),
                        },
                    ]
                }
                RI::Ebreak { .. } => panic!("RISC-V `ebreak` instruction is not supported"),

                // RV64I
                RI::Lwu {
                    rd,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_load(rd, imm, rs1, comment, LlvmType::I32, false),
                RI::Ld {
                    rd,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_load(rd, imm, rs1, comment, LlvmType::I64, true),
                RI::Sd {
                    rs2,
                    imm,
                    rs1,
                    comment,
                    ..
                } => self.build_store(rs2, imm, rs1, comment, LlvmType::I64),
                RI::Addiw { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Addiw", true)
                }
                RI::Slliw { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Slliw", true)
                }
                RI::Srliw { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Srliw", true)
                }
                RI::Sraiw { rd, rs1, imm, .. } => {
                    self.build_binary_immediate(rd, rs1, imm, "Sraiw", true)
                }
                RI::Addw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Addw", true),
                RI::Subw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Subw", true),
                RI::Sllw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Sllw", true),
                RI::Srlw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Srlw", true),
                RI::Sraw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Sraw", true),

                // RV32M
                RI::Mul { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Mul", false),
                RI::Mulh { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Mulh", false),
                RI::Mulhsu { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Mulhsu", false),
                RI::Mulhu { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Mulhu", false),
                RI::Div { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Div", false),
                RI::Divu { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Divu", false),
                RI::Rem { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Rem", false),
                RI::Remu { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Remu", false),

                // RV64M
                RI::Mulw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Mulw", true),
                RI::Divw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Divw", true),
                RI::Divuw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Divuw", true),
                RI::Remw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Remw", true),
                RI::Remuw { rd, rs1, rs2, .. } => self.build_binary(rd, rs1, rs2, "Remuw", true),

                // RV32A
                RI::LrW { ..} => todo!(),
                RI::ScW { ..} => todo!(),
                RI::AmoswapW { ..} => todo!(),
                RI::AmoaddW { ..} => todo!(),
                RI::AmoxorW { ..} => todo!(),
                RI::AmoandW { ..} => todo!(),
                RI::AmoorW { ..} => todo!(),
                RI::AmominW { ..} => todo!(),
                RI::AmomaxW { ..} => todo!(),
                RI::AmominuW { ..} => todo!(),
                RI::AmomaxuW { ..} => todo!(),

                // RV64A
                RI::LrD { ..} => todo!(),
                RI::ScD { ..} => todo!(),
                RI::AmoswapD { ..} => todo!(),
                RI::AmoaddD { ..} => todo!(),
                RI::AmoxorD { ..} => todo!(),
                RI::AmoandD { ..} => todo!(),
                RI::AmoorD { ..} => todo!(),
                RI::AmominD { ..} => todo!(),
                RI::AmomaxD { ..} => todo!(),
                RI::AmominuD { ..} => todo!(),
                RI::AmomaxuD { ..} => todo!(),

                // RV32F
                

                RI::Ret { .. } => vec![LI::Ret],
                _ => vec![],
            })
            .flatten()
            .collect()
    }

    fn build_branch(
        &mut self,
        rs1: RiscvRegister,
        rs2: RiscvRegister,
        cond: LlvmIntCondition,
        jump_target: usize,
        continue_target: usize,
    ) -> Vec<LlvmInstruction> {
        use LlvmInstruction::*;
        let temps = self.get_temps(3);
        vec![
            Load {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                pointer: rs1.into(),
            },
            Load {
                result: temps[1].clone(),
                ty: LlvmType::I64,
                pointer: rs2.into(),
            },
            Icmp {
                result: temps[2].clone(),
                cond,
                ty: LlvmType::I64,
                op1: temps[0].clone(),
                op2: temps[1].clone(),
            },
            ConBr {
                cond: temps[2].clone(),
                iftrue: format!("L{}", jump_target),
                iffalse: format!("L{}", continue_target),
            },
        ]
    }

    fn build_load(
        &mut self,
        rd: RiscvRegister,
        imm: RiscvImmediate,
        rs1: RiscvRegister,
        comment: Option<String>,
        ty: LlvmType,
        signed: bool,
    ) -> Vec<LlvmInstruction> {
        use LlvmInstruction::*;

        lazy_static! {
            static ref STATIC: Regex = Regex::new(r"<(.+)>").unwrap();
        }
        if let Some(comment) = comment {
            let caps = STATIC.captures(&comment).unwrap();
            if let Some(name) = caps.get(1) {
                if let LlvmType::I64 = ty {
                    let temps = self.get_temps(1);
                    return vec![
                        Load {
                            result: temps[0].clone(),
                            ty: ty.clone(),
                            pointer: LlvmValue::GlobalVar(name.as_str().to_string()),
                        },
                        Store {
                            ty: LlvmType::I64,
                            value: temps[0].clone(),
                            pointer: rd.into(),
                        },
                    ];
                } else {
                    let temps = self.get_temps(2);
                    return vec![
                        Load {
                            result: temps[0].clone(),
                            ty: ty.clone(),
                            pointer: LlvmValue::GlobalVar(name.as_str().to_string()),
                        },
                        match signed {
                            true => Sext {
                                result: temps[1].clone(),
                                ty: ty.clone(),
                                value: temps[0].clone(),
                                ty2: LlvmType::I64,
                            },
                            false => Zext {
                                result: temps[1].clone(),
                                ty: ty.clone(),
                                value: temps[0].clone(),
                                ty2: LlvmType::I64,
                            },
                        },
                        Store {
                            ty: LlvmType::I64,
                            value: temps[1].clone(),
                            pointer: rd.into(),
                        },
                    ];
                }
            }
        }

        match ty {
            LlvmType::I8 => {
                let temps = self.get_temps(5);
                vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Add {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        op1: temps[0].clone(),
                        op2: imm.into(),
                    },
                    Getelementptr {
                        result: temps[2].clone(),
                        index: temps[1].clone(),
                    },
                    Load {
                        result: temps[3].clone(),
                        ty: ty.clone(),
                        pointer: temps[2].clone(),
                    },
                    match signed {
                        true => Sext {
                            result: temps[4].clone(),
                            ty: ty.clone(),
                            value: temps[3].clone(),
                            ty2: LlvmType::I64,
                        },
                        false => Zext {
                            result: temps[4].clone(),
                            ty: ty.clone(),
                            value: temps[3].clone(),
                            ty2: LlvmType::I64,
                        },
                    },
                    Store {
                        ty: LlvmType::I64,
                        value: temps[4].clone(),
                        pointer: rd.into(),
                    },
                ]
            }
            LlvmType::I64 => {
                let temps = self.get_temps(5);
                vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Add {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        op1: temps[0].clone(),
                        op2: imm.into(),
                    },
                    Getelementptr {
                        result: temps[2].clone(),
                        index: temps[1].clone(),
                    },
                    Bitcast {
                        result: temps[3].clone(),
                        ty: LlvmType::I8,
                        value: temps[2].clone(),
                        ty2: ty.clone(),
                    },
                    Load {
                        result: temps[4].clone(),
                        ty: ty.clone(),
                        pointer: temps[3].clone(),
                    },
                    Store {
                        ty: LlvmType::I64,
                        value: temps[4].clone(),
                        pointer: rd.into(),
                    },
                ]
            }
            _ => {
                let temps = self.get_temps(6);
                vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Add {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        op1: temps[0].clone(),
                        op2: imm.into(),
                    },
                    Getelementptr {
                        result: temps[2].clone(),
                        index: temps[1].clone(),
                    },
                    Bitcast {
                        result: temps[3].clone(),
                        ty: LlvmType::I8,
                        value: temps[2].clone(),
                        ty2: ty.clone(),
                    },
                    Load {
                        result: temps[4].clone(),
                        ty: ty.clone(),
                        pointer: temps[3].clone(),
                    },
                    match signed {
                        true => Sext {
                            result: temps[5].clone(),
                            ty: ty.clone(),
                            value: temps[4].clone(),
                            ty2: LlvmType::I64,
                        },
                        false => Zext {
                            result: temps[5].clone(),
                            ty: ty.clone(),
                            value: temps[4].clone(),
                            ty2: LlvmType::I64,
                        },
                    },
                    Store {
                        ty: LlvmType::I64,
                        value: temps[5].clone(),
                        pointer: rd.into(),
                    },
                ]
            }
        }
    }

    fn build_store(
        &mut self,
        rs2: RiscvRegister,
        imm: RiscvImmediate,
        rs1: RiscvRegister,
        comment: Option<String>,
        ty: LlvmType,
    ) -> Vec<LlvmInstruction> {
        use LlvmInstruction::*;

        lazy_static! {
            static ref STATIC: Regex = Regex::new(r"<(.+)>").unwrap();
        }
        if let Some(comment) = comment {
            let caps = STATIC.captures(&comment).unwrap();
            if let Some(name) = caps.get(1) {
                if let LlvmType::I64 = ty {
                    let temps = self.get_temps(1);
                    return vec![
                        Load {
                            result: temps[0].clone(),
                            ty: LlvmType::I64,
                            pointer: rs2.into(),
                        },
                        Store {
                            ty: ty.clone(),
                            value: temps[0].clone(),
                            pointer: LlvmValue::GlobalVar(name.as_str().to_string()),
                        },
                    ];
                } else {
                    let temps = self.get_temps(2);
                    return vec![
                        Load {
                            result: temps[0].clone(),
                            ty: LlvmType::I64,
                            pointer: rs2.into(),
                        },
                        Trunc {
                            result: temps[1].clone(),
                            ty: LlvmType::I64,
                            value: temps[0].clone(),
                            ty2: ty.clone(),
                        },
                        Store {
                            ty: ty.clone(),
                            value: temps[1].clone(),
                            pointer: LlvmValue::GlobalVar(name.as_str().to_string()),
                        },
                    ];
                }
            }
        }

        match ty {
            LlvmType::I8 => {
                let temps = self.get_temps(5);
                vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Add {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        op1: temps[0].clone(),
                        op2: imm.into(),
                    },
                    Getelementptr {
                        result: temps[2].clone(),
                        index: temps[1].clone(),
                    },
                    Load {
                        result: temps[3].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                    Trunc {
                        result: temps[4].clone(),
                        ty: LlvmType::I64,
                        value: temps[3].clone(),
                        ty2: ty.clone(),
                    },
                    Store {
                        ty: ty.clone(),
                        value: temps[4].clone(),
                        pointer: temps[2].clone(),
                    },
                ]
            }
            LlvmType::I64 => {
                let temps = self.get_temps(5);
                vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Add {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        op1: temps[0].clone(),
                        op2: imm.into(),
                    },
                    Getelementptr {
                        result: temps[2].clone(),
                        index: temps[1].clone(),
                    },
                    Bitcast {
                        result: temps[3].clone(),
                        ty: LlvmType::I8,
                        value: temps[2].clone(),
                        ty2: ty.clone(),
                    },
                    Load {
                        result: temps[4].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                    Store {
                        ty: ty.clone(),
                        value: temps[4].clone(),
                        pointer: temps[3].clone(),
                    },
                ]
            }
            _ => {
                let temps = self.get_temps(6);
                vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Add {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        op1: temps[0].clone(),
                        op2: imm.into(),
                    },
                    Getelementptr {
                        result: temps[2].clone(),
                        index: temps[1].clone(),
                    },
                    Bitcast {
                        result: temps[3].clone(),
                        ty: LlvmType::I8,
                        value: temps[2].clone(),
                        ty2: ty.clone(),
                    },
                    Load {
                        result: temps[4].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                    Trunc {
                        result: temps[5].clone(),
                        ty: LlvmType::I64,
                        value: temps[4].clone(),
                        ty2: ty.clone(),
                    },
                    Store {
                        ty: ty.clone(),
                        value: temps[5].clone(),
                        pointer: temps[3].clone(),
                    },
                ]
            }
        }
    }

    fn build_binary_immediate(
        &mut self,
        rd: RiscvRegister,
        rs1: RiscvRegister,
        imm: RiscvImmediate,
        op: &str,
        word: bool,
    ) -> Vec<LlvmInstruction> {
        use LlvmInstruction::*;

        let (op1, mut insts) = match word {
            true => {
                let temps = self.get_temps(2);
                let insts = vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Trunc {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        value: temps[0].clone(),
                        ty2: LlvmType::I32,
                    },
                ];
                (temps[1].clone(), insts)
            }
            false => {
                let temps = self.get_temps(1);
                let insts = vec![Load {
                    result: temps[0].clone(),
                    ty: LlvmType::I64,
                    pointer: rs1.into(),
                }];
                (temps[0].clone(), insts)
            }
        };

        let temps = self.get_temps(1);
        insts.push(match op {
            "Addi" => Add {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Slti" => Icmp {
                result: temps[0].clone(),
                cond: LlvmIntCondition::Slt,
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Sltiu" => Icmp {
                result: temps[0].clone(),
                cond: LlvmIntCondition::Ult,
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Xori" => Xor {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Ori" => Or {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Andi" => And {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Slli" => Shl {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Srli" => Lshr {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Srai" => Ashr {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2: imm.into(),
            },
            "Addiw" => Add {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2: imm.into(),
            },
            "Slliw" => Shl {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2: imm.into(),
            },
            "Srliw" => Lshr {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2: imm.into(),
            },
            "Sraiw" => Ashr {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2: imm.into(),
            },
            _ => unreachable!(),
        });

        let value = temps[0].clone();
        if word {
            let temps = self.get_temps(1);
            insts.extend(vec![
                Sext {
                    result: temps[0].clone(),
                    ty: LlvmType::I32,
                    value,
                    ty2: LlvmType::I64,
                },
                Store {
                    ty: LlvmType::I64,
                    value: temps[0].clone(),
                    pointer: rd.into(),
                },
            ]);
        } else {
            insts.push(Store {
                ty: LlvmType::I64,
                value,
                pointer: rd.into(),
            })
        }

        insts
    }

    fn build_binary(
        &mut self,
        rd: RiscvRegister,
        rs1: RiscvRegister,
        rs2: RiscvRegister,
        op: &str,
        word: bool,
    ) -> Vec<LlvmInstruction> {
        use LlvmInstruction::*;

        let (op1, op2, mut insts) = match (op, word) {
            ("Mulh", _) => {
                let temps = self.get_temps(4);
                let insts = vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Sext {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        value: temps[0].clone(),
                        ty2: LlvmType::I128,
                    },
                    Load {
                        result: temps[2].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                    Sext {
                        result: temps[3].clone(),
                        ty: LlvmType::I64,
                        value: temps[2].clone(),
                        ty2: LlvmType::I128,
                    },
                ];
                (temps[1].clone(), temps[3].clone(), insts)
            }
            ("Mulhsu", _) => {
                let temps = self.get_temps(4);
                let insts = vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Sext {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        value: temps[0].clone(),
                        ty2: LlvmType::I128,
                    },
                    Load {
                        result: temps[2].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                    Zext {
                        result: temps[3].clone(),
                        ty: LlvmType::I64,
                        value: temps[2].clone(),
                        ty2: LlvmType::I128,
                    },
                ];
                (temps[1].clone(), temps[3].clone(), insts)
            }
            ("Mulhu", _) => {
                let temps = self.get_temps(4);
                let insts = vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Zext {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        value: temps[0].clone(),
                        ty2: LlvmType::I128,
                    },
                    Load {
                        result: temps[2].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                    Zext {
                        result: temps[3].clone(),
                        ty: LlvmType::I64,
                        value: temps[2].clone(),
                        ty2: LlvmType::I128,
                    },
                ];
                (temps[1].clone(), temps[3].clone(), insts)
            }
            (_, true) => {
                let temps = self.get_temps(4);
                let insts = vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Trunc {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        value: temps[0].clone(),
                        ty2: LlvmType::I32,
                    },
                    Load {
                        result: temps[2].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                    Trunc {
                        result: temps[3].clone(),
                        ty: LlvmType::I64,
                        value: temps[2].clone(),
                        ty2: LlvmType::I32,
                    },
                ];
                (temps[1].clone(), temps[3].clone(), insts)
            }
            (_, false) => {
                let temps = self.get_temps(2);
                let insts = vec![
                    Load {
                        result: temps[0].clone(),
                        ty: LlvmType::I64,
                        pointer: rs1.into(),
                    },
                    Load {
                        result: temps[1].clone(),
                        ty: LlvmType::I64,
                        pointer: rs2.into(),
                    },
                ];
                (temps[0].clone(), temps[1].clone(), insts)
            }
        };

        let temps = self.get_temps(1);
        insts.push(match op {
            "Add" => Add {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Sub" => Sub {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Sll" => Shl {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Slt" => Icmp {
                result: temps[0].clone(),
                cond: LlvmIntCondition::Slt,
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Sltu" => Icmp {
                result: temps[0].clone(),
                cond: LlvmIntCondition::Ult,
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Xor" => Xor {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Srl" => Lshr {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Sra" => Ashr {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Or" => Or {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "And" => And {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Addw" => Add {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Subw" => Sub {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Sllw" => Shl {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Srlw" => Lshr {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Sraw" => Ashr {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Mul" => Mul {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Mulh" | "Mulhsu" | "Mulhu" => Mul {
                result: temps[0].clone(),
                ty: LlvmType::I128,
                op1,
                op2,
            },
            "Div" => Sdiv {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Divu" => Udiv {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Rem" => Srem {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Remu" => Urem {
                result: temps[0].clone(),
                ty: LlvmType::I64,
                op1,
                op2,
            },
            "Mulw" => Mul {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Divw" => Sdiv {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Divuw" => Udiv {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Remw" => Srem {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            "Remuw" => Urem {
                result: temps[0].clone(),
                ty: LlvmType::I32,
                op1,
                op2,
            },
            _ => unreachable!(),
        });

        let value = temps[0].clone();
        if let "Mulh" | "Mulhsu" | "Mulhu" = op {
            let temps = self.get_temps(2);
            insts.extend(vec![
                Lshr {
                    result: temps[0].clone(),
                    ty: LlvmType::I128,
                    op1: value,
                    op2: 64_i64.into(),
                },
                Trunc {
                    result: temps[1].clone(),
                    ty: LlvmType::I128,
                    value: temps[0].clone(),
                    ty2: LlvmType::I64,
                },
                Store {
                    ty: LlvmType::I64,
                    value: temps[1].clone(),
                    pointer: rd.into(),
                },
            ]);
        } else if word {
            let temps = self.get_temps(1);
            insts.extend(vec![
                Sext {
                    result: temps[0].clone(),
                    ty: LlvmType::I32,
                    value,
                    ty2: LlvmType::I64,
                },
                Store {
                    ty: LlvmType::I64,
                    value: temps[0].clone(),
                    pointer: rd.into(),
                },
            ]);
        } else {
            insts.push(Store {
                ty: LlvmType::I64,
                value,
                pointer: rd.into(),
            });
        }

        insts
    }
}

#[cfg(test)]
mod tests {
    use super::LlvmTranslator;
    use crate::cfg_builder::CfgBuilder;
    use crate::llvm_isa::{
        LlvmFunction, LlvmInstruction::*, LlvmIntCondition, LlvmOrdering, LlvmType, LlvmValue,
        Program,
    };
    use crate::riscv_isa::RiscvRegister::*;
    use crate::riscv_parser;
    use std::collections::HashMap;

    macro_rules! build_test {
        ( $( $func:ident ( $source:literal, [ $( $inst:expr, )* ] ), )* ) => {
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
                                Ret,
                            ],
                        }]
                    };
                    assert_eq!(ll_program, expected);
                }
            )*
        };
    }

    // RV32I (47 tests)
    build_test! {
        lui("lui	a0,0x12", [
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
            },
        ]),

        auipc("auipc	a0,0x0", []),
    }

    #[test]
    fn jal() {
        let source = "
            Disassembly of section .text:

            00000000000104e0 <f>:
                104e0:	8082                	ret

            00000000000104ee <main>:
                104ee:	febff0ef          	jal	ra,104e0 <f>
                10504:	8082                	ret
        ";
        let indirect_targets = riscv_parser::parse_rodata(source);
        let mut statics = riscv_parser::parse_sdata(source);
        statics.extend(riscv_parser::parse_sbss(source));
        let rv_insts = riscv_parser::parse_text(source);
        let cfg = CfgBuilder::new(rv_insts, indirect_targets).run();
        let ll_program = LlvmTranslator::new(cfg, statics).run();
        let expected = Program {
            statics: HashMap::new(),
            functions: vec![
                LlvmFunction {
                    name: String::from("f"),
                    body: vec![
                        Label(String::from("Entry")),
                        UnconBr(String::from("L0")),
                        Label(String::from("L0")),
                        Ret,
                    ],
                },
                LlvmFunction {
                    name: String::from("main"),
                    body: vec![
                        Label(String::from("Entry")),
                        UnconBr(String::from("L0")),
                        Label(String::from("L0")),
                        Call(String::from("f")),
                        Ret,
                    ],
                },
            ],
        };
        assert_eq!(ll_program, expected);
    }

    #[test]
    fn jalr() {
        let source = "
            Disassembly of section .text:

            0000000000010506 <f>:
                10506:	8082                	ret

            000000000001051c <main>:
                1051c:	00000097          	auipc	ra,0x0
                10520:	fea080e7          	jalr	-22(ra) # 10506 <f>
                1052e:	8082                	ret
        ";
        let indirect_targets = riscv_parser::parse_rodata(source);
        let mut statics = riscv_parser::parse_sdata(source);
        statics.extend(riscv_parser::parse_sbss(source));
        let rv_insts = riscv_parser::parse_text(source);
        let cfg = CfgBuilder::new(rv_insts, indirect_targets).run();
        let ll_program = LlvmTranslator::new(cfg, statics).run();
        let expected = Program {
            statics: HashMap::new(),
            functions: vec![
                LlvmFunction {
                    name: String::from("f"),
                    body: vec![
                        Label(String::from("Entry")),
                        UnconBr(String::from("L0")),
                        Label(String::from("L0")),
                        Ret,
                    ],
                },
                LlvmFunction {
                    name: String::from("main"),
                    body: vec![
                        Label(String::from("Entry")),
                        UnconBr(String::from("L0")),
                        Label(String::from("L0")),
                        Call(String::from("f")),
                        Ret,
                    ],
                },
            ],
        };
        assert_eq!(ll_program, expected);
    }

    build_test! {
        beq("beq	a4,a5,10556", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Eq,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            ConBr {
                cond: 2_usize.into(),
                iftrue: String::from("L0"),
                iffalse: String::from("L1"),
            },
            Label(String::from("L1")),
        ]),

        bne("bne	a4,a5,10556", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Ne,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            ConBr {
                cond: 2_usize.into(),
                iftrue: String::from("L0"),
                iffalse: String::from("L1"),
            },
            Label(String::from("L1")),
        ]),

        blt("blt	a4,a5,10556", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Slt,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            ConBr {
                cond: 2_usize.into(),
                iftrue: String::from("L0"),
                iffalse: String::from("L1"),
            },
            Label(String::from("L1")),
        ]),

        bge("bge	a4,a5,10556", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Sge,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            ConBr {
                cond: 2_usize.into(),
                iftrue: String::from("L0"),
                iffalse: String::from("L1"),
            },
            Label(String::from("L1")),
        ]),

        bltu("bltu	a4,a5,10556", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Ult,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            ConBr {
                cond: 2_usize.into(),
                iftrue: String::from("L0"),
                iffalse: String::from("L1"),
            },
            Label(String::from("L1")),
        ]),

        bgeu("bgeu	a4,a5,10556", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Uge,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            ConBr {
                cond: 2_usize.into(),
                iftrue: String::from("L0"),
                iffalse: String::from("L1"),
            },
            Label(String::from("L1")),
        ]),

        lb_global("lb	a4,48(a5) # 12030 <g1>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I8,
                pointer: LlvmValue::GlobalVar(String::from("g1")),
            },
            Sext {
                result: 1_usize.into(),
                ty: LlvmType::I8,
                value: 0_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A4.into(),
            },
        ]),

        lb("lb	a5,-20(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: (-20_i64).into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Load {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                pointer: 2_usize.into(),
            },
            Sext {
                result: 4_usize.into(),
                ty: LlvmType::I8,
                value: 3_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 4_usize.into(),
                pointer: A5.into(),
            },
        ]),

        lh_global("lh	a4,48(a5) # 12030 <g1>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I16,
                pointer: LlvmValue::GlobalVar(String::from("g1")),
            },
            Sext {
                result: 1_usize.into(),
                ty: LlvmType::I16,
                value: 0_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A4.into(),
            },
        ]),

        lh("lh	a5,-20(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: (-20_i64).into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I16,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I16,
                pointer: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I16,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: A5.into(),
            },
        ]),

        lw_global("lw	a4,48(a5) # 12030 <g1>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I32,
                pointer: LlvmValue::GlobalVar(String::from("g1")),
            },
            Sext {
                result: 1_usize.into(),
                ty: LlvmType::I32,
                value: 0_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A4.into(),
            },
        ]),

        lw("lw	a5,-20(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: (-20_i64).into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                pointer: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: A5.into(),
            },
        ]),

        lbu_global("lbu	a4,48(a5) # 12030 <g1>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I8,
                pointer: LlvmValue::GlobalVar(String::from("g1")),
            },
            Zext {
                result: 1_usize.into(),
                ty: LlvmType::I8,
                value: 0_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A4.into(),
            },
        ]),

        lbu("lbu	a5,-20(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: (-20_i64).into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Load {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                pointer: 2_usize.into(),
            },
            Zext {
                result: 4_usize.into(),
                ty: LlvmType::I8,
                value: 3_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 4_usize.into(),
                pointer: A5.into(),
            },
        ]),

        lhu_global("lhu	a4,48(a5) # 12030 <g1>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I16,
                pointer: LlvmValue::GlobalVar(String::from("g1")),
            },
            Zext {
                result: 1_usize.into(),
                ty: LlvmType::I16,
                value: 0_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A4.into(),
            },
        ]),

        lhu("lhu	a5,-20(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: (-20_i64).into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I16,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I16,
                pointer: 3_usize.into(),
            },
            Zext {
                result: 5_usize.into(),
                ty: LlvmType::I16,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: A5.into(),
            },
        ]),

        sb_global("sb	a4,52(a5) # 12034 <g2>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I8,
            },
            Store {
                ty: LlvmType::I8,
                value: 1_usize.into(),
                pointer: LlvmValue::GlobalVar(String::from("g2")),
            },
        ]),

        sb("sb	a5,48(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 48_i64.into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Load {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Trunc {
                result: 4_usize.into(),
                ty: LlvmType::I64,
                value: 3_usize.into(),
                ty2: LlvmType::I8,
            },
            Store {
                ty: LlvmType::I8,
                value: 4_usize.into(),
                pointer: 2_usize.into(),
            },
        ]),

        sh_global("sh	a4,52(a5) # 12034 <g2>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I16,
            },
            Store {
                ty: LlvmType::I16,
                value: 1_usize.into(),
                pointer: LlvmValue::GlobalVar(String::from("g2")),
            },
        ]),

        sh("sh	a5,48(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 48_i64.into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I16,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Trunc {
                result: 5_usize.into(),
                ty: LlvmType::I64,
                value: 4_usize.into(),
                ty2: LlvmType::I16,
            },
            Store {
                ty: LlvmType::I16,
                value: 5_usize.into(),
                pointer: 3_usize.into(),
            },
        ]),

        sw_global("sw	a4,52(a5) # 12034 <g2>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Store {
                ty: LlvmType::I32,
                value: 1_usize.into(),
                pointer: LlvmValue::GlobalVar(String::from("g2")),
            },
        ]),

        sw("sw	a5,48(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 48_i64.into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Trunc {
                result: 5_usize.into(),
                ty: LlvmType::I64,
                value: 4_usize.into(),
                ty2: LlvmType::I32,
            },
            Store {
                ty: LlvmType::I32,
                value: 5_usize.into(),
                pointer: 3_usize.into(),
            },
        ]),

        addi("addi	a2,sp,8", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: Sp.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 8_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A2.into(),
            },
        ]),

        slti("slti	t0,t1,0", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Icmp {
                result: 1_usize.into(),
                cond: LlvmIntCondition::Slt,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: T0.into(),
            },
        ]),

        sltiu("sltiu	t0,t1,0", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Icmp {
                result: 1_usize.into(),
                cond: LlvmIntCondition::Ult,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: T0.into(),
            },
        ]),

        xori("xori	t0,t1,0", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Xor {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: T0.into(),
            },
        ]),

        ori("ori	t0,t1,0", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Or {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: T0.into(),
            },
        ]),

        andi("andi	t0,t1,0", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            And {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: T0.into(),
            },
        ]),

        slli("slli	a4,a5,0x2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Shl {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0x2_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A4.into(),
            },
        ]),

        srli("srli	a5,a1,0x3f", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A1.into(),
            },
            Lshr {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0x3f_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A5.into(),
            },
        ]),

        srai("srai	a5,a1,0x3", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A1.into(),
            },
            Ashr {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0x3_i64.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A5.into(),
            },
        ]),

        add("add	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Add {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        sub("sub	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Sub {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        sll("sll	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Shl {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        slt("slt	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Slt,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        sltu("sltu	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Icmp {
                result: 2_usize.into(),
                cond: LlvmIntCondition::Ult,
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        xor("xor	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Xor {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        srl("srl	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Lshr {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        sra("sra	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Ashr {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        or("or	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Or {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        and("and	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            And {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        fence("fence", [
            Fence(LlvmOrdering::AcqRel),
        ]),

        ecall("ecall", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A7.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: A0.into(),
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: A1.into(),
            },
            Load {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                pointer: A2.into(),
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I64,
                pointer: A3.into(),
            },
            Load {
                result: 5_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Load {
                result: 6_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Syscall {
                result: 7_usize.into(),
                no: 0_usize.into(),
                arg1: 1_usize.into(),
                arg2: 2_usize.into(),
                arg3: 3_usize.into(),
                arg4: 4_usize.into(),
                arg5: 5_usize.into(),
                arg6: 6_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 7_usize.into(),
                pointer: A0.into(),
            },
        ]),

        // RV64I (15 tests)
        lwu_global("lwu	a4,48(a5) # 12030 <g1>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I32,
                pointer: LlvmValue::GlobalVar(String::from("g1")),
            },
            Zext {
                result: 1_usize.into(),
                ty: LlvmType::I32,
                value: 0_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 1_usize.into(),
                pointer: A4.into(),
            },
        ]),

        lwu("lwu	a5,-20(s0)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: (-20_i64).into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                pointer: 3_usize.into(),
            },
            Zext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: A5.into(),
            },
        ]),

        ld_global("ld	a4,48(a5) # 12030 <g1>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: LlvmValue::GlobalVar(String::from("g1")),
            },
            Store {
                ty: LlvmType::I64,
                value: 0_usize.into(),
                pointer: A4.into(),
            },
        ]),

        ld("ld	a1,0(sp)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: Sp.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0_i64.into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I64,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I64,
                pointer: 3_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 4_usize.into(),
                pointer: A1.into(),
            },
        ]),

        sd_global("sd	a4,52(a5) # 12034 <g2>", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A4.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 0_usize.into(),
                pointer: LlvmValue::GlobalVar(String::from("g2")),
            },
        ]),

        sd("sd	s0,0(sp)", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: Sp.into(),
            },
            Add {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 0_i64.into(),
            },
            Getelementptr {
                result: 2_usize.into(),
                index: 1_usize.into(),
            },
            Bitcast {
                result: 3_usize.into(),
                ty: LlvmType::I8,
                value: 2_usize.into(),
                ty2: LlvmType::I64,
            },
            Load {
                result: 4_usize.into(),
                ty: LlvmType::I64,
                pointer: S0.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 4_usize.into(),
                pointer: 3_usize.into(),
            },
        ]),

        addiw("addiw	t0,t1,1", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Add {
                result: 2_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 1_i64.into(),
            },
            Sext {
                result: 3_usize.into(),
                ty: LlvmType::I32,
                value: 2_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 3_usize.into(),
                pointer: T0.into(),
            },
        ]),

        slliw("slliw	a4,a5,0x2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Shl {
                result: 2_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 0x2_i64.into(),
            },
            Sext {
                result: 3_usize.into(),
                ty: LlvmType::I32,
                value: 2_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 3_usize.into(),
                pointer: A4.into(),
            },
        ]),

        srliw("srliw	a4,a5,0x2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Lshr {
                result: 2_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 0x2_i64.into(),
            },
            Sext {
                result: 3_usize.into(),
                ty: LlvmType::I32,
                value: 2_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 3_usize.into(),
                pointer: A4.into(),
            },
        ]),

        sraiw("sraiw	a4,a5,0x2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: A5.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Ashr {
                result: 2_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 0x2_i64.into(),
            },
            Sext {
                result: 3_usize.into(),
                ty: LlvmType::I32,
                value: 2_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 3_usize.into(),
                pointer: A4.into(),
            },
        ]),

        addw("addw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Add {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        subw("subw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Sub {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        sllw("sllw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Shl {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        srlw("srlw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Lshr {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        sraw("sraw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Ashr {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        // RV32M (8 tests)
        mul("mul	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Mul {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        mulh("mulh	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Sext {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I128,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Sext {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I128,
            },
            Mul {
                result: 4_usize.into(),
                ty: LlvmType::I128,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Lshr {
                result: 5_usize.into(),
                ty: LlvmType::I128,
                op1: 4_usize.into(),
                op2: 64_i64.into(),
            },
            Trunc {
                result: 6_usize.into(),
                ty: LlvmType::I128,
                value: 5_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 6_usize.into(),
                pointer: T0.into(),
            },
        ]),

        mulhsu("mulhsu	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Sext {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I128,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Zext {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I128,
            },
            Mul {
                result: 4_usize.into(),
                ty: LlvmType::I128,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Lshr {
                result: 5_usize.into(),
                ty: LlvmType::I128,
                op1: 4_usize.into(),
                op2: 64_i64.into(),
            },
            Trunc {
                result: 6_usize.into(),
                ty: LlvmType::I128,
                value: 5_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 6_usize.into(),
                pointer: T0.into(),
            },
        ]),

        mulhu("mulhu	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Zext {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I128,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Zext {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I128,
            },
            Mul {
                result: 4_usize.into(),
                ty: LlvmType::I128,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Lshr {
                result: 5_usize.into(),
                ty: LlvmType::I128,
                op1: 4_usize.into(),
                op2: 64_i64.into(),
            },
            Trunc {
                result: 6_usize.into(),
                ty: LlvmType::I128,
                value: 5_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 6_usize.into(),
                pointer: T0.into(),
            },
        ]),

        div("div	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Sdiv {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        divu("divu	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Udiv {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        rem("rem	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Srem {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        remu("remu	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Load {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Urem {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                op1: 0_usize.into(),
                op2: 1_usize.into(),
            },
            Store {
                ty: LlvmType::I64,
                value: 2_usize.into(),
                pointer: T0.into(),
            },
        ]),

        // RV64M (5 tests)
        mulw("mulw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Mul {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        divw("divw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Sdiv {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        divuw("divuw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Udiv {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        remw("remw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Srem {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),

        remuw("remuw	t0,t1,t2", [
            Load {
                result: 0_usize.into(),
                ty: LlvmType::I64,
                pointer: T1.into(),
            },
            Trunc {
                result: 1_usize.into(),
                ty: LlvmType::I64,
                value: 0_usize.into(),
                ty2: LlvmType::I32,
            },
            Load {
                result: 2_usize.into(),
                ty: LlvmType::I64,
                pointer: T2.into(),
            },
            Trunc {
                result: 3_usize.into(),
                ty: LlvmType::I64,
                value: 2_usize.into(),
                ty2: LlvmType::I32,
            },
            Urem {
                result: 4_usize.into(),
                ty: LlvmType::I32,
                op1: 1_usize.into(),
                op2: 3_usize.into(),
            },
            Sext {
                result: 5_usize.into(),
                ty: LlvmType::I32,
                value: 4_usize.into(),
                ty2: LlvmType::I64,
            },
            Store {
                ty: LlvmType::I64,
                value: 5_usize.into(),
                pointer: T0.into(),
            },
        ]),
    }
}
