use crate::llvm_macro::next_pc;
use crate::riscv_isa as RV;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub entry: RV::Addr,
    pub tdata: RV::Addr,
    pub data_blocks: Vec<RV::DataBlock>,
    pub funcs: Vec<Func>,
    pub src_funcs: HashMap<RV::Addr, String>,
    pub syscall: String,
}

impl Program {
    pub fn in_parts(&self, parts: usize) -> Vec<String> {
        // Merge data blocks
        let mut memory = Vec::new();
        for data_block in &self.data_blocks {
            let RV::Addr(start) = data_block.address;
            memory.resize(start as usize, 0);
            memory.extend(&data_block.bytes);
        }

        // Append the stack
        let stack_len = 8192 * 1024;
        let sp = Value::Addr(RV::Addr(memory.len() as u64 + 8188 * 1024));
        let phdr = Value::Addr(RV::Addr(memory.len() as u64 + 8190 * 1024));
        memory.extend(vec![0; stack_len]);

        // Format the memory array
        let memory_len = memory.len();
        let memory_str = memory
            .iter()
            .map(|b| format!("i8 {b}"))
            .collect::<Vec<_>>()
            .join(", ");
        let memory = format!("@.memory = global [{memory_len} x i8] [{memory_str}]");

        // Build `get_memory_ptr`
        let get_memory_ptr = format!("define i8* @.get_memory_ptr(i64 %addr) {{
  %is_zero = icmp eq i64 0, %addr
  br i1 %is_zero, label %dynamic, label %test_static
test_static:
  %is_static = icmp ugt i64 u0x{memory_len:x}, %addr
  br i1 %is_static, label %static, label %dynamic
static:
  %static_ptr = getelementptr [u0x{memory_len:x} x i8], [u0x{memory_len:x} x i8]* @.memory, i64 0, i64 %addr
  ret i8* %static_ptr
dynamic:
  %dynamic_ptr = inttoptr i64 %addr to i8*
  ret i8* %dynamic_ptr
}}"
        );

        // Build the main dispatcher and function declarations
        let mut dispatcher = Vec::new();
        let mut func_decls = String::new();
        for func in &self.funcs {
            let last_rv_inst = func.inst_blocks.last().unwrap().rv_inst;
            let RV::Addr(mut end) = last_rv_inst.address();
            end += if last_rv_inst.is_compressed() { 2 } else { 4 };
            dispatcher.resize(end as usize, String::from("i64 0"));
            for inst_block in &func.inst_blocks {
                let RV::Addr(addr) = inst_block.rv_inst.address();
                if let Some(name) = self.src_funcs.get(&RV::Addr(addr)) {
                    dispatcher[addr as usize] = format!("i64 ptrtoint (i64 (i64)* @{name} to i64)");
                } else {
                    dispatcher[addr as usize] =
                        format!("i64 ptrtoint (i64 (i64)* @.{} to i64)", func.address);
                }
            }
            func_decls.push_str(&format!("declare i64 @.{}(i64)\n", func.address));
        }
        let dispatcher_len = dispatcher.len();
        let dispatcher_str = dispatcher.join(", ");
        let dispatcher =
            format!("@.dispatcher = global [{dispatcher_len} x i64] [{dispatcher_str}]");

        // Format rounding functions
        let round_ws = Self::format_round("i32", "float", "fptosi");
        let round_wus = Self::format_round("i32", "float", "fptoui");
        let round_ls = Self::format_round("i64", "float", "fptosi");
        let round_lus = Self::format_round("i64", "float", "fptoui");
        let round_wd = Self::format_round("i32", "double", "fptosi");
        let round_wud = Self::format_round("i32", "double", "fptoui");
        let round_ld = Self::format_round("i64", "double", "fptosi");
        let round_lud = Self::format_round("i64", "double", "fptoui");
        let round = format!("{round_ws}\n\n{round_wus}\n\n{round_ls}\n\n{round_lus}\n\n{round_wd}\n\n{round_wud}\n\n{round_ld}\n\n{round_lud}");

        // Format other supporting components
        let entry = self.entry;
        let tdata = self.tdata;
        let src_funcs = self
            .src_funcs
            .values()
            .map(|f| f.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");
        let syscall = &self.syscall;

        // Merge all supporting components
        let mut prog = vec![format!("define i64 @main(i32 %argc, i8** %argv) {{
  ; Initialize the stack pointer
  store i64 {sp}, i64* @.sp

  ; Initialize `argc`
  %argc_addr = add i64 {sp}, 0
  %argc_dest_b = call i8* @.get_memory_ptr(i64 %argc_addr)
  %argc_dest = bitcast i8* %argc_dest_b to i32*
  store i32 %argc, i32* %argc_dest

  ; Initialize `argv`
  %argv_addr = add i64 {sp}, 8
  %argv_dest_b = call i8* @.get_memory_ptr(i64 %argv_addr)
  %argv_dest = bitcast i8* %argv_dest_b to i8**
  %argv_cnt = call i64 @.copy(i8** %argv_dest, i8** %argv)

  ; Create empty `envp`
  %argv_val = ptrtoint i8** %argv_dest to i64
  %argv_offset = mul i64 %argv_cnt, 8
  %envp_val = add i64 %argv_val, %argv_offset

  ; Initialize `auxv`
  %auxv_val = add i64 %envp_val, 8
  %auxv = inttoptr i64 %auxv_val to i64*
  %host_phdr = call i8* @.get_memory_ptr(i64 {phdr})
  call void @.init_auxv(i64* %auxv, i64 {phdr}, i8* %host_phdr, i64 u{tdata})

  ; Load the entry address
  %entry_p= alloca i64
  store i64 u{entry}, i64* %entry_p
  br label %loop

loop:
  %entry = load i64, i64* %entry_p
  %func_addr_ptr = getelementptr [{dispatcher_len} x i64], [{dispatcher_len} x i64]* @.dispatcher, i64 0, i64 %entry
  %func_addr = load i64, i64* %func_addr_ptr
  %func = inttoptr i64 %func_addr to i64 (i64)*
  %next = call i64 %func(i64 %entry)
  store i64 %next, i64* %entry_p
  br label %loop
}}

{func_decls}

{src_funcs}

{memory}

{dispatcher}

@.zero = global i64 zeroinitializer
@.ra = global i64 zeroinitializer
@.sp = global i64 zeroinitializer
@.gp = global i64 zeroinitializer
@.tp = global i64 zeroinitializer
@.t0 = global i64 zeroinitializer
@.t1 = global i64 zeroinitializer
@.t2 = global i64 zeroinitializer
@.s0 = global i64 zeroinitializer
@.s1 = global i64 zeroinitializer
@.a0 = global i64 zeroinitializer
@.a1 = global i64 zeroinitializer
@.a2 = global i64 zeroinitializer
@.a3 = global i64 zeroinitializer
@.a4 = global i64 zeroinitializer
@.a5 = global i64 zeroinitializer
@.a6 = global i64 zeroinitializer
@.a7 = global i64 zeroinitializer
@.s2 = global i64 zeroinitializer
@.s3 = global i64 zeroinitializer
@.s4 = global i64 zeroinitializer
@.s5 = global i64 zeroinitializer
@.s6 = global i64 zeroinitializer
@.s7 = global i64 zeroinitializer
@.s8 = global i64 zeroinitializer
@.s9 = global i64 zeroinitializer
@.s10 = global i64 zeroinitializer
@.s11 = global i64 zeroinitializer
@.t3 = global i64 zeroinitializer
@.t4 = global i64 zeroinitializer
@.t5 = global i64 zeroinitializer
@.t6 = global i64 zeroinitializer

@.ft0 = global double zeroinitializer
@.ft1 = global double zeroinitializer
@.ft2 = global double zeroinitializer
@.ft3 = global double zeroinitializer
@.ft4 = global double zeroinitializer
@.ft5 = global double zeroinitializer
@.ft6 = global double zeroinitializer
@.ft7 = global double zeroinitializer
@.fs0 = global double zeroinitializer
@.fs1 = global double zeroinitializer
@.fa0 = global double zeroinitializer
@.fa1 = global double zeroinitializer
@.fa2 = global double zeroinitializer
@.fa3 = global double zeroinitializer
@.fa4 = global double zeroinitializer
@.fa5 = global double zeroinitializer
@.fa6 = global double zeroinitializer
@.fa7 = global double zeroinitializer
@.fs2 = global double zeroinitializer
@.fs3 = global double zeroinitializer
@.fs4 = global double zeroinitializer
@.fs5 = global double zeroinitializer
@.fs6 = global double zeroinitializer
@.fs7 = global double zeroinitializer
@.fs8 = global double zeroinitializer
@.fs9 = global double zeroinitializer
@.fs10 = global double zeroinitializer
@.fs11 = global double zeroinitializer
@.ft8 = global double zeroinitializer
@.ft9 = global double zeroinitializer
@.ft10 = global double zeroinitializer
@.ft11 = global double zeroinitializer

@.rs = global i64 zeroinitializer

{get_memory_ptr}

; long copy(char** dest, char** src) {{
;     long i = 1;
;     while ((*dest++ = *src++)) {{
;         ++i;
;     }}
;     return i;
; }}
define i64 @.copy(i8** %0, i8** %1) {{
  %3 = load i8*, i8** %1
  store i8* %3, i8** %0
  %4 = icmp eq i8* %3, null
  br i1 %4, label %14, label %5

5:                                                ; preds = %2, %5
  %6 = phi i64 [ %11, %5 ], [ 1, %2 ]
  %7 = phi i8** [ %10, %5 ], [ %1, %2 ]
  %8 = phi i8** [ %9, %5 ], [ %0, %2 ]
  %9 = getelementptr i8*, i8** %8, i64 1
  %10 = getelementptr i8*, i8** %7, i64 1
  %11 = add i64 %6, 1
  %12 = load i8*, i8** %10
  store i8* %12, i8** %9
  %13 = icmp eq i8* %12, null
  br i1 %13, label %14, label %5

14:                                               ; preds = %5, %2
  %15 = phi i64 [ 1, %2 ], [ %11, %5 ]
  ret i64 %15
}}

%.struct.Elf64_Phdr = type {{ i32, i32, i64, i64, i64, i64, i64, i64 }}
@.init_auxv.entries = constant [23 x i64] [i64 0, i64 1, i64 2, i64 4, i64 5, i64 6, i64 7, i64 8, i64 9, i64 10, i64 11, i64 12, i64 13, i64 14, i64 15, i64 16, i64 17, i64 23, i64 24, i64 25, i64 26, i64 31, i64 51]
declare i64 @getauxval(i64)

define void @.init_auxv(i64* %0, i64 %1, i8* %2, i64 %3) {{
  %5 = call i64 @getauxval(i64 3)
  %6 = call i64 @getauxval(i64 5)
  %7 = icmp ne i64 %5, 0
  %8 = icmp ne i64 %6, 0
  %9 = select i1 %7, i1 %8, i1 false
  br i1 %9, label %12, label %10

10:                                               ; preds = %15, %4
  %11 = phi i64* [ %0, %4 ], [ %17, %15 ]
  br label %42

12:                                               ; preds = %4
  %13 = bitcast i8* %2 to %.struct.Elf64_Phdr*
  %14 = inttoptr i64 %5 to %.struct.Elf64_Phdr*
  br label %18

15:                                               ; preds = %36
  %16 = getelementptr i64, i64* %0, i64 1
  store i64 3, i64* %0
  %17 = getelementptr i64, i64* %0, i64 2
  store i64 %1, i64* %16
  br label %10

18:                                               ; preds = %12, %36
  %19 = phi i64 [ 0, %12 ], [ %39, %36 ]
  %20 = phi %.struct.Elf64_Phdr* [ %13, %12 ], [ %37, %36 ]
  %21 = phi %.struct.Elf64_Phdr* [ %14, %12 ], [ %38, %36 ]
  %22 = getelementptr %.struct.Elf64_Phdr, %.struct.Elf64_Phdr* %21, i64 0, i32 0
  %23 = load i32, i32* %22
  switch i32 %23, label %33 [
    i32 7, label %24
    i32 1685382482, label %28
  ]

24:                                               ; preds = %18
  %25 = bitcast %.struct.Elf64_Phdr* %20 to i8*
  %26 = bitcast %.struct.Elf64_Phdr* %21 to i8*
  call void @.memcpy(i8* %25, i8* %26, i64 56)
  %27 = getelementptr %.struct.Elf64_Phdr, %.struct.Elf64_Phdr* %20, i64 0, i32 3
  store i64 %3, i64* %27
  br label %36

28:                                               ; preds = %18
  %29 = bitcast %.struct.Elf64_Phdr* %20 to i8*
  %30 = bitcast %.struct.Elf64_Phdr* %21 to i8*
  call void @.memcpy(i8* %29, i8* %30, i64 56)
  %31 = getelementptr %.struct.Elf64_Phdr, %.struct.Elf64_Phdr* %20, i64 0, i32 3
  store i64 %3, i64* %31
  %32 = getelementptr %.struct.Elf64_Phdr, %.struct.Elf64_Phdr* %20, i64 0, i32 6
  store i64 2760, i64* %32
  br label %36

33:                                               ; preds = %18
  %34 = bitcast %.struct.Elf64_Phdr* %20 to i8*
  %35 = bitcast %.struct.Elf64_Phdr* %21 to i8*
  call void @.memcpy(i8* %34, i8* %35, i64 56)
  br label %36

36:                                               ; preds = %24, %33, %28
  %37 = getelementptr %.struct.Elf64_Phdr, %.struct.Elf64_Phdr* %20, i64 1
  %38 = getelementptr %.struct.Elf64_Phdr, %.struct.Elf64_Phdr* %21, i64 1
  %39 = add i64 %19, 1
  %40 = icmp eq i64 %39, %6
  br i1 %40, label %15, label %18

41:                                               ; preds = %52
  ret void

42:                                               ; preds = %10, %52
  %43 = phi i64 [ %54, %52 ], [ 0, %10 ]
  %44 = phi i64* [ %53, %52 ], [ %11, %10 ]
  %45 = getelementptr [23 x i64], [23 x i64]* @.init_auxv.entries, i64 0, i64 %43
  %46 = load i64, i64* %45
  %47 = call i64 @getauxval(i64 %46)
  %48 = icmp eq i64 %47, 0
  br i1 %48, label %52, label %49

49:                                               ; preds = %42
  %50 = getelementptr i64, i64* %44, i64 1
  store i64 %46, i64* %44
  %51 = getelementptr i64, i64* %44, i64 2
  store i64 %47, i64* %50
  br label %52

52:                                               ; preds = %49, %42
  %53 = phi i64* [ %51, %49 ], [ %44, %42 ]
  %54 = add i64 %43, 1
  %55 = icmp eq i64 %54, 23
  br i1 %55, label %41, label %42
}}

; void memcpy(unsigned char* dest, unsigned char* src, unsigned long count) {{
;     for (unsigned long i = 0; i < count; ++i) {{
;         *dest++ = *src++;
;     }}
; }}

define void @.memcpy(i8* %0, i8* %1, i64 %2) {{
  %4 = icmp eq i64 %2, 0
  br i1 %4, label %5, label %6

5:                                                ; preds = %6, %3
  ret void

6:                                                ; preds = %3, %6
  %7 = phi i64 [ %13, %6 ], [ 0, %3 ]
  %8 = phi i8* [ %12, %6 ], [ %0, %3 ]
  %9 = phi i8* [ %10, %6 ], [ %1, %3 ]
  %10 = getelementptr i8, i8* %9, i64 1
  %11 = load i8, i8* %9
  %12 = getelementptr i8, i8* %8, i64 1
  store i8 %11, i8* %8
  %13 = add nuw i64 %7, 1
  %14 = icmp eq i64 %13, %2
  br i1 %14, label %5, label %6
}}

{round}

{syscall}
")];

        // Format other functions in parts
        let decls = "declare i8* @.get_memory_ptr(i64)
declare i64 @.system_call(i64, i64, i64, i64, i64, i64, i64)

declare float @llvm.sqrt.float(float %arg)
declare double @llvm.sqrt.double(double %arg)
declare float @llvm.fma.float(float %arg1, float %arg2, float %arg3)
declare double @llvm.fma.double(double %arg1, double %arg2, double %arg3)
declare float @llvm.fabs.float(float %arg)
declare double @llvm.fabs.double(double %arg)
declare float @llvm.copysign.float(float %mag, float %sgn)
declare double @llvm.copysign.double(double %mag, double %sgn)

declare i32 @.round_i32_float_fptosi(float, i1)
declare i32 @.round_i32_float_fptoui(float, i1)
declare i64 @.round_i64_float_fptosi(float, i1)
declare i64 @.round_i64_float_fptoui(float, i1)
declare i32 @.round_i32_double_fptosi(double, i1)
declare i32 @.round_i32_double_fptoui(double, i1)
declare i64 @.round_i64_double_fptosi(double, i1)
declare i64 @.round_i64_double_fptoui(double, i1)

@.zero = external global i64
@.ra = external global i64
@.sp = external global i64
@.gp = external global i64
@.tp = external global i64
@.t0 = external global i64
@.t1 = external global i64
@.t2 = external global i64
@.s0 = external global i64
@.s1 = external global i64
@.a0 = external global i64
@.a1 = external global i64
@.a2 = external global i64
@.a3 = external global i64
@.a4 = external global i64
@.a5 = external global i64
@.a6 = external global i64
@.a7 = external global i64
@.s2 = external global i64
@.s3 = external global i64
@.s4 = external global i64
@.s5 = external global i64
@.s6 = external global i64
@.s7 = external global i64
@.s8 = external global i64
@.s9 = external global i64
@.s10 = external global i64
@.s11 = external global i64
@.t3 = external global i64
@.t4 = external global i64
@.t5 = external global i64
@.t6 = external global i64

@.ft0 = external global double
@.ft1 = external global double
@.ft2 = external global double
@.ft3 = external global double
@.ft4 = external global double
@.ft5 = external global double
@.ft6 = external global double
@.ft7 = external global double
@.fs0 = external global double
@.fs1 = external global double
@.fa0 = external global double
@.fa1 = external global double
@.fa2 = external global double
@.fa3 = external global double
@.fa4 = external global double
@.fa5 = external global double
@.fa6 = external global double
@.fa7 = external global double
@.fs2 = external global double
@.fs3 = external global double
@.fs4 = external global double
@.fs5 = external global double
@.fs6 = external global double
@.fs7 = external global double
@.fs8 = external global double
@.fs9 = external global double
@.fs10 = external global double
@.fs11 = external global double
@.ft8 = external global double
@.ft9 = external global double
@.ft10 = external global double
@.ft11 = external global double

@.rs = external global i64";
        let part_len = (self.funcs.len() as f64 / parts as f64).ceil() as usize;
        let funcs = self
            .funcs
            .chunks(part_len)
            .map(|fs| {
                fs.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join("\n\n")
            })
            .map(|part| format!("{decls}\n\n{part}"));

        prog.extend(funcs);
        prog
    }

    fn format_round(int: &str, fp: &str, func: &str) -> String {
        format!(
            "define {int} @.round_{int}_{fp}_{func}({fp} %0, i1 %1) {{
  %3 = {func} {fp} %0 to {int}
  %4 = fcmp ule {fp} %0, 0.000000e+00
  %5 = or i1 %4, %1
  %6 = xor i1 %5, true
  %7 = sitofp {int} %3 to {fp}
  %8 = fcmp une {fp} %7, %0
  %9 = select i1 %6, i1 %8, i1 false
  br i1 %9, label %10, label %12

10:                                               ; preds = %2
  %11 = add {int} %3, 1
  br label %18

12:                                               ; preds = %2
  %13 = fcmp olt {fp} %0, 0.000000e+00
  %14 = and i1 %13, %1
  %15 = select i1 %14, i1 %8, i1 false
  %16 = sext i1 %15 to {int}
  %17 = add {int} %16, %3
  br label %18

18:                                               ; preds = %12, %10
  %19 = phi {int} [ %11, %10 ], [ %17, %12 ]
  ret {int} %19
}}"
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Func {
    pub section: String,
    pub symbol: String,
    pub address: RV::Addr,
    pub inst_blocks: Vec<InstBlock>,
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut dispatcher = String::from("switch i64 %entry, label %unreachable [");
        let mut inst_blocks = String::new();
        for inst_block in &self.inst_blocks {
            let addr = Value::Addr(inst_block.rv_inst.address());
            dispatcher += &format!("i64 {addr}, label %{addr} ");
            inst_blocks += &format!("{inst_block}\n");
        }
        dispatcher.pop();
        dispatcher += "]";
        let last_rv_inst = self.inst_blocks.last().unwrap().rv_inst;
        let next_pc = next_pc!(
            next_pc,
            last_rv_inst.address(),
            last_rv_inst.is_compressed()
        );
        write!(
            f,
            "; {} {} <{}>
define i64 @.{}(i64 %entry) {{
  {dispatcher}
unreachable:
  unreachable

{inst_blocks}
{next_pc}:
  ret i64 {next_pc}
}}",
            self.address, self.section, self.symbol, self.address
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstBlock {
    pub rv_inst: RV::Inst,
    pub insts: Vec<Inst>,
}

impl Display for InstBlock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let addr = Value::Addr(self.rv_inst.address());
        let insts_str = self
            .insts
            .iter()
            .map(|i| format!("  {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let br = if let Some(Inst::Ret { .. }) | Some(Inst::ConBr { .. }) | Some(Inst::Br { .. }) =
            self.insts.last()
        {
            String::from("")
        } else {
            let next_pc = next_pc!(
                next_pc,
                self.rv_inst.address(),
                self.rv_inst.is_compressed()
            );
            let br = Inst::Br { addr: next_pc };
            format!("\n  {br}")
        };
        write!(f, "; {:?}\n{addr}:\n{insts_str}{br}", self.rv_inst)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Inst {
    // Terminator Instructions
    Ret {
        val: Value,
    },
    ConBr {
        cond: Value,
        iftrue: Value,
        iffalse: Value,
    },
    Br {
        addr: Value,
    },

    // Unary Operations
    Fneg {
        rslt: Value,
        ty: Type,
        op: Value,
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
        mo: MO,
    },
    Cmpxchg {
        rslt: Value,
        ty: Type,
        ptr: Value,
        cmp: Value,
        new: Value,
        mo: MO,
    },
    Atomicrmw {
        rslt: Value,
        op: Op,
        ty: Type,
        ptr: Value,
        val: Value,
        mo: MO,
    },

    // Conversion Operations
    Trunc {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Zext {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Sext {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Fptrunc {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Fpext {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Fptoui {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
        rm: RV::RM,
    },
    Fptosi {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
        rm: RV::RM,
    },
    Uitofp {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Sitofp {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },
    Bitcast {
        rslt: Value,
        ty1: Type,
        val: Value,
        ty2: Type,
    },

    // Other Operations
    Icmp {
        rslt: Value,
        cond: Cond,
        op1: Value,
        op2: Value,
    },
    Fcmp {
        rslt: Value,
        fcond: FCond,
        op1: Value,
        op2: Value,
    },
    Select {
        rslt: Value,
        cond: Value,
        ty: Type,
        op1: Value,
        op2: Value,
    },

    // Standard C/C++ Library Intrinsics
    Sqrt {
        rslt: Value,
        ty: Type,
        arg: Value,
    },
    Fma {
        rslt: Value,
        ty: Type,
        arg1: Value,
        arg2: Value,
        arg3: Value,
    },
    Fabs {
        rslt: Value,
        ty: Type,
        arg: Value,
    },
    Copysign {
        rslt: Value,
        ty: Type,
        mag: Value,
        sgn: Value,
    },

    // Misc
    Getmemptr {
        rslt: Value,
        addr: Value,
    },
    Syscall {
        rslt: Value,
        nr: Value,
        arg1: Value,
        arg2: Value,
        arg3: Value,
        arg4: Value,
        arg5: Value,
        arg6: Value,
    },
    Unreachable,
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Inst::*;

        match self {
            // Terminator Instructions
            Ret { val } => write!(f, "ret i64 {val}"),
            ConBr { cond, iftrue, iffalse } => write!(f, "br i1 {cond}, label %{iftrue}, label %{iffalse}"),
            Br { addr } => write!(f, "br label %{addr}"),

            // Unary Operations
            Fneg { rslt, ty, op } => write!(f, "{rslt} = fneg {ty} {op}"),

            // Binary Operations
            Add { rslt, ty, op1, op2 } => write!(f, "{rslt} = add {ty} {op1}, {op2}"),
            Fadd { rslt, ty, op1, op2 } => write!(f, "{rslt} = fadd {ty} {op1}, {op2}"),
            Sub { rslt, ty, op1, op2 } => write!(f, "{rslt} = sub {ty} {op1}, {op2}"),
            Fsub { rslt, ty, op1, op2 } => write!(f, "{rslt} = fsub {ty} {op1}, {op2}"),
            Mul { rslt, ty, op1, op2 } => write!(f, "{rslt} = mul {ty} {op1}, {op2}"),
            Fmul { rslt, ty, op1, op2 } => write!(f, "{rslt} = fmul {ty} {op1}, {op2}"),
            Udiv { rslt, ty, op1, op2 } => write!(f, "{rslt} = udiv {ty} {op1}, {op2}"),
            Sdiv { rslt, ty, op1, op2 } => write!(f, "{rslt} = sdiv {ty} {op1}, {op2}"),
            Fdiv { rslt, ty, op1, op2 } => write!(f, "{rslt} = fdiv {ty} {op1}, {op2}"),
            Urem { rslt, ty, op1, op2 } => write!(f, "{rslt} = urem {ty} {op1}, {op2}"),
            Srem { rslt, ty, op1, op2 } => write!(f, "{rslt} = srem {ty} {op1}, {op2}"),

            // Bitwise Binary Operations
            Shl { rslt, ty, op1, op2 } => write!(f, "{rslt} = shl {ty} {op1}, {op2}"),
            Lshr { rslt, ty, op1, op2 } => write!(f, "{rslt} = lshr {ty} {op1}, {op2}"),
            Ashr { rslt, ty, op1, op2 } => write!(f, "{rslt} = ashr {ty} {op1}, {op2}"),
            And { rslt, ty, op1, op2 } => write!(f, "{rslt} = and {ty} {op1}, {op2}"),
            Or { rslt, ty, op1, op2 } => write!(f, "{rslt} = or {ty} {op1}, {op2}"),
            Xor { rslt, ty, op1, op2 } => write!(f, "{rslt} = xor {ty} {op1}, {op2}"),

            // Aggregate Operations
            Extractvalue { rslt, ty, val, idx } => write!(f, "{rslt} = extractvalue {{ {ty}, i1 }} {val}, {idx}"),

            // Memory Access and Addressing Operations
            Load { rslt, ty, ptr } => write!(f, "{rslt} = load {ty}, {ty}* {ptr}"),
            Store { ty, val, ptr } => write!(f, "store {ty} {val}, {ty}* {ptr}"),
            Fence { mo } => write!(f, "fence {mo}"),
            Cmpxchg { rslt, ty, ptr, cmp, new, mo } => write!(f, "{rslt} = cmpxchg {ty}* {ptr}, {ty} {cmp}, {ty} {new} {mo} monotonic"),
            Atomicrmw { rslt, op, ty, ptr, val, mo } => write!(f, "{rslt} = atomicrmw {op} {ty}* {ptr}, {ty} {val} {mo}"),

            // Conversion Operations
            Trunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = trunc {ty1} {val} to {ty2}"),
            Zext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = zext {ty1} {val} to {ty2}"),
            Sext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sext {ty1} {val} to {ty2}"),
            Fptrunc { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fptrunc {ty1} {val} to {ty2}"),
            Fpext { rslt, ty1, val, ty2 } => write!(f, "{rslt} = fpext {ty1} {val} to {ty2}"),
            Fptoui { rslt, ty1, val, ty2, rm } => match rm {
                RV::RM::Rdn => write!(f, "{rslt} = call {ty2} @.round_{ty2}_{ty1}_fptoui({ty1} {val}, i1 1)"),
                RV::RM::Rup => write!(f, "{rslt} = call {ty2} @.round_{ty2}_{ty1}_fptoui({ty1} {val}, i1 0)"),
                _ => write!(f, "{rslt} = fptoui {ty1} {val} to {ty2}"),
            }
            Fptosi { rslt, ty1, val, ty2, rm } => match rm {
                RV::RM::Rdn => write!(f, "{rslt} = call {ty2} @.round_{ty2}_{ty1}_fptosi({ty1} {val}, i1 1)"),
                RV::RM::Rup => write!(f, "{rslt} = call {ty2} @.round_{ty2}_{ty1}_fptosi({ty1} {val}, i1 0)"),
                _ => write!(f, "{rslt} = fptosi {ty1} {val} to {ty2}"),
            }
            Uitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = uitofp {ty1} {val} to {ty2}"),
            Sitofp { rslt, ty1, val, ty2 } => write!(f, "{rslt} = sitofp {ty1} {val} to {ty2}"),
            Bitcast { rslt, ty1, val, ty2 } => match (ty1, ty2) {
                (Type::Float, _) | (_, Type::Float) | (Type::Double, _) | (_, Type::Double) => write!(f, "{rslt} = bitcast {ty1} {val} to {ty2}"),
                _ => write!(f, "{rslt} = bitcast {ty1}* {val} to {ty2}*"),
            },

            // Other Operations
            Icmp { rslt, cond, op1, op2 } => write!(f, "{rslt} = icmp {cond} i64 {op1}, {op2}"),
            Fcmp {rslt,fcond,op1,op2} => write!(f, "{rslt} = fcmp {fcond} double {op1}, {op2}"),
            Select {rslt,cond, ty, op1,op2} => write!(f, "{rslt} = select i1 {cond}, {ty} {op1}, {ty} {op2}"),

            // Standard C/C++ Library Intrinsics
            Sqrt { rslt, ty, arg } => write!(f,"{rslt} = call {ty} @llvm.sqrt.{ty}({ty} {arg})"),
            Fma { rslt, ty, arg1, arg2, arg3 } => write!(f, "{rslt} = call {ty} @llvm.fma.{ty}({ty} {arg1}, {ty} {arg2}, {ty} {arg3})"),
            Fabs { rslt, ty, arg } => write!(f, "{rslt} = call {ty} @llvm.fabs.{ty}({ty} {arg})"),
            Copysign { rslt, ty, mag, sgn } => write!(f, "{rslt} = call {ty} @llvm.copysign.{ty}({ty} {mag}, {ty} {sgn})"),

            // Misc
            Getmemptr { rslt, addr } => write!(f, "{rslt} = call i8* @.get_memory_ptr(i64 {addr})"),
            Syscall { rslt, nr, arg1, arg2, arg3, arg4, arg5, arg6 } =>
                write!(f, "{rslt} = call i64 (i64, i64, i64, i64, i64, i64, i64) @.system_call(i64 {nr}, i64 {arg1}, i64 {arg2}, i64 {arg3}, i64 {arg4}, i64 {arg5}, i64 {arg6})"),
            Unreachable => write!(f, "unreachable"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    fn fmt(&self, f: &mut Formatter) -> Result {
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Reg(RV::Reg),
    FReg(RV::FReg),
    Imm(RV::Imm),
    Addr(RV::Addr),
    Temp(RV::Addr, usize),
    RS,
    Label(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Value::*;

        match self {
            Reg(reg) => write!(f, "@.{reg}"),
            FReg(freg) => write!(f, "@.{freg}"),
            Imm(imm) => write!(f, "{imm}"),
            Addr(addr) => write!(f, "u{addr}"),
            Temp(addr, i) => write!(f, "%u{addr}_{i}"),
            RS => write!(f, "@.rs"),
            Label(label) => write!(f, "{label}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MO {
    Mono,
    Aq,
    Rl,
    AqRl,
}

impl Display for MO {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use MO::*;

        match self {
            Mono => write!(f, "monotonic"),
            Aq => write!(f, "acquire"),
            Rl => write!(f, "release"),
            AqRl => write!(f, "acq_rel"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Op {
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

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Op::*;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Cond {
    Eq,
    Ne,
    Uge,
    Ult,
    Sgt,
    Sge,
    Slt,
    Sle,
}

impl Display for Cond {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Cond::*;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FCond {
    Oeq,
    Olt,
    Ole,
}

impl Display for FCond {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use FCond::*;

        match self {
            Oeq => write!(f, "oeq"),
            Olt => write!(f, "olt"),
            Ole => write!(f, "ole"),
        }
    }
}
