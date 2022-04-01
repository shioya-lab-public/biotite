#![allow(dead_code)]

use crate::riscv_isa::{
    Abi, Address, DataBlock, FPRegister, Immediate, Instruction as RiscvInstruction, Register,
};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result as FmtResult};

const SYSCALL: &str = "declare i{xlen} @syscall(i{xlen}, ...)

%struct.tms = type { i64, i64, i64, i64 }

%struct.stat = type { i64, i64, i64, i32, i32, i32, i32, i64, i64, i64, i64, %struct.timespec, %struct.timespec, %struct.timespec, [3 x i64] }
%struct.timespec = type { i64, i64 }

define i64 @sys_call(i64 %nr, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) {
  switch i64 %nr, label %fallback [
    i64 93, label %SYS_exit
    i64 169, label %SYS_gettimeofday
    i64 214, label %SYS_brk
    i64 57, label %SYS_close
    i64 80, label %SYS_fstat
    i64 62, label %SYS_lseek
    i64 63, label %SYS_read
    i64 64, label %SYS_write
  ]

SYS_exit:
  %SYS_exit_rslt = call i64 (i64, ...) @syscall(i64 60, i64 %arg1)
  ret i64 %SYS_exit_rslt

SYS_gettimeofday:
  %tms_ptr = call i8* @get_data_ptr(i64 %arg1)
  %tms = bitcast i8* %tms_ptr to %struct.tms*
  %SYS_gettimeofday_rslt = call i64 (i64, ...) @syscall(i64 96, %struct.tms* %tms, i64 %arg2)
  ret i64 %SYS_gettimeofday_rslt

SYS_brk:
  %SYS_brk_rslt = call i64 (i64, ...) @syscall(i64 12, i64 %arg1)
  ret i64 %SYS_brk_rslt

SYS_close:
  %SYS_close_rslt = call i64 (i64, ...) @syscall(i64 3, i64 %arg1)
  ret i64 %SYS_close_rslt

SYS_fstat:
  %stat_ptr = call i8* @get_data_ptr(i64 %arg2)
  %stat = bitcast i8* %stat_ptr to %struct.stat*
  %SYS_fstat_rslt = call i64 (i64, ...) @syscall(i64 5, i64 %arg1, %struct.stat* %stat)
  ret i64 %SYS_fstat_rslt

SYS_lseek:
  %SYS_lseek_rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)
  ret i64 %SYS_lseek_rslt

SYS_read:
  %read_buf = call i8* @get_data_ptr(i64 %arg2)
  %SYS_read_rslt = call i64 (i64, ...) @syscall(i64 0, i64 %arg1, i8* %read_buf, i64 %arg3)
  ret i64 %SYS_read_rslt

SYS_write:
  %write_buf = call i8* @get_data_ptr(i64 %arg2)
  %SYS_write_rslt = call i64 (i64, ...) @syscall(i64 1, i64 %arg1, i8* %write_buf, i64 %arg3)
  ret i64 %SYS_write_rslt

fallback:
  unreachable
}";

const FPFUNCTIONS: &str = "declare {ftype} @llvm.sqrt.f{flen}({ftype} %op1)
declare {ftype} @llvm.fma.f{flen}({ftype} %op1, {ftype} %op2, {ftype} %op3)
declare {ftype} @llvm.fabs.f{flen}({ftype} %op1)
declare {ftype} @llvm.minimum.f{flen}({ftype} %op1, {ftype} %op2)
declare {ftype} @llvm.maximum.f{flen}({ftype} %op1, {ftype} %op2)
declare {ftype} @llvm.copysign.f{flen}({ftype} %mag, {ftype} %sgn)";

const REGISTERS: &str = "  %zero = alloca i{xlen}
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

  store i{xlen} zeroinitializer, i{xlen}* %zero
  store i{xlen} zeroinitializer, i{xlen}* %ra
  store i{xlen} 10240, i{xlen}* %sp
  store i{xlen} zeroinitializer, i{xlen}* %gp
  store i{xlen} zeroinitializer, i{xlen}* %tp
  store i{xlen} zeroinitializer, i{xlen}* %t0
  store i{xlen} zeroinitializer, i{xlen}* %t1
  store i{xlen} zeroinitializer, i{xlen}* %t2
  store i{xlen} zeroinitializer, i{xlen}* %s0
  store i{xlen} zeroinitializer, i{xlen}* %s1
  store i{xlen} zeroinitializer, i{xlen}* %a0
  store i{xlen} zeroinitializer, i{xlen}* %a1
  store i{xlen} zeroinitializer, i{xlen}* %a2
  store i{xlen} zeroinitializer, i{xlen}* %a3
  store i{xlen} zeroinitializer, i{xlen}* %a4
  store i{xlen} zeroinitializer, i{xlen}* %a5
  store i{xlen} zeroinitializer, i{xlen}* %a6
  store i{xlen} zeroinitializer, i{xlen}* %a7
  store i{xlen} zeroinitializer, i{xlen}* %s2
  store i{xlen} zeroinitializer, i{xlen}* %s3
  store i{xlen} zeroinitializer, i{xlen}* %s4
  store i{xlen} zeroinitializer, i{xlen}* %s5
  store i{xlen} zeroinitializer, i{xlen}* %s6
  store i{xlen} zeroinitializer, i{xlen}* %s7
  store i{xlen} zeroinitializer, i{xlen}* %s8
  store i{xlen} zeroinitializer, i{xlen}* %s9
  store i{xlen} zeroinitializer, i{xlen}* %s10
  store i{xlen} zeroinitializer, i{xlen}* %s11
  store i{xlen} zeroinitializer, i{xlen}* %t3
  store i{xlen} zeroinitializer, i{xlen}* %t4
  store i{xlen} zeroinitializer, i{xlen}* %t5
  store i{xlen} zeroinitializer, i{xlen}* %t6";

const FPREGISTERS: &str = "  %ft0 = alloca {ftype}
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

  store {ftype} zeroinitializer, {ftype}* %ft0
  store {ftype} zeroinitializer, {ftype}* %ft1
  store {ftype} zeroinitializer, {ftype}* %ft2
  store {ftype} zeroinitializer, {ftype}* %ft3
  store {ftype} zeroinitializer, {ftype}* %ft4
  store {ftype} zeroinitializer, {ftype}* %ft5
  store {ftype} zeroinitializer, {ftype}* %ft6
  store {ftype} zeroinitializer, {ftype}* %ft7
  store {ftype} zeroinitializer, {ftype}* %fs0
  store {ftype} zeroinitializer, {ftype}* %fs1
  store {ftype} zeroinitializer, {ftype}* %fa0
  store {ftype} zeroinitializer, {ftype}* %fa1
  store {ftype} zeroinitializer, {ftype}* %fa2
  store {ftype} zeroinitializer, {ftype}* %fa3
  store {ftype} zeroinitializer, {ftype}* %fa4
  store {ftype} zeroinitializer, {ftype}* %fa5
  store {ftype} zeroinitializer, {ftype}* %fa6
  store {ftype} zeroinitializer, {ftype}* %fa7
  store {ftype} zeroinitializer, {ftype}* %fs2
  store {ftype} zeroinitializer, {ftype}* %fs3
  store {ftype} zeroinitializer, {ftype}* %fs4
  store {ftype} zeroinitializer, {ftype}* %fs5
  store {ftype} zeroinitializer, {ftype}* %fs6
  store {ftype} zeroinitializer, {ftype}* %fs7
  store {ftype} zeroinitializer, {ftype}* %fs8
  store {ftype} zeroinitializer, {ftype}* %fs9
  store {ftype} zeroinitializer, {ftype}* %fs10
  store {ftype} zeroinitializer, {ftype}* %fs11
  store {ftype} zeroinitializer, {ftype}* %ft8
  store {ftype} zeroinitializer, {ftype}* %ft9
  store {ftype} zeroinitializer, {ftype}* %ft10
  store {ftype} zeroinitializer, {ftype}* %ft11";

#[derive(Debug, PartialEq)]
pub struct Program {
    pub abi: Abi,
    pub entry: Address,
    pub data_blocks: Vec<DataBlock>,
    pub code_blocks: Vec<CodeBlock>,
    pub stack: HashMap<Address, HashSet<Type>>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (xlen, flen, ftype) = match self.abi {
            Abi::Ilp32 => ("32", None, None),
            Abi::Ilp32f => ("32", Some("32"), Some("float")),
            Abi::Ilp32d => ("32", Some("64"), Some("double")),
            Abi::Lp64 => ("64", None, None),
            Abi::Lp64f => ("64", Some("32"), Some("float")),
            Abi::Lp64d => ("64", Some("64"), Some("double")),
        };

        let abi = format!("; ABI: {}", self.abi);
        let syscall = SYSCALL.replace("{xlen}", xlen);
        let fpfunctions = if let (Some(flen), Some(ftype)) = (flen, ftype) {
            FPFUNCTIONS
                .replace("{flen}", flen)
                .replace("{ftype}", ftype)
        } else {
            String::new()
        };
        let data_blocks = self
            .data_blocks
            .iter()
            .fold(String::new(), |s, b| s + &format!("{}\n", b));

        let get_data_ptr = if self.data_blocks.is_empty() {
            String::new()
        } else {
        let mut get_data_ptr = format!("define i8* @get_data_ptr(i{} %addr) {{\n", xlen);
        let mut data_blocks_iter = self.data_blocks.iter();
        let mut current = data_blocks_iter.next();
        let mut next = data_blocks_iter.next();
        while let Some(cur) = current {
            if let Some(nxt) = next {
                let Address(addr) = cur.address;
                let cur_end = addr as usize + cur.bytes.len();
                get_data_ptr += &format!(
                    "data_{cur}:
  %data_{cur}_start = icmp sle i{xlen} {cur}, %addr
  br i1 %data_{cur}_start, label %data_{cur}_start_true, label %data_{nxt}
data_{cur}_start_true:
  %data_{cur}_end = icmp sgt i{xlen} {cur_end}, %addr
  br i1 %data_{cur}_end, label %data_{cur}_true, label %data_{nxt}
data_{cur}_true:
  %rel_addr_{cur} = sub i{xlen} %addr, {cur}
  %ptr_{cur} = getelementptr [{len} x i8], [{len} x i8]* @data_{cur}, i64 0, i{xlen} %rel_addr_{cur}
  ret i8* %ptr_{cur}
",
                    cur = cur.address,
                    xlen = xlen,
                    nxt = nxt.address,
                    len = cur.bytes.len()
                );
            } else {
                let Address(addr) = cur.address;
                let cur_end = addr as usize + cur.bytes.len();
                get_data_ptr += &format!(
                    "data_{cur}:
  %data_{cur}_start = icmp sle i{xlen} {cur}, %addr
  br i1 %data_{cur}_start, label %data_{cur}_start_true, label %fallback
data_{cur}_start_true:
  %data_{cur}_end = icmp sgt i{xlen} {cur_end}, %addr
  br i1 %data_{cur}_end, label %data_{cur}_true, label %fallback
data_{cur}_true:
  %rel_addr_{cur} = sub i{xlen} %addr, {cur}
  %ptr_{cur} = getelementptr [{len} x i8], [{len} x i8]* @data_{cur}, i64 0, i{xlen} %rel_addr_{cur}
  ret i8* %ptr_{cur}
fallback:
  %ptr = inttoptr i{xlen} %addr to i8*
  ret i8* %ptr
",
                    cur = cur.address,
                    xlen = xlen,
                    len = cur.bytes.len()
                );
            }
            current = next;
            next = data_blocks_iter.next();
        }
        get_data_ptr += "}";
        get_data_ptr
        };

        let mut registers = REGISTERS.replace("{xlen}", xlen);
        if xlen == "64" {
            registers += "\n\n  %argc_i64 = sext i32 %argc to i64\n";
            registers += "  store i64 %argc_i64, i64* %a0";
        } else {
            registers += "\n\n  store i32 %argc, i32* %a0";
        };
        let fpregisters = if let Some(ftype) = ftype {
            FPREGISTERS.replace("{ftype}", ftype)
        } else {
            String::new()
        };
        let mut stack = String::new();
        for (addr, tys) in self.stack.iter() {
            for ty in tys {
                stack += &format!("  %stack_{}_{} = alloca {}\n", addr, ty, ty);
            }
        }
        let entry = format!("  br label %label_{}", self.entry);
        let code_blocks = self
            .code_blocks
            .iter()
            .fold(String::new(), |s, b| s + &format!("{}\n", b));

        write!(
            f,
            "{}

{}

{}

{}

{}

declare dso_local void @exit(i32)
declare dso_local i32 @printf(i8*, ...)
@.str.d = private unnamed_addr constant [14 x i8] c\"#value: %ld#\\0A\\00\", align 1
@.str.f = private unnamed_addr constant [13 x i8] c\"#value: %f#\\0A\\00\", align 1
@.str.s = private unnamed_addr constant [13 x i8] c\"#value: %s#\\0A\\00\", align 1

define i{xlen} @main(i32 %argc, i8** %argv) {{
entry:
  %switch_target = alloca i64

{}

{}

{}
{}



label_1:
  %switch_target_value = load i64, i64* %switch_target
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str.d, i64 0, i64 0), i64 %switch_target_value)
  call void @exit(i32 2)
  unreachable

{}

label_0:
  %ret = load i{xlen}, i{xlen}* %a0
  ret i{xlen} %ret
}}
",
            abi,
            syscall,
            fpfunctions,
            get_data_ptr,
            data_blocks,
            registers,
            fpregisters,
            stack,
            entry,
            code_blocks
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct CodeBlock {
    pub section: String,
    pub symbol: String,
    pub address: Address,
    pub instruction_blocks: Vec<InstructionBlock>,
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut code_block = format!("; {}: {} <{}>\n", self.address, self.section, self.symbol);
        code_block += &format!("label_{}:\n", self.address);
        for inst_block in self.instruction_blocks.iter() {
            code_block += &format!("{}", inst_block);
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

        // debug
        // inst_block += &format!("  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i64 0, i64 0), i64 {})\n", self.riscv_instruction.address());

        for inst in self.instructions.iter() {
            inst_block += &format!("  {}\n", inst);
        }
        write!(f, "{}", inst_block)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Register(Register),
    FPRegister(FPRegister),
    Temp(Address, usize),
    Immediate(Immediate),
    Address(Address),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Value::*;

        match self {
            Register(reg) => write!(f, "%{}", reg),
            FPRegister(freg) => write!(f, "%{}", freg),
            Temp(addr, nr) => write!(f, "%t_{}_{}", addr, nr),
            Immediate(imm) => write!(f, "{}", imm),
            Address(addr) => write!(f, "{}", addr),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    I1,
    I8,
    I16,
    I32,
    I64,
    I128,
    Float,
    Double,
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
            Float => write!(f, "float"),
            Double => write!(f, "double"),
        }
    }
}

impl Type {
    fn size(&self) -> usize {
        use Type::*;

        match self {
            Float => 32,
            Double => 64,
            _ => unreachable!(),
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
        iftrue: Value,
        iffalse: Value,
    },
    UnconBr {
        addr: Value,
    },
    Switch {
        ty: Type,
        val: Value,
        dflt: Value,
        tgts: Vec<Value>,
    },

    // Unary Operations
    Fneg {
        rslt: Value,
        ty: Type,
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
        ty: Type,
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
        ty: Type,
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
        ty: Type,
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
        ty: Type,
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
    Load {
        rslt: Value,
        ty: Type,
        ptr: Value,
    },
    Store {
        ty: Type,
        val: Value,
        ptr: Value,
    },
    Fence {
        ord: Ordering,
    },
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
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Fpext {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Fptoui {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Fptosi {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Uitofp {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
    },
    Sitofp {
        rslt: Value,
        ty: Type,
        val: Value,
        ty2: Type,
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
        ty: Type,
        op1: Value,
        op2: Value,
    },

    // Standard C/C++ Library Intrinsics
    Sqrt {
        rslt: Value,
        ty: Type,
        op1: Value,
    },
    Fma {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
        op3: Value,
    },
    Fabs {
        rslt: Value,
        ty: Type,
        op1: Value,
    },
    Minimum {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Maximum {
        rslt: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },
    Copysign {
        rslt: Value,
        ty: Type,
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

    // Misc
    Loadstack {
        rslt: Value,
        ty: Type,
        stk: Value,
    },
    Storestack {
        ty: Type,
        val: Value,
        stk: Value,
    },

    Getdataptr {
        rslt: Value,
        ty: Type,
        addr: Value,
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
            UnconBr { addr } => write!(f, "br label %label_{}", addr),
            Switch {
                ty,
                val,
                dflt,
                tgts,
            } => {
                println!("{}", tgts.len());
                let mut s = format!("store i64 {}, i64* %switch_target\nswitch {} {}, label %label_{} [", val, ty, val, dflt);
                for target in tgts {
                    s += &format!("{} {}, label %label_{} ", ty, target, target);
                }
                s += "]";
                write!(f, "{}", s)
            }

            // Unary Operations
            Fneg { rslt, ty, op1 } => write!(f, "{} = fneg {} {}", rslt, ty, op1),

            // Binary Operations
            Add { rslt, ty, op1, op2 } => write!(f, "{} = add {} {}, {}", rslt, ty, op1, op2),
            Fadd { rslt, ty, op1, op2 } => write!(f, "{} = fadd {} {}, {}", rslt, ty, op1, op2),
            Sub { rslt, ty, op1, op2 } => write!(f, "{} = sub {} {}, {}", rslt, ty, op1, op2),
            Fsub { rslt, ty, op1, op2 } => write!(f, "{} = fsub {} {}, {}", rslt, ty, op1, op2),
            Mul { rslt, ty, op1, op2 } => write!(f, "{} = mul {} {}, {}", rslt, ty, op1, op2),
            Fmul { rslt, ty, op1, op2 } => write!(f, "{} = fmul {} {}, {}", rslt, ty, op1, op2),
            Udiv { rslt, ty, op1, op2 } => write!(f, "{} = udiv {} {}, {}", rslt, ty, op1, op2),
            Sdiv { rslt, ty, op1, op2 } => write!(f, "{} = sdiv {} {}, {}", rslt, ty, op1, op2),
            Fdiv { rslt, ty, op1, op2 } => write!(f, "{} = fdiv {} {}, {}", rslt, ty, op1, op2),
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
            Load { rslt, ty, ptr } => write!(f, "{} = load {}, {}* {}", rslt, ty, ty, ptr),
            Store { ty, val, ptr } => write!(f, "store {} {}, {}* {}", ty, val, ty, ptr),
            Fence { ord } => write!(f, "fence {}", ord),
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

            // Conversion Operations
            Trunc { rslt, ty, val, ty2 } => match ty == ty2 {
                true => write!(f, "{} = add {} {}, {}", rslt, ty, val, 0),
                false => write!(f, "{} = trunc {} {} to {}", rslt, ty, val, ty2),
            },
            Zext { rslt, ty, val, ty2 } => match ty == ty2 {
                true => write!(f, "{} = add {} {}, {}", rslt, ty, val, 0),
                false => write!(f, "{} = zext {} {} to {}", rslt, ty, val, ty2),
            },
            Sext { rslt, ty, val, ty2 } => match ty == ty2 {
                true => write!(f, "{} = add {} {}, {}", rslt, ty, val, 0),
                false => write!(f, "{} = sext {} {} to {}", rslt, ty, val, ty2),
            },
            Fptrunc { rslt, ty, val, ty2 } => match ty == ty2 {
                true => write!(f, "{} = fadd {} {}, {}", rslt, ty, val, 0),
                false => write!(f, "{} = fptrunc {} {} to {}", rslt, ty, val, ty2),
            },
            Fpext { rslt, ty, val, ty2 } => match ty == ty2 {
                true => write!(f, "{} = fadd {} {}, {}", rslt, ty, val, 0),
                false => write!(f, "{} = fpext {} {} to {}", rslt, ty, val, ty2),
            },
            Fptoui { rslt, ty, val, ty2 } => {
                write!(f, "{} = fptoui {} {} to {}", rslt, ty, val, ty2)
            }
            Fptosi { rslt, ty, val, ty2 } => {
                write!(f, "{} = fptosi {} {} to {}", rslt, ty, val, ty2)
            }
            Uitofp { rslt, ty, val, ty2 } => {
                write!(f, "{} = uitofp {} {} to {}", rslt, ty, val, ty2)
            }
            Sitofp { rslt, ty, val, ty2 } => {
                write!(f, "{} = sitofp {} {} to {}", rslt, ty, val, ty2)
            }
            Bitcast { rslt, ty, val, ty2 } => match (ty, ty2) {
                (Type::Double, _) | (_, Type::Double) | (Type::Float, _) | (_, Type::Float) => write!(f, "{} = bitcast {} {} to {}", rslt, ty, val, ty2),
                _ => write!(f, "{} = bitcast {}* {} to {}*", rslt, ty, val, ty2),
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
                ty,
                op1,
                op2,
            } => write!(f, "{} = fcmp {} {} {}, {}", rslt, fcond, ty, op1, op2),

            // Standard C/C++ Library Intrinsics
            Sqrt { rslt, ty, op1 } => write!(
                f,
                "{} = call {} @llvm.sqrt.f{}({} {})",
                rslt,
                ty,
                ty.size(),
                ty,
                op1
            ),
            Fma {
                rslt,
                ty,
                op1,
                op2,
                op3,
            } => write!(
                f,
                "{} = call {} @llvm.fma.f{}({} {}, {} {}, {} {})",
                rslt,
                ty,
                ty.size(),
                ty,
                op1,
                ty,
                op2,
                ty,
                op3
            ),
            Fabs { rslt, ty, op1 } => write!(
                f,
                "{} = call {} @llvm.fabs.f{}({} {})",
                rslt,
                ty,
                ty.size(),
                ty,
                op1
            ),
            Minimum { rslt, ty, op1, op2 } => write!(
                f,
                "{} = call {} @llvm.minimum.f{}({} {}, {} {})",
                rslt,
                ty,
                ty.size(),
                ty,
                op1,
                ty,
                op2
            ),
            Maximum { rslt, ty, op1, op2 } => write!(
                f,
                "{} = call {} @llvm.maximum.f{}({} {}, {} {})",
                rslt,
                ty,
                ty.size(),
                ty,
                op1,
                ty,
                op2
            ),
            Copysign { rslt, ty, mag, sgn } => write!(
                f,
                "{} = call {} @llvm.copysign.f{}({} {}, {} {})",
                rslt,
                ty,
                ty.size(),
                ty,
                mag,
                ty,
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
                "{} = call {} @sys_call({} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {})",
                rslt, ty, ty, nr, ty, arg1, ty, arg2, ty, arg3, ty, arg4, ty, arg5, ty, arg6
            ),

            // Misc
            Loadstack { rslt, ty, stk } => {
                write!(f, "{} = load {}, {}* %stack_{}_{}", rslt, ty, ty, stk, ty)
            }
            Storestack { ty, val, stk } => {
                write!(f, "store {} {}, {}* %stack_{}_{}", ty, val, ty, stk, ty)
            }

            Getdataptr { rslt, ty, addr } => {
                write!(f, "{} = call i8* @get_data_ptr({} {})", rslt, ty, addr)
            }
        }
    }
}
