#![allow(dead_code)]

use crate::riscv_isa::{
    Address, DataBlock, FPRegister, Immediate, Instruction as RiscvInstruction, Register,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

const SYSCALL: &str = "declare i64 @syscall(i64, ...)

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

const FPFUNCTIONS: &str = "declare double @llvm.sqrt.f64(double %op1)
declare double @llvm.fma.f64(double %op1, double %op2, double %op3)
declare double @llvm.fabs.f64(double %op1)
declare double @llvm.minimum.f64(double %op1, double %op2)
declare double @llvm.maximum.f64(double %op1, double %op2)
declare double @llvm.copysign.f64(double %mag, double %sgn)";

const REGISTERS: &str = "

  %reg = alloca %struct.reg
  %freg = alloca %struct.freg
  %reg_byte = bitcast %struct.reg* %reg to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %reg_byte, i8 0, i64 256, i1 false)
  %freg_byte = bitcast %struct.freg* %freg to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %freg_byte, i8 0, i64 256, i1 false)

  %sp = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 2
  store i64 10240, i64* %sp

  %argc_i64 = sext i32 %argc to i64
  %a0 = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 10
  store i64 %argc_i64, i64* %a0
        
";

#[derive(Debug, PartialEq)]
pub struct Program {
    pub entry: Address,
    pub data_blocks: Vec<DataBlock>,
    pub functions: Vec<Vec<CodeBlock>>,
    pub targets: Vec<Address>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let data_blocks = self
            .data_blocks
            .iter()
            .fold(String::new(), |s, b| s + &format!("{}\n", b));

        let get_data_ptr = if self.data_blocks.is_empty() {
            String::new()
        } else {
            let mut get_data_ptr = format!("define i8* @get_data_ptr(i64 %addr) {{\n");
            let mut data_blocks_iter = self.data_blocks.iter();
            let mut current = data_blocks_iter.next();
            let mut next = data_blocks_iter.next();
            while let Some(cur) = current {
                if let Some(nxt) = next {
                    let Address(addr) = cur.address;
                    let cur_end = addr as usize + cur.bytes.len();
                    get_data_ptr += &format!(
                        "data_{cur}:
  %data_{cur}_start = icmp sle i64 {cur}, %addr
  br i1 %data_{cur}_start, label %data_{cur}_start_true, label %data_{nxt}
data_{cur}_start_true:
  %data_{cur}_end = icmp sgt i64 {cur_end}, %addr
  br i1 %data_{cur}_end, label %data_{cur}_true, label %data_{nxt}
data_{cur}_true:
  %rel_addr_{cur} = sub i64 %addr, {cur}
  %ptr_{cur} = getelementptr [{len} x i8], [{len} x i8]* @data_{cur}, i64 0, i64 %rel_addr_{cur}
  ret i8* %ptr_{cur}
",
                        cur = cur.address,
                        nxt = nxt.address,
                        len = cur.bytes.len()
                    );
                } else {
                    let Address(addr) = cur.address;
                    let cur_end = addr as usize + cur.bytes.len();
                    get_data_ptr += &format!(
                        "data_{cur}:
  %data_{cur}_start = icmp sle i64 {cur}, %addr
  br i1 %data_{cur}_start, label %data_{cur}_start_true, label %fallback
data_{cur}_start_true:
  %data_{cur}_end = icmp sgt i64 {cur_end}, %addr
  br i1 %data_{cur}_end, label %data_{cur}_true, label %fallback
data_{cur}_true:
  %rel_addr_{cur} = sub i64 %addr, {cur}
  %ptr_{cur} = getelementptr [{len} x i8], [{len} x i8]* @data_{cur}, i64 0, i64 %rel_addr_{cur}
  ret i8* %ptr_{cur}
fallback:
  %ptr = inttoptr i64 %addr to i8*
  ret i8* %ptr
",
                        cur = cur.address,
                        len = cur.bytes.len()
                    );
                }
                current = next;
                next = data_blocks_iter.next();
            }
            get_data_ptr += "}";
            get_data_ptr
        };

        // Build each function
        let mut functions = String::new();
        for func in &self.functions {
            let mut targets = String::from("switch i64 %entry, label %label_0 [");
            for block in func {
                for inst in &block.instruction_blocks {
                    let Address(addr) = inst.riscv_instruction.address();
                    targets += &format!("i64 {}, label %label_{} ", addr, addr);
                }
            }
            targets.pop();
            targets += "]";

            let code_blocks = func
                .iter()
                .fold(String::new(), |s, b| s + &format!("{}\n", b));

            let s = format!(
                "
define i64 @func_{}(i64 %entry, %struct.reg* %reg, %struct.freg* %freg) {{
  {targets}
  {code_blocks}
label_0:
  unreachable
}}
",
                func[0].address
            );
            functions += &s;
        }

        // Main dispatcher
        let mut inst_cnt = 150000;
        let mut table = vec![String::from("i64 0"); inst_cnt];
        for func in &self.functions {
            let f = func[0].instruction_blocks[0].riscv_instruction.address();
            for block in func {
                for b in &block.instruction_blocks {
                    let Address(addr) = b.riscv_instruction.address();
                    table[addr as usize] = format!(
                        "i64 ptrtoint (i64 (i64, %struct.reg*, %struct.freg*)* @func_{f} to i64)"
                    );
                }
            }
        }
        let table = table.join(", ");
        let mut table = format!("@dispatch_table = dso_local global [{inst_cnt} x i64] [{table}]");

        // Main formatting
        write!(
            f,
            "{}

{}

{}

{}

%struct.reg = type {{ i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64 }}
%struct.freg = type {{ double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, double }}
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg)
declare dso_local void @exit(i32)
declare dso_local i32 @printf(i8*, ...)
@.str.d = private unnamed_addr constant [14 x i8] c\"#value: %ld#\\0A\\00\", align 1
@.str.f = private unnamed_addr constant [13 x i8] c\"#value: %f#\\0A\\00\", align 1
@.str.s = private unnamed_addr constant [13 x i8] c\"#value: %s#\\0A\\00\", align 1

{table}

define i64 @main(i32 %argc, i8** %argv) {{
{}

%entry_p= alloca i64
store i64 {}, i64* %entry_p
br label %loop
loop:
%entry = load i64, i64* %entry_p
%func_val_ptr = getelementptr [{inst_cnt} x i64], [{inst_cnt} x i64]* @dispatch_table, i64 0, i64 %entry
%func_val = load i64, i64* %func_val_ptr
%func_ptr = inttoptr i64 %func_val to i64 (i64, %struct.reg*, %struct.freg*)*
%target = call i64 %func_ptr(i64 %entry, %struct.reg* %reg, %struct.freg* %freg)
store i64 %target, i64* %entry_p
br label %loop


label_0:
  unreachable
}}

{}

",
SYSCALL,
FPFUNCTIONS,
            get_data_ptr,
            data_blocks,
            REGISTERS,
            self.entry,
            functions
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
        code_block += &format!("label_{}:
  ; call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str.d, i64 0, i64 0), i64 {})\n", self.address, self.address);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Ordering {
    Monotonic,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
}

impl Display for Ordering {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Ordering::*;

        match self {
            Monotonic => write!(f, "monotonic"),
            Acquire => write!(f, "acquire"),
            Release => write!(f, "release"),
            AcqRel => write!(f, "acq_rel"),
            SeqCst => write!(f, "seq_cst"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

    Call {
        addr: Value,
    },
    SwitchCall {
        ty: Type,
        val: Value,
        dflt: Value,
        tgts: Vec<Value>,
        next_pc: Value,
    },
    Ret {
        ty: Type,
        val: Value,
    },
    Unreachable {
        addr: Value,
    },
}

static mut T: usize = 1000;

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Instruction::*;

        match self {
            // Terminator Instructions
            ConBr {
                cond,
                iftrue,
                iffalse,
            } =>
            write!(
                f,
                "br i1 {}, label %label_{}, label %label_{}",
                cond, iftrue, iffalse
            ),
            // {
            //     let c = &format!("{cond}")[1..];
            //     write!(
            //         f,
            //         "br i1 {cond}, label {cond}_t, label {cond}_f
            //         {c}_t:
            //           ret i64 {iftrue}
            //         {c}_f:
            //           ret i64 {iffalse}"
            //     )
            // }
            UnconBr { addr } => write!(f, "br label %label_{}", addr),
            Switch {
                ty,
                val,
                dflt,
                tgts,
            } => {
                // let mut s = format!("switch {} {}, label %label_{} [", ty, val, dflt);
                // for target in tgts {
                //     s += &format!("{} {}, label %label_{} ", ty, target, target);
                // }
                // s += "]";
                let s = format!(
                    "store i64 {}, i64* %switch_target\n  br label %label_1",
                    val
                );
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
            Load { rslt, ty, ptr } => {
                // let temp = if let Value::Temp(addr, t) = rslt {
                //     Value::Temp(*addr, t+1000)
                // } else if let Value::Immediate(_) = rslt {
                let t = unsafe { T };
                unsafe {
                    T += 1;
                }
                let temp = Value::Temp(Address(0), t);
                // } else {
                //     unreachable!()
                // };
                let load = match ptr {
                    Value::Register(Register::Zero) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 0"
                    ),
                    Value::Register(Register::Ra) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 1"
                    ),
                    Value::Register(Register::Sp) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 2"
                    ),
                    Value::Register(Register::Gp) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 3"
                    ),
                    Value::Register(Register::Tp) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 4"
                    ),
                    Value::Register(Register::T0) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 5"
                    ),
                    Value::Register(Register::T1) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 6"
                    ),
                    Value::Register(Register::T2) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 7"
                    ),
                    Value::Register(Register::S0) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 8"
                    ),
                    Value::Register(Register::S1) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 9"
                    ),
                    Value::Register(Register::A0) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 10"
                    ),
                    Value::Register(Register::A1) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 11"
                    ),
                    Value::Register(Register::A2) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 12"
                    ),
                    Value::Register(Register::A3) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 13"
                    ),
                    Value::Register(Register::A4) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 14"
                    ),
                    Value::Register(Register::A5) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 15"
                    ),
                    Value::Register(Register::A6) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 16"
                    ),
                    Value::Register(Register::A7) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 17"
                    ),
                    Value::Register(Register::S2) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 18"
                    ),
                    Value::Register(Register::S3) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 19"
                    ),
                    Value::Register(Register::S4) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 20"
                    ),
                    Value::Register(Register::S5) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 21"
                    ),
                    Value::Register(Register::S6) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 22"
                    ),
                    Value::Register(Register::S7) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 23"
                    ),
                    Value::Register(Register::S8) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 24"
                    ),
                    Value::Register(Register::S9) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 25"
                    ),
                    Value::Register(Register::S10) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 26"
                    ),
                    Value::Register(Register::S11) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 27"
                    ),
                    Value::Register(Register::T3) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 28"
                    ),
                    Value::Register(Register::T4) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 29"
                    ),
                    Value::Register(Register::T5) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 30"
                    ),
                    Value::Register(Register::T6) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 31"
                    ),

                    Value::FPRegister(FPRegister::Ft0) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 0"
                    ),
                    Value::FPRegister(FPRegister::Ft1) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 1"
                    ),
                    Value::FPRegister(FPRegister::Ft2) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 2"
                    ),
                    Value::FPRegister(FPRegister::Ft3) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 3"
                    ),
                    Value::FPRegister(FPRegister::Ft4) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 4"
                    ),
                    Value::FPRegister(FPRegister::Ft5) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 5"
                    ),
                    Value::FPRegister(FPRegister::Ft6) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 6"
                    ),
                    Value::FPRegister(FPRegister::Ft7) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 7"
                    ),
                    Value::FPRegister(FPRegister::Fs0) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 8"
                    ),
                    Value::FPRegister(FPRegister::Fs1) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 9"
                    ),
                    Value::FPRegister(FPRegister::Fa0) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 10"
                    ),
                    Value::FPRegister(FPRegister::Fa1) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 11"
                    ),
                    Value::FPRegister(FPRegister::Fa2) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 12"
                    ),
                    Value::FPRegister(FPRegister::Fa3) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 13"
                    ),
                    Value::FPRegister(FPRegister::Fa4) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 14"
                    ),
                    Value::FPRegister(FPRegister::Fa5) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 15"
                    ),
                    Value::FPRegister(FPRegister::Fa6) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 16"
                    ),
                    Value::FPRegister(FPRegister::Fa7) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 17"
                    ),
                    Value::FPRegister(FPRegister::Fs2) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 18"
                    ),
                    Value::FPRegister(FPRegister::Fs3) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 19"
                    ),
                    Value::FPRegister(FPRegister::Fs4) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 20"
                    ),
                    Value::FPRegister(FPRegister::Fs5) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 21"
                    ),
                    Value::FPRegister(FPRegister::Fs6) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 22"
                    ),
                    Value::FPRegister(FPRegister::Fs7) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 23"
                    ),
                    Value::FPRegister(FPRegister::Fs8) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 24"
                    ),
                    Value::FPRegister(FPRegister::Fs9) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 25"
                    ),
                    Value::FPRegister(FPRegister::Fs10) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 26"
                    ),
                    Value::FPRegister(FPRegister::Fs11) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 27"
                    ),
                    Value::FPRegister(FPRegister::Ft8) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 28"
                    ),
                    Value::FPRegister(FPRegister::Ft9) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 29"
                    ),
                    Value::FPRegister(FPRegister::Ft10) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 30"
                    ),
                    Value::FPRegister(FPRegister::Ft11) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 31"
                    ),

                    _ => String::new(),
                };
                if load.is_empty() {
                    write!(f, "{} = load {}, {}* {}", rslt, ty, ty, ptr)
                } else {
                    write!(f, "{load}\n  {} = load {}, {}* {}", rslt, ty, ty, temp)
                }
            }
            // write!(f, "{} = load {}, {}* {}", rslt, ty, ty, ptr),
            Store { ty, val, ptr } => {
                // let temp = if let Value::Temp(addr, t) = val {
                //     Value::Temp(*addr, t+1000)
                // } else {
                let t = unsafe { T };
                unsafe {
                    T += 1;
                }
                let temp = Value::Temp(Address(0), t);
                // };
                let load = match ptr {
                    Value::Register(Register::Zero) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 0"
                    ),
                    Value::Register(Register::Ra) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 1"
                    ),
                    Value::Register(Register::Sp) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 2"
                    ),
                    Value::Register(Register::Gp) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 3"
                    ),
                    Value::Register(Register::Tp) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 4"
                    ),
                    Value::Register(Register::T0) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 5"
                    ),
                    Value::Register(Register::T1) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 6"
                    ),
                    Value::Register(Register::T2) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 7"
                    ),
                    Value::Register(Register::S0) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 8"
                    ),
                    Value::Register(Register::S1) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 9"
                    ),
                    Value::Register(Register::A0) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 10"
                    ),
                    Value::Register(Register::A1) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 11"
                    ),
                    Value::Register(Register::A2) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 12"
                    ),
                    Value::Register(Register::A3) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 13"
                    ),
                    Value::Register(Register::A4) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 14"
                    ),
                    Value::Register(Register::A5) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 15"
                    ),
                    Value::Register(Register::A6) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 16"
                    ),
                    Value::Register(Register::A7) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 17"
                    ),
                    Value::Register(Register::S2) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 18"
                    ),
                    Value::Register(Register::S3) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 19"
                    ),
                    Value::Register(Register::S4) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 20"
                    ),
                    Value::Register(Register::S5) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 21"
                    ),
                    Value::Register(Register::S6) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 22"
                    ),
                    Value::Register(Register::S7) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 23"
                    ),
                    Value::Register(Register::S8) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 24"
                    ),
                    Value::Register(Register::S9) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 25"
                    ),
                    Value::Register(Register::S10) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 26"
                    ),
                    Value::Register(Register::S11) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 27"
                    ),
                    Value::Register(Register::T3) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 28"
                    ),
                    Value::Register(Register::T4) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 29"
                    ),
                    Value::Register(Register::T5) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 30"
                    ),
                    Value::Register(Register::T6) => format!(
                        "{temp} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 31"
                    ),

                    Value::FPRegister(FPRegister::Ft0) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 0"
                    ),
                    Value::FPRegister(FPRegister::Ft1) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 1"
                    ),
                    Value::FPRegister(FPRegister::Ft2) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 2"
                    ),
                    Value::FPRegister(FPRegister::Ft3) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 3"
                    ),
                    Value::FPRegister(FPRegister::Ft4) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 4"
                    ),
                    Value::FPRegister(FPRegister::Ft5) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 5"
                    ),
                    Value::FPRegister(FPRegister::Ft6) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 6"
                    ),
                    Value::FPRegister(FPRegister::Ft7) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 7"
                    ),
                    Value::FPRegister(FPRegister::Fs0) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 8"
                    ),
                    Value::FPRegister(FPRegister::Fs1) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 9"
                    ),
                    Value::FPRegister(FPRegister::Fa0) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 10"
                    ),
                    Value::FPRegister(FPRegister::Fa1) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 11"
                    ),
                    Value::FPRegister(FPRegister::Fa2) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 12"
                    ),
                    Value::FPRegister(FPRegister::Fa3) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 13"
                    ),
                    Value::FPRegister(FPRegister::Fa4) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 14"
                    ),
                    Value::FPRegister(FPRegister::Fa5) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 15"
                    ),
                    Value::FPRegister(FPRegister::Fa6) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 16"
                    ),
                    Value::FPRegister(FPRegister::Fa7) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 17"
                    ),
                    Value::FPRegister(FPRegister::Fs2) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 18"
                    ),
                    Value::FPRegister(FPRegister::Fs3) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 19"
                    ),
                    Value::FPRegister(FPRegister::Fs4) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 20"
                    ),
                    Value::FPRegister(FPRegister::Fs5) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 21"
                    ),
                    Value::FPRegister(FPRegister::Fs6) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 22"
                    ),
                    Value::FPRegister(FPRegister::Fs7) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 23"
                    ),
                    Value::FPRegister(FPRegister::Fs8) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 24"
                    ),
                    Value::FPRegister(FPRegister::Fs9) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 25"
                    ),
                    Value::FPRegister(FPRegister::Fs10) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 26"
                    ),
                    Value::FPRegister(FPRegister::Fs11) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 27"
                    ),
                    Value::FPRegister(FPRegister::Ft8) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 28"
                    ),
                    Value::FPRegister(FPRegister::Ft9) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 29"
                    ),
                    Value::FPRegister(FPRegister::Ft10) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 30"
                    ),
                    Value::FPRegister(FPRegister::Ft11) => format!(
                        "{temp} = getelementptr %struct.freg, %struct.freg* %freg, i32 0, i32 31"
                    ),

                    _ => String::new(),
                };
                if load.is_empty() {
                    write!(f, "store {} {}, {}* {}", ty, val, ty, ptr)
                } else {
                    write!(f, "{load}\n  store {} {}, {}* {}", ty, val, ty, temp)
                }
            }
            // write!(f, "store {} {}, {}* {}", ty, val, ty, ptr),
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
                (Type::Double, _) | (_, Type::Double) | (Type::Float, _) | (_, Type::Float) => {
                    write!(f, "{} = bitcast {} {} to {}", rslt, ty, val, ty2)
                }
                _ => write!(f, "{} = bitcast {}* {} to {}*", rslt, ty, val, ty2),
            },

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

            Call { addr } => {
                let t = unsafe { T };
                unsafe {
                    T += 1;
                }
                write!(
                    f,
                    "%ret_{t} = call i64 @func_{}(i64 {addr}, %struct.reg* %reg, %struct.freg* %freg)
  ret i64 %ret_{t}",
                    addr
                )
            }

            SwitchCall {
                ty,
                val,
                dflt,
                tgts,
                next_pc,
            } => {
                let mut s = format!("switch {} {}, label %label_{} [", ty, val, dflt);
                for target in tgts {
                    s += &format!("{} {}, label %call_{next_pc}_{} ", ty, target, target);
                }
                s += "]\n";
                for target in tgts {
                    s += &format!("call_{next_pc}_{}:\n  call i64 @func_{}(%struct.reg* %reg, %struct.freg* %freg)\n  br label %label_{}\n", target, target, next_pc);
                }
                write!(f, "{}", s)
            }

            Ret { ty, val } => {
                write!(f, "ret {} {}", ty, val)
            }
            Unreachable { addr } => {
                write!(f, "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str.d, i64 0, i64 0), i64 {addr})\nunreachable")
            }
        }
    }
}
