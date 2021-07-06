use crate::cfg::Cfg;
use crate::riscv_isa::RiscvDisassembly;

pub struct CfgBuilder {}

impl CfgBuilder {
    pub fn new() -> Self {
        CfgBuilder {}
    }

    pub fn run(&mut self, riscv_instructions: Vec<RiscvDisassembly>) -> Cfg {
        // self.source = source.lines().map(|line| line.to_string()).collect();
        // let func = self.translate_function("main");
        // self.output.push(func);
        // self.output.iter()
        // .map(|func| format!("{}\n", func))
        // .reduce(|funcs, func| funcs + &func)
        // .unwrap()
        todo!();
    }

    // fn translate_function(&self, name: &str) -> LlvmFunction {
    //     self.stack.clear();
    //     let riscv_insts: Vec<_> = self.source.iter()
    //         .skip_while(|line| !line.contains(&format!("<{}>:", name)))
    //         .skip(1)
    //         .take_while(|line| !line.is_empty())
    //         // .map(self.build_riscv_instruction)
    //         // .map(self.build_llvm_instruction)
    //         .collect();

    // }
}

// mod riscv_isa;
// mod llvm_isa;

// use riscv_isa::RiscvInstruction;
// use riscv_isa::RiscvRegister;
// use riscv_isa::riscv_regex;
// use riscv_isa::Stack;

// use llvm_isa::LlvmFunction;
// use llvm_isa::LlvmInstruction;
// use llvm_isa::LlvmType;
// use llvm_isa::LlvmValue;
// use llvm_isa::SSABuilder;

// fn build_riscv_instruction(line: &str) -> RiscvInstruction {
//     match line {
//         line if riscv_regex::ADDI.is_match(line) => {
//             let caps = riscv_regex::ADDI.captures(line).unwrap();
//             RiscvInstruction::Addi(
//                 build_riscv_register(&caps["rd"]),
//                 caps["imm"].parse().unwrap(),
//                 build_riscv_register(&caps["rs1"]),
//             )
//         }
//         line if riscv_regex::SD.is_match(line) => {
//             let caps = riscv_regex::SD.captures(line).unwrap();
//             RiscvInstruction::Sd(
//                 build_riscv_register(&caps["rs2"]),
//                 caps["imm"].parse().unwrap(),
//                 build_riscv_register(&caps["rs1"]),
//             )
//         }
//         line if riscv_regex::LI.is_match(line) => {
//             let caps = riscv_regex::LI.captures(line).unwrap();
//             RiscvInstruction::Li(
//                 build_riscv_register(&caps["rd"]),
//                 caps["imm"].parse().unwrap(),
//             )
//         }
//         line if riscv_regex::MV.is_match(line) => {
//             let caps = riscv_regex::MV.captures(line).unwrap();
//             RiscvInstruction::Mv(
//                 build_riscv_register(&caps["rd"]),
//                 build_riscv_register(&caps["rs1"]),
//             )
//         }
//         line if riscv_regex::LD.is_match(line) => {
//             let caps = riscv_regex::LD.captures(line).unwrap();
//             RiscvInstruction::Ld(
//                 build_riscv_register(&caps["rd"]),
//                 caps["imm"].parse().unwrap(),
//                 build_riscv_register(&caps["rs1"]),
//             )
//         }
//         line if riscv_regex::RET.is_match(line) => RiscvInstruction::Ret,
//         line => panic!("Unknown RISC-V instruction `{}`", line),
//     }
// }

// fn build_riscv_register(reg: &str) -> RiscvRegister {
//     match reg {
//         "sp" => RiscvRegister::Sp,
//         "s0" => RiscvRegister::S0,
//         "a5" => RiscvRegister::A5,
//         "a0" => RiscvRegister::A0,
//         reg => panic!("Unknown RISC-V register `{}`", reg),
//     }
// }

// fn lift_to_llvm(riscv_insts: Vec<RiscvInstruction>) -> Vec<LlvmInstruction> {
//     let mut llvm_insts = Vec::new();
//     let mut symbol_table: HashMap<RiscvRegister, (u64, LlvmType, LlvmValue)> = HashMap::new();
//     for riscv_inst in riscv_insts {
//         match riscv_inst {
//             RiscvInstruction::Addi(_rd, _imm, _rs1) => {}
//             RiscvInstruction::Sd(_rs2, _imm, _rs1) => {}
//             RiscvInstruction::Li(rd, imm) => {
//                 let sub = symbol_table
//                     .get(&rd)
//                     .map(|(sub, _, _)| sub + 1)
//                     .unwrap_or_default();
//                 symbol_table.insert(rd, (sub, LlvmType::I64, LlvmValue::Literal(imm)));
//                 llvm_insts.push(LlvmInstruction::Add(
//                     LlvmValue::Register(rd, sub),
//                     LlvmType::I64,
//                     LlvmValue::Literal(imm),
//                     LlvmValue::Literal(0),
//                 ))
//             }
//             RiscvInstruction::Mv(rd, rs1) => {
//                 let (rs1_sub, rs1_ty) = symbol_table
//                     .get(&rs1)
//                     .map(|(sub, ty, _)| (*sub, *ty))
//                     .unwrap_or_else(|| panic!("Read the uninitialized register `{:?}`", rs1));
//                 let rd_sub = symbol_table
//                     .get(&rd)
//                     .map(|(sub, _, _)| sub + 1)
//                     .unwrap_or_default();
//                 symbol_table.insert(rd, (rd_sub, rs1_ty, LlvmValue::Register(rs1, rs1_sub)));
//                 llvm_insts.push(LlvmInstruction::Add(
//                     LlvmValue::Register(rd, rd_sub),
//                     rs1_ty,
//                     LlvmValue::Register(rs1, rs1_sub),
//                     LlvmValue::Literal(0),
//                 ))
//             }
//             RiscvInstruction::Ld(_rd, _imm, _rs1) => {}
//             RiscvInstruction::Ret => match symbol_table.get(&RiscvRegister::A0) {
//                 Some((_, ty, LlvmValue::Literal(value))) => {
//                     llvm_insts.push(LlvmInstruction::Ret(*ty, LlvmValue::Literal(*value)))
//                 }
//                 Some((sub, ty, LlvmValue::Register(_, _))) => llvm_insts.push(
//                     LlvmInstruction::Ret(*ty, LlvmValue::Register(RiscvRegister::A0, *sub)),
//                 ),
//                 None => panic!("`ret void` is not supported yet."),
//             },
//         }
//     }
//     llvm_insts
// }
