use crate::riscv_isa::{
    Abi, Address, DataBlock, FPRegister, Instruction as RiscvInstruction, Register,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

const SYSCALL: &str = "
declare i{xlen} @syscall(i{xlen}, ...)
";

const FPFUNCTIONS: &str = "
declare {ftype} @llvm.sqrt.f{flen}({ftype} %op1)
declare {ftype} @llvm.fma.f{flen}({ftype} %op1, {ftype} %op2, {ftype} %op3)
declare {ftype} @llvm.fabs.f{flen}({ftype} %op1)
declare {ftype} @llvm.minimum.f{flen}({ftype} %op1, {ftype} %op2)
declare {ftype} @llvm.maximum.f{flen}({ftype} %op1, {ftype} %op2)
declare {ftype} @llvm.copysign.f{flen}({ftype} %mag, {ftype} %sgn)
";

const REGISTERS: &str = "
%zero = alloca i{xlen}
%ra = alloca i{xlen}
%sp = alloca i{xlen}
%gp = alloca i{xlen}
%tp = alloca i{xlen}
%t0 = alloca i{xlen}
%t1 = alloca i{xlen}
%t2 = alloca i{xlen}
%s0 = alloca i{xlen}
%s1 = alloca i{xlen}
%a0 = alloca i{xlen}
%a1 = alloca i{xlen}
%a2 = alloca i{xlen}
%a3 = alloca i{xlen}
%a4 = alloca i{xlen}
%a5 = alloca i{xlen}
%a6 = alloca i{xlen}
%a7 = alloca i{xlen}
%s2 = alloca i{xlen}
%s3 = alloca i{xlen}
%s4 = alloca i{xlen}
%s5 = alloca i{xlen}
%s6 = alloca i{xlen}
%s7 = alloca i{xlen}
%s8 = alloca i{xlen}
%s9 = alloca i{xlen}
%s10 = alloca i{xlen}
%s11 = alloca i{xlen}
%t3 = alloca i{xlen}
%t4 = alloca i{xlen}
%t5 = alloca i{xlen}
%t6 = alloca i{xlen}

store i{xlen} 0, i{xlen}* %zero
store i{xlen} 0, i{xlen}* %ra
store i{xlen} 0, i{xlen}* %sp
store i{xlen} 0, i{xlen}* %gp
store i{xlen} 0, i{xlen}* %tp
store i{xlen} 0, i{xlen}* %t0
store i{xlen} 0, i{xlen}* %t1
store i{xlen} 0, i{xlen}* %t2
store i{xlen} 0, i{xlen}* %s0
store i{xlen} 0, i{xlen}* %s1
store i{xlen} 0, i{xlen}* %a0
store i{xlen} 0, i{xlen}* %a1
store i{xlen} 0, i{xlen}* %a2
store i{xlen} 0, i{xlen}* %a3
store i{xlen} 0, i{xlen}* %a4
store i{xlen} 0, i{xlen}* %a5
store i{xlen} 0, i{xlen}* %a6
store i{xlen} 0, i{xlen}* %a7
store i{xlen} 0, i{xlen}* %s2
store i{xlen} 0, i{xlen}* %s3
store i{xlen} 0, i{xlen}* %s4
store i{xlen} 0, i{xlen}* %s5
store i{xlen} 0, i{xlen}* %s6
store i{xlen} 0, i{xlen}* %s7
store i{xlen} 0, i{xlen}* %s8
store i{xlen} 0, i{xlen}* %s9
store i{xlen} 0, i{xlen}* %s10
store i{xlen} 0, i{xlen}* %s11
store i{xlen} 0, i{xlen}* %t3
store i{xlen} 0, i{xlen}* %t4
store i{xlen} 0, i{xlen}* %t5
store i{xlen} 0, i{xlen}* %t6
";

const FPREGISTERS: &str = "
%ft0 = alloca {ftype}
%ft1 = alloca {ftype}
%ft2 = alloca {ftype}
%ft3 = alloca {ftype}
%ft4 = alloca {ftype}
%ft5 = alloca {ftype}
%ft6 = alloca {ftype}
%ft7 = alloca {ftype}
%fs0 = alloca {ftype}
%fs1 = alloca {ftype}
%fa0 = alloca {ftype}
%fa1 = alloca {ftype}
%fa2 = alloca {ftype}
%fa3 = alloca {ftype}
%fa4 = alloca {ftype}
%fa5 = alloca {ftype}
%fa6 = alloca {ftype}
%fa7 = alloca {ftype}
%fs2 = alloca {ftype}
%fs3 = alloca {ftype}
%fs4 = alloca {ftype}
%fs5 = alloca {ftype}
%fs6 = alloca {ftype}
%fs7 = alloca {ftype}
%fs8 = alloca {ftype}
%fs9 = alloca {ftype}
%fs10 = alloca {ftype}
%fs11 = alloca {ftype}
%ft8 = alloca {ftype}
%ft9 = alloca {ftype}
%ft10 = alloca {ftype}
%ft11 = alloca {ftype}

store {ftype} 0, {ftype}* %ft0
store {ftype} 0, {ftype}* %ft1
store {ftype} 0, {ftype}* %ft2
store {ftype} 0, {ftype}* %ft3
store {ftype} 0, {ftype}* %ft4
store {ftype} 0, {ftype}* %ft5
store {ftype} 0, {ftype}* %ft6
store {ftype} 0, {ftype}* %ft7
store {ftype} 0, {ftype}* %fs0
store {ftype} 0, {ftype}* %fs1
store {ftype} 0, {ftype}* %fa0
store {ftype} 0, {ftype}* %fa1
store {ftype} 0, {ftype}* %fa2
store {ftype} 0, {ftype}* %fa3
store {ftype} 0, {ftype}* %fa4
store {ftype} 0, {ftype}* %fa5
store {ftype} 0, {ftype}* %fa6
store {ftype} 0, {ftype}* %fa7
store {ftype} 0, {ftype}* %fs2
store {ftype} 0, {ftype}* %fs3
store {ftype} 0, {ftype}* %fs4
store {ftype} 0, {ftype}* %fs5
store {ftype} 0, {ftype}* %fs6
store {ftype} 0, {ftype}* %fs7
store {ftype} 0, {ftype}* %fs8
store {ftype} 0, {ftype}* %fs9
store {ftype} 0, {ftype}* %fs10
store {ftype} 0, {ftype}* %fs11
store {ftype} 0, {ftype}* %ft8
store {ftype} 0, {ftype}* %ft9
store {ftype} 0, {ftype}* %ft10
store {ftype} 0, {ftype}* %ft11
";

#[derive(Debug, PartialEq)]
pub struct Program {
    pub abi: Abi,
    pub data_blocks: Vec<DataBlock>,
    pub code_blocks: Vec<CodeBlock>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut program = String::new();

        let (xlen, flen, ftype) = match self.abi {
            Abi::Ilp32 => ("32", None, None),
            Abi::Ilp32f => ("32", Some("32"), Some("float")),
            Abi::Ilp32d => ("32", Some("64"), Some("double")),
            Abi::Lp64 => ("64", None, None),
            Abi::Lp64f => ("64", Some("32"), Some("float")),
            Abi::Lp64d => ("64", Some("64"), Some("double")),
        };

        program += &SYSCALL.replace("{xlen}", xlen);
        if let (Some(flen), Some(ftype)) = (flen, ftype) {
            program += &FPFUNCTIONS
                .replace("{flen}", flen)
                .replace("{ftype}", ftype);
        }
        for data_block in self.data_blocks.iter() {
            program += &format!("\n{}\n", data_block);
        }

        program += &format!(
            "\ndefine i{xlen} @main(i32 %argc, i8** %argv) {{\n",
            xlen = xlen
        );
        program += &REGISTERS.replace("{xlen}", xlen);
        if xlen == "64" {
            program += "  %argc_i64 = sext i32 %argc to i64\n";
            program += "  store i64 %argc_i64, i64* %a0\n";
        } else {
            program += "  store i32 %argc, i32* %a0\n";
        }
        if let Some(ftype) = ftype {
            program += &FPREGISTERS.replace("{ftype}", ftype);
        }
        for code_block in self.code_blocks.iter() {
            program += &format!("\n{}\n", code_block);
        }
        program += &format!("  %ret = load i{xlen}, i{xlen}* %a0", xlen = xlen);
        program += &format!("  ret i{xlen} %ret", xlen = xlen);
        program += "}\n";

        write!(f, "{}", program)
    }
}

impl Display for DataBlock {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut data_block = format!("  ; {} {} {}\n", self.address, self.section, self.symbol);
        data_block += &format!(
            "@data_{} = global [{} x i8] [",
            self.address,
            self.bytes.len()
        );
        for byte in self.bytes.iter() {
            data_block += &format!("i8 {}, ", byte);
        }
        data_block += "]\n";
        write!(f, "{}", data_block)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let Address(addr) = self;
        write!(f, "{}", addr)
    }
}

#[derive(Debug, PartialEq)]
pub struct CodeBlock {
    pub section: String,
    pub symbol: String,
    pub address: Address,
    pub instructions: Vec<InstructionBlock>,
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut code_block = format!("  ; {:?} {} {}\n", self.address, self.section, self.symbol);
        for inst_block in self.instructions.iter() {
            code_block += &format!("\n{}\n", inst_block);
        }
        write!(f, "{}", code_block)
    }
}

#[derive(Debug, PartialEq)]
pub struct InstructionBlock {
    pub riscv_instruction: RiscvInstruction,
    pub instructions: Vec<Instruction>,
}

impl Display for InstructionBlock {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut inst_block = format!("  ; {:?}\n", self.riscv_instruction);
        for inst in self.instructions.iter() {
            inst_block += &format!("{}\n", inst);
        }
        write!(f, "{}", inst_block)
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Register(Register),
    FPRegister(FPRegister),
    Temp(Address, usize),
    Integer(i64),
    Offset(u64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Value::*;

        match self {
            Register(reg) => write!(f, "%{}", reg),
            FPRegister(freg) => write!(f, "%{}", freg),
            Temp(addr, nr) => write!(f, "%inst_{}_{}", addr, nr),
            Integer(int) => write!(f, "{}", int),
            Offset(int) => write!(f, "{}", int),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Register::*;

        match self {
            Zero => write!(f, "zero"),
            Ra => write!(f, "ra"),
            Sp => write!(f, "sp"),
            Gp => write!(f, "gp"),
            Tp => write!(f, "tp"),
            T0 => write!(f, "t0"),
            T1 => write!(f, "t1"),
            T2 => write!(f, "t2"),
            S0 => write!(f, "s0"),
            S1 => write!(f, "s1"),
            A0 => write!(f, "a0"),
            A1 => write!(f, "a1"),
            A2 => write!(f, "a2"),
            A3 => write!(f, "a3"),
            A4 => write!(f, "a4"),
            A5 => write!(f, "a5"),
            A6 => write!(f, "a6"),
            A7 => write!(f, "a7"),
            S2 => write!(f, "s2"),
            S3 => write!(f, "s3"),
            S4 => write!(f, "s4"),
            S5 => write!(f, "s5"),
            S6 => write!(f, "s6"),
            S7 => write!(f, "s7"),
            S8 => write!(f, "s8"),
            S9 => write!(f, "s9"),
            S10 => write!(f, "s10"),
            S11 => write!(f, "s11"),
            T3 => write!(f, "t3"),
            T4 => write!(f, "t4"),
            T5 => write!(f, "t5"),
            T6 => write!(f, "t6"),
        }
    }
}

impl Display for FPRegister {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use FPRegister::*;

        match self {
            Ft0 => write!(f, "ft0"),
            Ft1 => write!(f, "ft1"),
            Ft2 => write!(f, "ft2"),
            Ft3 => write!(f, "ft3"),
            Ft4 => write!(f, "ft4"),
            Ft5 => write!(f, "ft5"),
            Ft6 => write!(f, "ft6"),
            Ft7 => write!(f, "ft7"),
            Fs0 => write!(f, "fs0"),
            Fs1 => write!(f, "fs1"),
            Fa0 => write!(f, "fa0"),
            Fa1 => write!(f, "fa1"),
            Fa2 => write!(f, "fa2"),
            Fa3 => write!(f, "fa3"),
            Fa4 => write!(f, "fa4"),
            Fa5 => write!(f, "fa5"),
            Fa6 => write!(f, "fa6"),
            Fa7 => write!(f, "fa7"),
            Fs2 => write!(f, "fs2"),
            Fs3 => write!(f, "fs3"),
            Fs4 => write!(f, "fs4"),
            Fs5 => write!(f, "fs5"),
            Fs6 => write!(f, "fs6"),
            Fs7 => write!(f, "fs7"),
            Fs8 => write!(f, "fs8"),
            Fs9 => write!(f, "fs9"),
            Fs10 => write!(f, "fs10"),
            Fs11 => write!(f, "fs11"),
            Ft8 => write!(f, "ft8"),
            Ft9 => write!(f, "ft9"),
            Ft10 => write!(f, "ft10"),
            Ft11 => write!(f, "ft11"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    I1,
    I8,
    I16,
    I32,
    I64,
    I128,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Type::*;

        match self {
            I1 => write!(f, "i1"),
            I8 => write!(f, "i8"),
            I16 => write!(f, "i16"),
            I32 => write!(f, "i32"),
            I64 => write!(f, "i64"),
            I128 => write!(f, "i128"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FPType {
    Float,
    Double,
}

impl FPType {
    fn flen(&self) -> u8 {
        use FPType::*;

        match self {
            Float => 32,
            Double => 64,
        }
    }
}

impl Display for FPType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use FPType::*;

        match self {
            Float => write!(f, "float"),
            Double => write!(f, "double"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Ordering {
    Monotonic,
    Acquire,
    Release,
    SeqCst,
}

impl Display for Ordering {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Ordering::*;

        match self {
            Monotonic => write!(f, "monotonic"),
            Acquire => write!(f, "acquire"),
            Release => write!(f, "release"),
            SeqCst => write!(f, "seq_cst"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Xchg,
    Add,
    And,
    Or,
    Xor,
    Max,
    Min,
    Umax,
    Umin,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Operation::*;

        match self {
            Xchg => write!(f, "xchg"),
            Add => write!(f, "add"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Xor => write!(f, "xor"),
            Max => write!(f, "max"),
            Min => write!(f, "min"),
            Umax => write!(f, "umax"),
            Umin => write!(f, "umin"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Eq,
    Ne,
    Uge,
    Ult,
    Sgt,
    Sge,
    Slt,
    Sle,
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Condition::*;

        match self {
            Eq => write!(f, "eq"),
            Ne => write!(f, "ne"),
            Uge => write!(f, "uge"),
            Ult => write!(f, "ult"),
            Sgt => write!(f, "sgt"),
            Sge => write!(f, "sge"),
            Slt => write!(f, "slt"),
            Sle => write!(f, "sle"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FPCondition {
    Oeq,
    Olt,
    Ole,
}

impl Display for FPCondition {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use FPCondition::*;

        match self {
            Oeq => write!(f, "oeq"),
            Olt => write!(f, "olt"),
            Ole => write!(f, "ole"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    // Terminator Instructions
    ConBr {
        cond: Value,
        iftrue: Address,
        iffalse: Address,
    },
    UnconBr(Address),
    Switch {
        ty: Type,
        val: Value,
        default: Address,
        targets: Vec<(Value, Address)>,
    },

    // Unary Operations
    Fneg {
        rslt: Value,
        fty: FPType,
        op1: Value,
    },

    // Binary Operations
    Add {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fadd {
        rslt: Value,
        fty: FPType,
        op1: Value,
        op2: Value,
    },
    Sub {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fsub {
        rslt: Value,
        fty: FPType,
        op1: Value,
        op2: Value,
    },
    Mul {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fmul {
        rslt: Value,
        fty: FPType,
        op1: Value,
        op2: Value,
    },
    Udiv {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Sdiv {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fdiv {
        rslt: Value,
        fty: FPType,
        op1: Value,
        op2: Value,
    },
    Urem {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Srem {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },

    // Bitwise Binary Operations
    Shl {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Lshr {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Ashr {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    And {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Or {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Xor {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },

    // Aggregate Operations
    Extractvalue {
        rslt: Value,
        ty: Type,
        val: Value,
        idx: Value,
    },

    // Memory Access and Addressing Operations
    Alloca {
        rslt: Value,
        ty: Type,
    },
    Falloca {
        rslt: Value,
        fty: FPType,
    },
    Load {
        rslt: Value,
        ty: Type,
        ptr: Value,
    },
    Fload {
        rslt: Value,
        fty: FPType,
        ptr: Value,
    },
    Store {
        ty: Type,
        val: Value,
        ptr: Value,
    },
    Fstore {
        fty: FPType,
        val: Value,
        ptr: Value,
    },
    Fence(Ordering),
    Cmpxchg {
        rslt: Value,
        ty: Type,
        ptr: Value,
        cmp: Value,
        new: Value,
        succ_ord: Ordering,
        fail_ord: Ordering,
    },
    Atomicrmw {
        rslt: Value,
        op: Operation,
        ty: Type,
        ptr: Value,
        val: Value,
        ord: Ordering,
    },
    Getelementptr {
        rslt: Value,
        len: Value,
        ptr: Value,
        idx: Value,
    },

    // Conversion Operations
    Trunc {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Zext {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Sext {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Fptrunc {
        rslt: Value,
        fty: FPType,
        val: Value,
        fty2: FPType,
    },
    Fpext {
        rslt: Value,
        fty: FPType,
        val: Value,
        fty2: FPType,
    },
    Fptoui {
        rslt: Value,
        fty: FPType,
        val: Value,
        ty: Type,
    },
    Fptosi {
        rslt: Value,
        fty: FPType,
        val: Value,
        ty: Type,
    },
    Uitofp {
        rslt: Value,
        ty: Type,
        val: Value,
        fty: FPType,
    },
    Sitofp {
        rslt: Value,
        ty: Type,
        val: Value,
        fty: FPType,
    },
    Bitcast {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },

    // Other Operations
    Icmp {
        rslt: Value,
        cond: Condition,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Fcmp {
        rslt: Value,
        fcond: FPCondition,
        fty: FPType,
        op1: Value,
        op2: Value,
    },

    // Standard C/C++ Library Intrinsics
    Sqrt {
        rslt: Value,
        fty: FPType,
        op1: Value,
    },
    Fma {
        rslt: Value,
        fty: FPType,
        op1: Value,
        op2: Value,
        op3: Value,
    },
    Fabs {
        rslt: Value,
        fty: FPType,
        op1: Value,
    },
    Minimum {
        rslt: Value,
        fty: FPType,
        op1: Value,
        op2: Value,
    },
    Maximum {
        rslt: Value,
        fty: FPType,
        op1: Value,
        op2: Value,
    },
    Copysign {
        rslt: Value,
        fty: FPType,
        mag: Value,
        sgn: Value,
    },

    // System Calls
    Syscall {
        rslt: Value,
        ty: Type,
        nr: Value,
        arg1: Value,
        arg2: Value,
        arg3: Value,
        arg4: Value,
        arg5: Value,
        arg6: Value,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Instruction::*;

        match self {
            // Terminator Instructions
            ConBr {
                cond,
                iftrue,
                iffalse,
            } => write!(
                f,
                "br i1 {}, label %label_{}, label %label_{}",
                cond, iftrue, iffalse
            ),
            UnconBr(addr) => write!(f, "br label %label_{}", addr),
            Switch {
                ty,
                val,
                default,
                targets,
            } => {
                let mut s = format!("switch {} {}, label %label_{} [", ty, val, default);
                for (val, target) in targets {
                    s += &format!("{} {}, label %label_{} ", ty, val, target);
                }
                s += "]";
                write!(f, "{}", s)
            }

            // Unary Operations
            Fneg { rslt, fty, op1 } => write!(f, "{} = fneg {} {}", rslt, fty, op1),

            // Binary Operations
            Add { rslt, ty, op1, op2 } => write!(f, "{} = add {} {}, {}", rslt, ty, op1, op2),
            Fadd {
                rslt,
                fty,
                op1,
                op2,
            } => write!(f, "{} = fadd {} {}, {}", rslt, fty, op1, op2),
            Sub { rslt, ty, op1, op2 } => write!(f, "{} = sub {} {}, {}", rslt, ty, op1, op2),
            Fsub {
                rslt,
                fty,
                op1,
                op2,
            } => write!(f, "{} = fsub {} {}, {}", rslt, fty, op1, op2),
            Mul { rslt, ty, op1, op2 } => write!(f, "{} = mul {} {}, {}", rslt, ty, op1, op2),
            Fmul {
                rslt,
                fty,
                op1,
                op2,
            } => write!(f, "{} = fmul {} {}, {}", rslt, fty, op1, op2),
            Udiv { rslt, ty, op1, op2 } => write!(f, "{} = udiv {} {}, {}", rslt, ty, op1, op2),
            Sdiv { rslt, ty, op1, op2 } => write!(f, "{} = sdiv {} {}, {}", rslt, ty, op1, op2),
            Fdiv {
                rslt,
                fty,
                op1,
                op2,
            } => write!(f, "{} = fdiv {} {}, {}", rslt, fty, op1, op2),
            Urem { rslt, ty, op1, op2 } => write!(f, "{} = urem {} {}, {}", rslt, ty, op1, op2),
            Srem { rslt, ty, op1, op2 } => write!(f, "{} = srem {} {}, {}", rslt, ty, op1, op2),

            // Bitwise Binary Operations
            Shl { rslt, ty, op1, op2 } => write!(f, "{} = shl {} {}, {}", rslt, ty, op1, op2),
            Lshr { rslt, ty, op1, op2 } => write!(f, "{} = lshr {} {}, {}", rslt, ty, op1, op2),
            Ashr { rslt, ty, op1, op2 } => write!(f, "{} = ashr {} {}, {}", rslt, ty, op1, op2),
            And { rslt, ty, op1, op2 } => write!(f, "{} = and {} {}, {}", rslt, ty, op1, op2),
            Or { rslt, ty, op1, op2 } => write!(f, "{} = or {} {}, {}", rslt, ty, op1, op2),
            Xor { rslt, ty, op1, op2 } => write!(f, "{} = xor {} {}, {}", rslt, ty, op1, op2),

            // Aggregate Operations
            Extractvalue { rslt, ty, val, idx } => write!(
                f,
                "{} = extractvalue {{ {}, i1 }} {}, {}",
                rslt, ty, val, idx
            ),

            // Memory Access and Addressing Operations
            Alloca { rslt, ty } => write!(f, "{} = alloca {}*", rslt, ty),
            Falloca { rslt, fty } => write!(f, "{} = alloca {}*", rslt, fty),
            Load { rslt, ty, ptr } => write!(f, "{} = load {}, {}* {}", rslt, ty, ty, ptr),
            Fload { rslt, fty, ptr } => write!(f, "{} = load {}, {}* {}", rslt, fty, fty, ptr),
            Store { ty, val, ptr } => write!(f, "store {} {}, {}* {}", ty, val, ty, ptr),
            Fstore { fty, val, ptr } => write!(f, "store {} {}, {}* {}", fty, val, fty, ptr),
            Fence(ord) => write!(f, "fence {}", ord),
            Cmpxchg {
                rslt,
                ty,
                ptr,
                cmp,
                new,
                succ_ord,
                fail_ord,
            } => write!(
                f,
                "{} = cmpxchg {}* {}, {} {}, {} {} {} {}",
                rslt, ty, ptr, ty, cmp, ty, new, succ_ord, fail_ord
            ),
            Atomicrmw {
                rslt,
                op,
                ty,
                ptr,
                val,
                ord,
            } => write!(
                f,
                "{} = atomicrmw {} {}* {}, {} {} {}",
                rslt, op, ty, ptr, ty, val, ord
            ),
            Getelementptr {
                rslt,
                len,
                ptr,
                idx,
            } => write!(
                f,
                "{} = getelementptr [{} x i8], [{} x i8]* {}, i8 0, i128 {}",
                rslt, len, len, ptr, idx
            ),

            // Conversion Operations
            Trunc { rslt, ty, val, ty2 } => write!(f, "{} = trunc {} {} to {}", rslt, ty, val, ty2),
            Zext { rslt, ty, val, ty2 } => write!(f, "{} = zext {} {} to {}", rslt, ty, val, ty2),
            Sext { rslt, ty, val, ty2 } => write!(f, "{} = sext {} {} to {}", rslt, ty, val, ty2),
            Fptrunc {
                rslt,
                fty,
                val,
                fty2,
            } => write!(f, "{} = fptrunc {} {} to {}", rslt, fty, val, fty2),
            Fpext {
                rslt,
                fty,
                val,
                fty2,
            } => write!(f, "{} = fpext {} {} to {}", rslt, fty, val, fty2),
            Fptoui { rslt, fty, val, ty } => {
                write!(f, "{} = fptoui {} {} to {}", rslt, fty, val, ty)
            }
            Fptosi { rslt, fty, val, ty } => {
                write!(f, "{} = fptosi {} {} to {}", rslt, fty, val, ty)
            }
            Uitofp { rslt, ty, val, fty } => {
                write!(f, "{} = uitofp {} {} to {}", rslt, ty, val, fty)
            }
            Sitofp { rslt, ty, val, fty } => {
                write!(f, "{} = sitofp {} {} to {}", rslt, ty, val, fty)
            }
            Bitcast { rslt, ty, val, ty2 } => {
                write!(f, "{} = bitcast {}* {} to {}*", rslt, ty, val, ty2)
            }

            // Other Operations
            Icmp {
                rslt,
                cond,
                ty,
                op1,
                op2,
            } => write!(f, "{} = icmp {} {} {}, {}", rslt, cond, ty, op1, op2),
            Fcmp {
                rslt,
                fcond,
                fty,
                op1,
                op2,
            } => write!(f, "{} = fcmp {} {} {}, {}", rslt, fcond, fty, op1, op2),

            // Standard C/C++ Library Intrinsics
            Sqrt { rslt, fty, op1 } => write!(
                f,
                "{} = call {} @llvm.sqrt.f{}({} {})",
                rslt,
                fty,
                fty.flen(),
                fty,
                op1
            ),
            Fma {
                rslt,
                fty,
                op1,
                op2,
                op3,
            } => write!(
                f,
                "{} = call {} @llvm.fma.f{}({} {}, {} {}, {} {})",
                rslt,
                fty,
                fty.flen(),
                fty,
                op1,
                fty,
                op2,
                fty,
                op3
            ),
            Fabs { rslt, fty, op1 } => write!(
                f,
                "{} = call {} @llvm.fabs.f{}({} {})",
                rslt,
                fty,
                fty.flen(),
                fty,
                op1
            ),
            Minimum {
                rslt,
                fty,
                op1,
                op2,
            } => write!(
                f,
                "{} = call {} @llvm.minimum.f{}({} {}, {} {})",
                rslt,
                fty,
                fty.flen(),
                fty,
                op1,
                fty,
                op2
            ),
            Maximum {
                rslt,
                fty,
                op1,
                op2,
            } => write!(
                f,
                "{} = call {} @llvm.maximum.f{}({} {}, {} {})",
                rslt,
                fty,
                fty.flen(),
                fty,
                op1,
                fty,
                op2
            ),
            Copysign {
                rslt,
                fty,
                mag,
                sgn,
            } => write!(
                f,
                "{} = call {} @llvm.copysign.f{}({} {}, {} {})",
                rslt,
                fty,
                fty.flen(),
                fty,
                mag,
                fty,
                sgn
            ),

            // System Calls
            Syscall {
                rslt,
                ty,
                nr,
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6,
            } => write!(
                f,
                "{} = call {} @syscall({} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {})",
                rslt, ty, ty, nr, ty, arg1, ty, arg2, ty, arg3, ty, arg4, ty, arg5, ty, arg6
            ),
        }
    }
}
