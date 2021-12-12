use crate::riscv_isa::{
    Abi, Address, DataBlock, FPRegister, Immediate, Instruction as RiscvInstruction, Register,
};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

const SYSCALL: &str = "; declare dso_local void @exit(i32)
; declare dso_local i32 @printf(i8*, ...)
; @.str = private unnamed_addr constant [13 x i8] c\"#value: %d#\\0A\\00\", align 1
; %val = load i64, i64* %zero
; %code = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i64 0, i64 0), i64 %val)
; call void @exit(i32 0)

declare i{xlen} @syscall(i{xlen}, ...)
";

const FPFUNCTIONS: &str = "declare {ftype} @llvm.sqrt.f{flen}({ftype} %op1)
declare {ftype} @llvm.fma.f{flen}({ftype} %op1, {ftype} %op2, {ftype} %op3)
declare {ftype} @llvm.fabs.f{flen}({ftype} %op1)
declare {ftype} @llvm.minimum.f{flen}({ftype} %op1, {ftype} %op2)
declare {ftype} @llvm.maximum.f{flen}({ftype} %op1, {ftype} %op2)
declare {ftype} @llvm.copysign.f{flen}({ftype} %mag, {ftype} %sgn)
";

const GETDATAPTR: &str = "define i8* @get_data_ptr(i64 %addr) {
  %rel_addr = add i64 %addr, -66912
  %ptr = getelementptr [24 x i8], [24 x i8]* @data_66912, i64 0, i64 %rel_addr
  ret i8* %ptr
}
";

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
  store i{xlen} zeroinitializer, i{xlen}* %sp
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
  store i{xlen} zeroinitializer, i{xlen}* %t6
";

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
  store {ftype} zeroinitializer, {ftype}* %ft11
";

#[derive(Debug, PartialEq)]
pub struct Program {
    pub abi: Abi,
    pub entry: Address,
    pub data_blocks: Vec<DataBlock>,
    pub code_blocks: Vec<CodeBlock>,
    pub stack: HashMap<Address, Vec<Type>>,
    pub fpstack: HashMap<Address, Vec<FPType>>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut program = format!("; ABI: {}\n", self.abi);

        let (xlen, flen, ftype) = match self.abi {
            Abi::Ilp32 => ("32", None, None),
            Abi::Ilp32f => ("32", Some("32"), Some("float")),
            Abi::Ilp32d => ("32", Some("64"), Some("double")),
            Abi::Lp64 => ("64", None, None),
            Abi::Lp64f => ("64", Some("32"), Some("float")),
            Abi::Lp64d => ("64", Some("64"), Some("double")),
        };

        program += &format!("\n{}", &SYSCALL.replace("{xlen}", xlen));
        if let (Some(flen), Some(ftype)) = (flen, ftype) {
            program += &format!(
                "\n{}",
                FPFUNCTIONS
                    .replace("{flen}", flen)
                    .replace("{ftype}", ftype)
            );
        }
        for data_block in self.data_blocks.iter() {
            program += &format!("\n{}", data_block);
        }
        program += &format!("\n{}", GETDATAPTR);

        program += "\ndefine void @main(i32 %argc, i8** %argv) {\n";
        program += "entry:\n";
        program += &REGISTERS.replace("{xlen}", xlen);
        if xlen == "64" {
            program += "\n  %argc_i64 = sext i32 %argc to i64\n";
            program += "  store i64 %argc_i64, i64* %a0\n";
        } else {
            program += "\n  store i32 %argc, i32* %a0\n";
        }
        if let Some(ftype) = ftype {
            program += &format!("\n{}", FPREGISTERS.replace("{ftype}", ftype));
        }
        program += "\n";
        for (addr, tys) in self.stack.iter() {
            // for (i, ty) in tys.iter().enumerate() {
                program += &format!("  %stack_{} = alloca {}\n", addr, tys[0]);
            // }
        }
        for (addr, tys) in self.fpstack.iter() {
            for (i, ty) in tys.iter().enumerate() {
                program += &format!("  %stack_{}_{} = alloca {}\n", addr, i, ty);
            }
        }
        program += &format!("\n  br label %label_{}\n", self.entry);
        program += "label_1:\n  unreachable\n";
        for code_block in self.code_blocks.iter() {
            program += &format!("\n{}", code_block);
        }
        program += "}\n";

        write!(f, "{}", program)
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
        iftrue: Value,
        iffalse: Value,
    },
    UnconBr {
        addr: Value,
    },
    Switch {
        ty: Type,
        val: Value,
        default: Value,
        targets: Vec<Value>,
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

    // Misc
    Loadstack {
        rslt: Value,
        ty: Type,
        stk: Value,
        ver: Value,
    },
    Floadstack {
        rslt: Value,
        fty: FPType,
        stk: Value,
        ver: Value,
    },
    Storestack {
        ty: Type,
        val: Value,
        stk: Value,
        ver: Value,
    },
    Fstorestack {
        fty: FPType,
        val: Value,
        stk: Value,
        ver: Value,
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
                default,
                targets,
            } => {
                let mut s = format!("switch {} {}, label %label_{} [", ty, val, default);
                for target in targets {
                    s += &format!("{} {}, label %label_{} ", ty, target, target);
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
            Fptrunc {
                rslt,
                fty,
                val,
                fty2,
            } => match fty == fty2 {
                true => write!(f, "{} = fadd {} {}, {}", rslt, fty, val, 0),
                false => write!(f, "{} = fptrunc {} {} to {}", rslt, fty, val, fty2),
            },
            Fpext {
                rslt,
                fty,
                val,
                fty2,
            } => match fty == fty2 {
                true => write!(f, "{} = fadd {} {}, {}", rslt, fty, val, 0),
                false => write!(f, "{} = fpext {} {} to {}", rslt, fty, val, fty2),
            },
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

            // Misc
            Loadstack { rslt, ty, stk, ver } => {
                write!(f, "{} = load {}, {}* %stack_{}", rslt, ty, ty, stk)
            }
            Floadstack {
                rslt,
                fty,
                stk,
                ver,
            } => write!(
                f,
                "{} = load {}, {}* %fpstack_{}_{}",
                rslt, fty, fty, stk, ver
            ),
            Storestack { ty, val, stk, ver } => {
                write!(f, "store {} {}, {}* %stack_{}", ty, val, ty, stk)
            }
            Fstorestack { fty, val, stk, ver } => {
                write!(f, "store {} {}, {}* %stack_{}_{}", fty, val, fty, stk, ver)
            }

            Getdataptr { rslt, ty, addr } => {
                write!(f, "{} = call i8* @get_data_ptr({} {})", rslt, ty, addr)
            }
        }
    }
}
