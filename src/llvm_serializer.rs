use crate::llvm_isa::{LlvmFunction, LlvmInstruction, Program};
use crate::riscv_isa::RiscvRegister;

const GLOBAL: &str = "
; @.str = private unnamed_addr constant [8 x i8] c\"### %d\\0A\\00\", align 1
; declare dso_local i32 @printf(i8*, ...)
; %val = load i64, i64* @zero
; call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.str, i64 0, i64 0), i64 %val)

@zero = global i64 0
@ra = global i64 0
@sp = global i64 1023
@gp = global i64 0
@tp = global i64 0
@t0 = global i64 0
@t1 = global i64 0
@t2 = global i64 0
@s0 = global i64 0
@s1 = global i64 0
@a0 = global i64 0
@a1 = global i64 0
@a2 = global i64 0
@a3 = global i64 0
@a4 = global i64 0
@a5 = global i64 0
@a6 = global i64 0
@a7 = global i64 0
@s2 = global i64 0
@s3 = global i64 0
@s4 = global i64 0
@s5 = global i64 0
@s6 = global i64 0
@s7 = global i64 0
@s8 = global i64 0
@s9 = global i64 0
@s10 = global i64 0
@s11 = global i64 0
@t3 = global i64 0
@t4 = global i64 0
@t5 = global i64 0
@t6 = global i64 0

@stack = global [1024 x i8] zeroinitializer

";

pub fn serialize(program: Program) -> String {
    // let mut program_str = GLOBAL.to_string();
    // for LlvmFunction { name, body } in program {
    //     let mut counter = 0;
    //     program_str += &format!("define i64 @{}() {{\n", name);
    //     let mut inst_str = String::new();
    //     for inst in body {
    //         if !matches!(inst, LlvmInstruction::Switch { .. }) {
    //             program_str += &inst_str;
    //         }
    //         program_str += &format!("    ; {:?}\n", inst);
    //         inst_str = serialize_instruction(&inst, &mut counter);
    //     }
    //     program_str += &inst_str;
    //     program_str.pop();
    //     program_str += "}\n\n";
    // }
    // program_str.pop();
    // program_str
    String::new()
}

// fn serialize_instruction(instruction: &LlvmInstruction, counter: &mut usize) -> String {
//     use LlvmInstruction::*;

//     match instruction {
//         Label(label) => format!("{}:\n", label),
//         Add { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = add i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Addi { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &format!(
//                     "    %temp_{} = add i64 %temp_{}, {}\n",
//                     *counter + 1,
//                     *counter,
//                     op2
//                 )
//                 + &serialize_register_store(*counter + 1, result)
//                 + "\n";
//             *counter += 2;
//             s
//         }
//         And { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = and i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Andi { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &format!(
//                     "    %temp_{} = and i64 %temp_{}, {}\n",
//                     *counter + 1,
//                     *counter,
//                     op2
//                 )
//                 + &serialize_register_store(*counter + 1, result)
//                 + "\n";
//             *counter += 2;
//             s
//         }
//         Icmp {
//             condition,
//             op1,
//             op2,
//         } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = icmp {} i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     condition,
//                     *counter,
//                     *counter + 1
//                 )
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Br { iftrue, iffalse } => format!(
//             "    br i1 %temp_{}, label %{}, label %{}\n\n",
//             *counter - 1,
//             iftrue,
//             iffalse
//         ),
//         DirectBr(label) => format!("    br label %{}\n\n", label),
//         Switch { register, targets } => {
//             let mut s = serialize_register_load(*counter, register)
//                 + &format!(
//                     "    switch i64 %temp_{}, label %L{} [ ",
//                     *counter,
//                     *counter + 1
//                 );
//             for (addr, target) in targets {
//                 s += &format!("i64 {}, label %L{} ", addr, target);
//             }
//             s += "]\n";
//             s += &format!("L{}:\n", *counter + 1);
//             s += "    unreachable\n\n";
//             *counter += 2;
//             s
//         }
//         Call(func) => format!("    call i64 @{}()\n\n", func),
//         Load {
//             ty,
//             result,
//             op1,
//             op2,
//         } => {
//             let mut s =
//                 serialize_register_load(*counter, op1)
//                     + &format!(
//                         "    %temp_{} = add i64 %temp_{}, {}\n",
//                         *counter + 1,
//                         *counter,
//                         op2
//                     )
//                     + &format!(
//                         "    %temp_{} = sub i64 1023, %temp_{}\n",
//                         *counter + 2,
//                         *counter + 1,
//                     )
//                     + &format!(
//                     "    %temp_{} = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_{}\n",
//                     *counter + 3,
//                     *counter + 2,
//                 ) + &format!(
//                     "    %temp_{} = bitcast i8* %temp_{} to {}*\n",
//                     *counter + 4,
//                     *counter + 3,
//                     ty
//                 ) + &format!(
//                     "    %temp_{} = load {}, {}* %temp_{}\n",
//                     *counter + 5,
//                     ty,
//                     ty,
//                     *counter + 4,
//                 );
//             if &format!("{}", ty) != "i64" {
//                 s += &format!(
//                     "    %temp_{} = sext {} %temp_{} to i64\n",
//                     *counter + 6,
//                     ty,
//                     *counter + 5,
//                 );
//                 s += &serialize_register_store(*counter + 6, result);
//                 s += "\n";
//                 *counter += 7;
//             } else {
//                 s += &serialize_register_store(*counter + 5, result);
//                 s += "\n";
//                 *counter += 6;
//             }
//             s
//         }
//         Shli12 { result, op1 } => {
//             let s = format!("    %temp_{} = shl i64 {}, 12\n", *counter, op1,)
//                 + &serialize_register_store(*counter, result)
//                 + "\n";
//             *counter += 1;
//             s
//         }
//         Or { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = or i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Ori { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &format!(
//                     "    %temp_{} = or i64 %temp_{}, {}\n",
//                     *counter + 1,
//                     *counter,
//                     op2
//                 )
//                 + &serialize_register_store(*counter + 1, result)
//                 + "\n";
//             *counter += 2;
//             s
//         }
//         Save {
//             ty,
//             op1,
//             op2,
//             source,
//         } => {
//             let mut s =
//                 serialize_register_load(*counter, op1)
//                     + &format!(
//                         "    %temp_{} = add i64 %temp_{}, {}\n",
//                         *counter + 1,
//                         *counter,
//                         op2
//                     )
//                     + &format!(
//                         "    %temp_{} = sub i64 1023, %temp_{}\n",
//                         *counter + 2,
//                         *counter + 1,
//                     )
//                     + &format!(
//                     "    %temp_{} = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_{}\n",
//                     *counter + 3,
//                     *counter + 2,
//                 ) + &format!(
//                     "    %temp_{} = bitcast i8* %temp_{} to {}*\n",
//                     *counter + 4,
//                     *counter + 3,
//                     ty
//                 ) + &serialize_register_load(*counter + 5, source);
//             if &format!("{}", ty) != "i64" {
//                 s += &format!(
//                     "    %temp_{} = trunc i64 %temp_{} to {}\n",
//                     *counter + 6,
//                     *counter + 5,
//                     ty
//                 );
//                 s += &format!(
//                     "    store {} %temp_{}, {}* %temp_{}\n",
//                     ty,
//                     *counter + 6,
//                     ty,
//                     *counter + 4,
//                 );
//                 s += "\n";
//                 *counter += 7;
//             } else {
//                 s += &format!(
//                     "    store i64 %temp_{}, i64* %temp_{}\n",
//                     *counter + 5,
//                     *counter + 4,
//                 );
//                 s += "\n";
//                 *counter += 6;
//             }
//             s
//         }
//         Shl { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = shl i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Shli { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &format!(
//                     "    %temp_{} = shl i64 %temp_{}, {}\n",
//                     *counter + 1,
//                     *counter,
//                     op2
//                 )
//                 + &serialize_register_store(*counter + 1, result)
//                 + "\n";
//             *counter += 2;
//             s
//         }
//         Ashr { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = ashr i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Ashri { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &format!(
//                     "    %temp_{} = ashr i64 %temp_{}, {}\n",
//                     *counter + 1,
//                     *counter,
//                     op2
//                 )
//                 + &serialize_register_store(*counter + 1, result)
//                 + "\n";
//             *counter += 2;
//             s
//         }
//         Lshr { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = lshr i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Lshri { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &format!(
//                     "    %temp_{} = lshr i64 %temp_{}, {}\n",
//                     *counter + 1,
//                     *counter,
//                     op2
//                 )
//                 + &serialize_register_store(*counter + 1, result)
//                 + "\n";
//             *counter += 2;
//             s
//         }
//         Sub { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = sub i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Xor { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &serialize_register_load(*counter + 1, op2)
//                 + &format!(
//                     "    %temp_{} = xor i64 %temp_{}, %temp_{}\n",
//                     *counter + 2,
//                     *counter,
//                     *counter + 1
//                 )
//                 + &serialize_register_store(*counter + 2, result)
//                 + "\n";
//             *counter += 3;
//             s
//         }
//         Xori { result, op1, op2 } => {
//             let s = serialize_register_load(*counter, op1)
//                 + &format!(
//                     "    %temp_{} = xor i64 %temp_{}, {}\n",
//                     *counter + 1,
//                     *counter,
//                     op2
//                 )
//                 + &serialize_register_store(*counter + 1, result)
//                 + "\n";
//             *counter += 2;
//             s
//         }
//         Ret => {
//             serialize_register_load(*counter, &RiscvRegister::A0)
//                 + &format!("    ret i64 %temp_{}\n", *counter)
//                 + "\n"
//         }
//     }
// }

// fn serialize_register_load(counter: usize, register: &RiscvRegister) -> String {
//     format!("    %temp_{} = load i64, i64* @{}\n", counter, register)
// }

// fn serialize_register_store(counter: usize, register: &RiscvRegister) -> String {
//     if let RiscvRegister::Zero = register {
//         String::new()
//     } else {
//         format!("    store i64 %temp_{}, i64* @{}\n", counter, register)
//     }
// }
