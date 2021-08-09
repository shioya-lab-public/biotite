use crate::llvm_isa::{LlvmFunction, LlvmInstruction, Program};
use crate::riscv_isa::RiscvRegister;

const GLOBAL: &str = "@zero = global i64 0
@ra = global i64 0
@sp = global i64 0
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
    let mut program_str = GLOBAL.to_string();
    for LlvmFunction { name, body } in program {
        let mut counter = 0;
        program_str += &format!("define void @{}() {{\n", name);
        for inst in body {
            program_str += &serialize_instruction(&inst, &mut counter);
        }
        program_str.pop();
        program_str += "}\n\n";
    }
    program_str.pop();
    program_str
}

fn serialize_instruction(instruction: &LlvmInstruction, counter: &mut usize) -> String {
    use LlvmInstruction::*;

    match instruction {
        Label(label) => format!("{}:\n", label),
        Add { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = add i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Addi { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &format!(
                    "    %temp{} = add i64 %temp{}, {}\n",
                    *counter + 1,
                    *counter,
                    op2
                )
                + &serialize_register_store(*counter + 1, result)
                + "\n";
            *counter += 2;
            s
        }
        And { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = and i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Andi { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &format!(
                    "    %temp{} = and i64 %temp{}, {}\n",
                    *counter + 1,
                    *counter,
                    op2
                )
                + &serialize_register_store(*counter + 1, result)
                + "\n";
            *counter += 2;
            s
        }
        Icmp {
            condition,
            op1,
            op2,
        } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = icmp {} i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    condition,
                    *counter,
                    *counter + 1
                );
            *counter += 3;
            s
        }
        Br { iftrue, iffalse } => format!(
            "    br i1 %temp{}, label %{}, label %{}\n\n",
            *counter - 1,
            iftrue,
            iffalse
        ),
        DirectBr(label) => format!("    br label %{}\n\n", label),
        IndirectBr { register, labels } => {
            let mut s = format!("    indirectbr i64* @{}, [", register);
            for label in labels {
                s += &format!("label %{}, ", label);
            }
            s.pop();
            s.pop();
            s += "]\n\n";
            s
        }
        Call(func) => format!("    call void @{}()\n\n", func),
        Load {
            ty,
            result,
            op1,
            op2,
        } => {
            let s =
                serialize_register_load(*counter, op1)
                    + &format!(
                        "    %temp{} = add i64 %temp{}, {}\n",
                        *counter + 1,
                        *counter,
                        -op2
                    )
                    + &format!(
                    "    %temp{} = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 {}\n",
                    *counter + 2,
                    *counter + 1,
                ) + &format!(
                    "    %temp{} = bitcast i8* %temp{} to {}*\n",
                    *counter + 3,
                    *counter + 2,
                    ty
                ) + &format!(
                    "    %temp{} = load {}, {}* %temp{}\n",
                    *counter + 4,
                    ty,
                    ty,
                    *counter + 3,
                ) + &serialize_register_store(*counter + 4, result)
                    + "\n";
            *counter += 5;
            s
        }
        Shli12 { result, op1 } => {
            let s = format!("    %temp{} = shli i64 %{}, 12\n", *counter, op1,)
                + &serialize_register_store(*counter, result)
                + "\n";
            *counter += 1;
            s
        }
        Or { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = or i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Ori { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &format!(
                    "    %temp{} = or i64 %temp{}, {}\n",
                    *counter + 1,
                    *counter,
                    op2
                )
                + &serialize_register_store(*counter + 1, result)
                + "\n";
            *counter += 2;
            s
        }
        Save {
            ty,
            op1,
            op2,
            source,
        } => {
            let s =
                serialize_register_load(*counter, op1)
                    + &format!(
                        "    %temp{} = add i64 %temp{}, {}\n",
                        *counter + 1,
                        *counter,
                        -op2
                    )
                    + &format!(
                    "    %temp{} = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 {}\n",
                    *counter + 2,
                    *counter + 1,
                ) + &format!(
                    "    %temp{} = bitcast i8* %temp{} to {}*\n",
                    *counter + 3,
                    *counter + 2,
                    ty
                ) + &serialize_register_load(*counter + 4, source)
                    + &format!(
                        "    store {} {}, {}* %temp{}\n",
                        ty,
                        *counter + 4,
                        ty,
                        *counter + 3,
                    )
                    + "\n";
            *counter += 5;
            s
        }
        Shl { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = shl i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Shli { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &format!(
                    "    %temp{} = shl i64 %temp{}, {}\n",
                    *counter + 1,
                    *counter,
                    op2
                )
                + &serialize_register_store(*counter + 1, result)
                + "\n";
            *counter += 2;
            s
        }
        Ashr { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = ashr i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Ashri { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &format!(
                    "    %temp{} = ashr i64 %temp{}, {}\n",
                    *counter + 1,
                    *counter,
                    op2
                )
                + &serialize_register_store(*counter + 1, result)
                + "\n";
            *counter += 2;
            s
        }
        Lshr { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = lshr i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Lshri { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &format!(
                    "    %temp{} = lshr i64 %temp{}, {}\n",
                    *counter + 1,
                    *counter,
                    op2
                )
                + &serialize_register_store(*counter + 1, result)
                + "\n";
            *counter += 2;
            s
        }
        Sub { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = sub i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Xor { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &serialize_register_load(*counter + 1, op2)
                + &format!(
                    "    %temp{} = xor i64 %temp{}, %temp{}\n",
                    *counter + 2,
                    *counter,
                    *counter + 1
                )
                + &serialize_register_store(*counter + 2, result)
                + "\n";
            *counter += 3;
            s
        }
        Xori { result, op1, op2 } => {
            let s = serialize_register_load(*counter, op1)
                + &format!(
                    "    %temp{} = xor i64 %temp{}, {}\n",
                    *counter + 1,
                    *counter,
                    op2
                )
                + &serialize_register_store(*counter + 1, result)
                + "\n";
            *counter += 2;
            s
        }
        Ret => format!("    ret\n\n"),
    }
}

fn serialize_register_load(counter: usize, register: &RiscvRegister) -> String {
    format!("    %temp{} = load i64, i64* @{}\n", counter, register)
}

fn serialize_register_store(counter: usize, register: &RiscvRegister) -> String {
    format!("    store i64 %temp{}, i64* @{}\n", counter, register)
}
