use crate::llvm_isa::*;
use crate::llvm_macro::*;
use crate::riscv_isa as RV;
use rayon::prelude::*;

pub fn run(rv_prog: RV::Program, sys_call: Option<String>, src_funcs: Vec<String>) -> Program {
    let (memory, sp, phdr) = build_memory(&rv_prog.data_blocks);
    Program {
        entry: rv_prog.entry,
        tdata: rv_prog.tdata.unwrap_or(RV::Addr(0)),
        funcs: rv_prog
            .code_blocks
            .into_par_iter()
            .map(translate_rv_code_block)
            .collect(),
        src_funcs: src_funcs.clone(),
        sys_call,
        memory,
        sp,
        phdr,
        func_syms: rv_prog.symbols,
        native_mem_func: false,
    }
}

fn build_memory(data_blocks: &Vec<RV::DataBlock>) -> (Vec<u8>, Value, Value) {
    // Merge data blocks
    let mut memory = Vec::new();
    for data_block in data_blocks {
        let RV::Addr(start) = data_block.address;
        memory.resize(start as usize, 0);
        memory.extend(&data_block.bytes);
    }

    // Append the stack
    let stack_len = 8192 * 1024;
    let sp = Value::Addr(RV::Addr(memory.len() as u64 + 8188 * 1024));
    let phdr = Value::Addr(RV::Addr(memory.len() as u64 + 8190 * 1024));
    memory.extend(vec![0; stack_len]);

    (memory, sp, phdr)
}

fn translate_rv_code_block(rv_code_block: RV::CodeBlock) -> Func {
    Func {
        section: rv_code_block.section,
        symbol: rv_code_block.symbol,
        address: rv_code_block.address,
        inst_blocks: rv_code_block
            .insts
            .into_iter()
            .map(translate_rv_inst)
            .collect(),
        stack_vars: Vec::new(),
        dynamic: true,
        used_regs: Vec::new(),
        used_fregs: Vec::new(),
    }
}

#[allow(dead_code)]
fn translate_rv_inst(rv_inst: RV::Inst) -> InstBlock {
    let insts = translate_rv_inst!(rv_inst,
        // RV32I
        Lui { rd, imm } => {
            Shl { rslt: _0, ty: i_32, op1: imm, op2: { Value::Imm(RV::Imm(12)) } },
            Sext { rslt: _1, ty1: i_32, val: _0, ty2: i_64 },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Auipc { rd, imm } => {
            Shl { rslt: _0, ty: i_32, op1: imm, op2: { Value::Imm(RV::Imm(12)) } },
            Sext { rslt: _1, ty1: i_32, val: _0, ty2: i_64 },
            Add { rslt: _2, ty: i_64, op1: _1, op2: pc },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Jal { rd, addr } => {
            Store { ty: i_64, val: next_pc, ptr: rd },
            Ret { val: addr },
        }
        Jalr { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: next_pc, ptr: rd },
            Ret { val: _1 },
        }
        Beq { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Eq }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bne { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Ne }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Blt { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Slt }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bge { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Sge }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bltu { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Ult }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bgeu { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Uge }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Lb { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Load { rslt: _3, ty: i_8, ptr: _2 },
            Sext { rslt: _4, ty1: i_8, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Lh { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Bitcast { rslt: _3, ty1: i_8, val: _2, ty2: i_16 },
            Load { rslt: _4, ty: i_16, ptr: _3 },
            Sext { rslt: _5, ty1: i_16, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Lw { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Bitcast { rslt: _3, ty1: i_8, val: _2, ty2: i_32 },
            Load { rslt: _4, ty: i_32, ptr: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Lbu { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Load { rslt: _3, ty: i_8, ptr: _2 },
            Zext { rslt: _4, ty1: i_8, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Lhu { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Bitcast { rslt: _3, ty1: i_8, val: _2, ty2: i_16 },
            Load { rslt: _4, ty: i_16, ptr: _3 },
            Zext { rslt: _5, ty1: i_16, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Sb { rs2, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs2 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_8 },
            Load { rslt: _2, ty: i_64, ptr: rs1 },
            Add { rslt: _3, ty: i_64, op1: _2, op2: imm },
            Getmemptr { rslt: _4, addr: _3 },
            Store { ty: i_8, val: _1, ptr: _4 },
        }
        Sh { rs2, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs2 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_16 },
            Load { rslt: _2, ty: i_64, ptr: rs1 },
            Add { rslt: _3, ty: i_64, op1: _2, op2: imm },
            Getmemptr { rslt: _4, addr: _3 },
            Bitcast { rslt: _5, ty1: i_8, val: _4, ty2: i_16 },
            Store { ty: i_16, val: _1, ptr: _5 },
        }
        Sw { rs2, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs2 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs1 },
            Add { rslt: _3, ty: i_64, op1: _2, op2: imm },
            Getmemptr { rslt: _4, addr: _3 },
            Bitcast { rslt: _5, ty1: i_8, val: _4, ty2: i_32 },
            Store { ty: i_32, val: _1, ptr: _5 },
        }
        Addi { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Slti { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Slt }, op1: _0, op2: imm },
            Zext { rslt: _2, ty1: i_1, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Sltiu { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Ult }, op1: _0, op2: imm },
            Zext { rslt: _2, ty1: i_1, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Xori { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Xor { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Ori { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Or { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Andi { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            And { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Slli { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Shl { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Srli { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Lshr { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Srai { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Ashr { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Add { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Add { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Sub { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Sub { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Sll { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Shl { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Slt { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Slt }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        Sltu { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Ult }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        Xor { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Xor { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Srl { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Lshr { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Sra { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Ashr { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Or { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Or { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        And { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            And { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Fence {} => {
            Fence { mo: { MO::SeqCst } },
        }
        Ecall {} => {
            Load { rslt: _0, ty: i_64, ptr: { Value::Reg(RV::Reg::A7) } },
            Load { rslt: _1, ty: i_64, ptr: { Value::Reg(RV::Reg::A0) } },
            Load { rslt: _2, ty: i_64, ptr: { Value::Reg(RV::Reg::A1) } },
            Load { rslt: _3, ty: i_64, ptr: { Value::Reg(RV::Reg::A2) } },
            Load { rslt: _4, ty: i_64, ptr: { Value::Reg(RV::Reg::A3) } },
            Load { rslt: _5, ty: i_64, ptr: { Value::Reg(RV::Reg::A4) } },
            Load { rslt: _6, ty: i_64, ptr: { Value::Reg(RV::Reg::A5) } },
            Syscall { rslt: _7, nr: _0, arg1: _1, arg2: _2, arg3: _3, arg4: _4, arg5: _5, arg6: _6 },
            Store { ty: i_64, val: _7, ptr: { Value::Reg(RV::Reg::A0) } },
        }
        Ebreak {} => {
            Unreachable {},
        }

        // RV64I (in addition to RV32I)
        Lwu { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Bitcast { rslt: _3, ty1: i_8, val: _2, ty2: i_32 },
            Load { rslt: _4, ty: i_32, ptr: _3 },
            Zext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Ld { rd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Bitcast { rslt: _3, ty1: i_8, val: _2, ty2: i_64 },
            Load { rslt: _4, ty: i_64, ptr: _3 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Sd { rs2, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs2 },
            Load { rslt: _1, ty: i_64, ptr: rs1 },
            Add { rslt: _2, ty: i_64, op1: _1, op2: imm },
            Getmemptr { rslt: _3, addr: _2 },
            Bitcast { rslt: _4, ty1: i_8, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _0, ptr: _4 },
        }
        Addiw { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Trunc { rslt: _2, ty1: i_64, val: _1, ty2: i_32 },
            Sext { rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        Slliw { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Shl { rslt: _2, ty: i_32, op1: _1, op2: imm },
            Sext { rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        Srliw { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Lshr { rslt: _2, ty: i_32, op1: _1, op2: imm },
            Sext { rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        Sraiw { rd, rs1, imm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Ashr { rslt: _2, ty: i_32, op1: _1, op2: imm },
            Sext { rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        Addw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Add { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Subw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Sub { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Sllw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Shl { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Srlw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Lshr { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Sraw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Ashr { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }

        // RV32/RV64 Zifencei
        FenceI {} => {
            Fence { mo: { MO::SeqCst } },
        }

        // RV32/RV64 Zicsr
        Csrrw { rd, csr, rs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Csrrs { rd, csr, rs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Csrrc { rd, csr, rs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Csrrwi { rd, csr, imm } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Csrrsi { rd, csr, imm } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Csrrci { rd, csr, imm } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }

        // RV32M
        Mul { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Mul { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Mulh { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Sext { rslt: _1, ty1: i_64, val: _0, ty2: i_128 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Sext { rslt: _3, ty1: i_64, val: _2, ty2: i_128 },
            Mul { rslt: _4, ty: i_128, op1: _1, op2: _3 },
            Lshr { rslt: _5, ty: i_128, op1: _4, op2: { Value::Imm(RV::Imm(64)) } },
            Trunc { rslt: _6, ty1: i_128, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        Mulhsu { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Sext { rslt: _1, ty1: i_64, val: _0, ty2: i_128 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Zext { rslt: _3, ty1: i_64, val: _2, ty2: i_128 },
            Mul { rslt: _4, ty: i_128, op1: _1, op2: _3 },
            Lshr { rslt: _5, ty: i_128, op1: _4, op2: { Value::Imm(RV::Imm(64)) } },
            Trunc { rslt: _6, ty1: i_128, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        Mulhu { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Zext { rslt: _1, ty1: i_64, val: _0, ty2: i_128 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Zext { rslt: _3, ty1: i_64, val: _2, ty2: i_128 },
            Mul { rslt: _4, ty: i_128, op1: _1, op2: _3 },
            Lshr { rslt: _5, ty: i_128, op1: _4, op2: { Value::Imm(RV::Imm(64)) } },
            Trunc { rslt: _6, ty1: i_128, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        Div { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Sdiv { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Divu { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Udiv { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Rem { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Srem { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Remu { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Urem { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Store { ty: i_64, val: _2, ptr: rd },
        }

        // RV64M (in addition to RV32M)
        Mulw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Mul { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Divw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Sdiv { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Divuw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Udiv { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Remw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Srem { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        Remuw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Urem { rslt: _2, ty: i_64, op1: _0, op2: _1 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
        }

        // RV32A
        LrW { mo, rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_32, ptr: _2 },
            Sext { rslt: _4, ty1: i_32, val: _3, ty2: i_64 },
            Store { ty: i_64, val: _4, ptr: rd },
            Store { ty: i_64, val: _4, ptr: rs },
        }
        ScW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Load { rslt: _5, ty: i_64, ptr: rs2 },
            Trunc { rslt: _6, ty1: i_64, val: _5, ty2: i_32 },
            Cmpxchg { rslt: _7, ty: i_32, ptr: _2, cmp: _4, new: _6, mo: mo },
            Extractvalue { rslt: _8, ty: i_32, val: _7, idx: { Value::Imm(RV::Imm(1)) } },
            Xor { rslt: _9, ty: i_1, op1: _8, op2: { Value::Imm(RV::Imm(1)) } },
            Zext { rslt: _10, ty1: i_1, val: _9, ty2: i_64 },
            Store { ty: i_64, val: _10, ptr: rd },
        }
        AmoswapW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Xchg }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmoaddW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Add }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmoxorW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Xor }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmoandW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::And }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmoorW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Or }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmominW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Min }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmomaxW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Max }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmominuW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Umin }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }
        AmomaxuW { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Trunc { rslt: _4, ty1: i_64, val: _3, ty2: i_32 },
            Atomicrmw { rslt: _5, op: { Op::Umax }, ty: i_32, ptr: _2, val: _4, mo: mo },
            Sext { rslt: _6, ty1: i_32, val: _5, ty2: i_64 },
            Store { ty: i_64, val: _6, ptr: rd },
        }

        // RV64A (in addition to RV32A)
        LrD { mo, rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: _2 },
            Store { ty: i_64, val: _3, ptr: rd },
            Store { ty: i_64, val: _3, ptr: rs },
        }
        ScD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs },
            Load { rslt: _4, ty: i_64, ptr: rs2 },
            Cmpxchg { rslt: _5, ty: i_64, ptr: _2, cmp: _3, new: _4, mo: mo },
            Extractvalue { rslt: _6, ty: i_64, val: _5, idx: { Value::Imm(RV::Imm(1)) } },
            Xor { rslt: _7, ty: i_1, op1: _6, op2: { Value::Imm(RV::Imm(1)) } },
            Zext { rslt: _8, ty1: i_1, val: _7, ty2: i_64 },
            Store { ty: i_64, val: _8, ptr: rd },
        }
        AmoswapD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Xchg }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmoaddD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Add }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmoxorD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Xor }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmoandD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::And }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmoorD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Or }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmominD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Min }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmomaxD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Max }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmominuD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Umin }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }
        AmomaxuD { mo, rd, rs2, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Getmemptr { rslt: _1, addr: _0 },
            Bitcast { rslt: _2, ty1: i_8, val: _1, ty2: i_64 },
            Load { rslt: _3, ty: i_64, ptr: rs2 },
            Atomicrmw { rslt: _4, op: { Op::Umax }, ty: i_64, ptr: _2, val: _3, mo: mo },
            Store { ty: i_64, val: _4, ptr: rd },
        }

        // RV32F
        Flw { frd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Bitcast { rslt: _3, ty1: i_8, val: _2, ty2: i_32 },
            Load { rslt: _4, ty: i_32, ptr: _3 },
            Bitcast { rslt: _5, ty1: i_32, val: _4, ty2: f },
            Fpext { rslt: _6, ty1: f, val: _5, ty2: d },
            Store { ty: d, val: _6, ptr: frd },
        }
        Fsw { frs2, imm, rs1 } => {
            Load { rslt: _0, ty: d, ptr: frs2 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Bitcast { rslt: _2, ty1: f, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: i_64, ptr: rs1 },
            Add { rslt: _4, ty: i_64, op1: _3, op2: imm },
            Getmemptr { rslt: _5, addr: _4 },
            Bitcast { rslt: _6, ty1: i_8, val: _5, ty2: i_32 },
            Store { ty: i_32, val: _2, ptr: _6 },
        }
        FmaddS { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Load { rslt: _4, ty: d, ptr: frs3 },
            Fptrunc { rslt: _5, ty1: d, val: _4, ty2: f },
            Fma { rslt: _6, ty: f, arg1: _1, arg2: _3, arg3: _5 },
            Fpext { rslt: _7, ty1: f, val: _6, ty2: d },
            Store { ty: d, val: _7, ptr: frd },
        }
        FmsubS { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Load { rslt: _4, ty: d, ptr: frs3 },
            Fptrunc { rslt: _5, ty1: d, val: _4, ty2: f },
            Fneg { rslt: _6, ty: f, op: _5 },
            Fma { rslt: _7, ty: f, arg1: _1, arg2: _3, arg3: _6 },
            Fpext { rslt: _8, ty1: f, val: _7, ty2: d },
            Store { ty: d, val: _8, ptr: frd },
        }
        FnmsubS { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Load { rslt: _4, ty: d, ptr: frs3 },
            Fptrunc { rslt: _5, ty1: d, val: _4, ty2: f },
            Fneg { rslt: _6, ty: f, op: _5 },
            Fma { rslt: _7, ty: f, arg1: _1, arg2: _3, arg3: _6 },
            Fneg { rslt: _8, ty: f, op: _7 },
            Fpext { rslt: _9, ty1: f, val: _8, ty2: d },
            Store { ty: d, val: _9, ptr: frd },
        }
        FnmaddS { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Load { rslt: _4, ty: d, ptr: frs3 },
            Fptrunc { rslt: _5, ty1: d, val: _4, ty2: f },
            Fma { rslt: _6, ty: f, arg1: _1, arg2: _3, arg3: _5 },
            Fneg { rslt: _7, ty: f, op: _6 },
            Fpext { rslt: _8, ty1: f, val: _7, ty2: d },
            Store { ty: d, val: _8, ptr: frd },
        }
        FaddS { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Fadd { rslt: _4, ty: f, op1: _1, op2: _3 },
            Fpext { rslt: _5, ty1: f, val: _4, ty2: d },
            Store { ty: d, val: _5, ptr: frd },
        }
        FsubS { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Fsub { rslt: _4, ty: f, op1: _1, op2: _3 },
            Fpext { rslt: _5, ty1: f, val: _4, ty2: d },
            Store { ty: d, val: _5, ptr: frd },
        }
        FmulS { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Fmul { rslt: _4, ty: f, op1: _1, op2: _3 },
            Fpext { rslt: _5, ty1: f, val: _4, ty2: d },
            Store { ty: d, val: _5, ptr: frd },
        }
        FdivS { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Fdiv { rslt: _4, ty: f, op1: _1, op2: _3 },
            Fpext { rslt: _5, ty1: f, val: _4, ty2: d },
            Store { ty: d, val: _5, ptr: frd },
        }
        FsqrtS { frd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Sqrt { rslt: _2, ty: f, arg: _1 },
            Fpext { rslt: _3, ty1: f, val: _2, ty2: d },
            Store { ty: d, val: _3, ptr: frd },
        }
        FsgnjS { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Copysign { rslt: _4, ty: f, mag: _1, sgn: _3 },
            Fpext { rslt: _5, ty1: f, val: _4, ty2: d },
            Store { ty: d, val: _5, ptr: frd },
        }
        FsgnjnS { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Fneg { rslt: _4, ty: f, op: _3 },
            Copysign { rslt: _5, ty: f, mag: _1, sgn: _4 },
            Fpext { rslt: _6, ty1: f, val: _5, ty2: d },
            Store { ty: d, val: _6, ptr: frd },
        }
        FsgnjxS { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Bitcast { rslt: _2, ty1: f, val: _1, ty2: i_32 },
            Load { rslt: _3, ty: d, ptr: frs2 },
            Fptrunc { rslt: _4, ty1: d, val: _3, ty2: f },
            Bitcast { rslt: _5, ty1: f, val: _4, ty2: i_32 },
            Xor { rslt: _6, ty: i_32, op1: _2, op2: _5 },
            Bitcast { rslt: _7, ty1: i_32, val: _6, ty2: f },
            Copysign { rslt: _8, ty: f, mag: _1, sgn: _7 },
            Fpext { rslt: _9, ty1: f, val: _8, ty2: d },
            Store { ty: d, val: _9, ptr: frd },
        }
        FminS { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Fcmp { rslt: _4, fcond: { FCond::Olt }, op1: _1, op2: _3 },
            Select { rslt: _5, cond: _4, ty: f, op1: _1, op2: _3 },
            Fpext { rslt: _6, ty1: f, val: _5, ty2: d },
            Store { ty: d, val: _6, ptr: frd },
        }
        FmaxS { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Fptrunc { rslt: _3, ty1: d, val: _2, ty2: f },
            Fcmp { rslt: _4, fcond: { FCond::Olt }, op1: _1, op2: _3 },
            Select { rslt: _5, cond: _4, ty: f, op1: _3, op2: _1 },
            Fpext { rslt: _6, ty1: f, val: _5, ty2: d },
            Store { ty: d, val: _6, ptr: frd },
        }
        FcvtWS { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Fptosi { rslt: _2, ty1: f, val: _1, ty2: i_32, rm: rm },
            Sext {rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FcvtWuS { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Fptoui { rslt: _2, ty1: f, val: _1, ty2: i_32, rm: rm },
            Zext {rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FmvXW { rd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Bitcast { rslt: _2, ty1: f, val: _1, ty2: i_32 },
            Sext {rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FeqS { rd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Oeq }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FltS { rd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Olt }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FleS { rd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Ole }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FclassS { rd, frs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        FcvtSW { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Sitofp { rslt: _2, ty1: i_32, val: _1, ty2: f },
            Fpext {rslt: _3, ty1: f, val: _2, ty2: d },
            Store { ty: d, val: _3, ptr: frd },
        }
        FcvtSWu { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Uitofp { rslt: _2, ty1: i_32, val: _1, ty2: f },
            Fpext {rslt: _3, ty1: f, val: _2, ty2: d },
            Store { ty: d, val: _3, ptr: frd },
        }
        FmvWX { frd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Bitcast { rslt: _2, ty1: i_32, val: _1, ty2: f },
            Fpext {rslt: _3, ty1: f, val: _2, ty2: d },
            Store { ty: d, val: _3, ptr: frd },
        }

        // RV64F (in addition to RV32F)
        FcvtLS { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Fptosi { rslt: _2, ty1: f, val: _1, ty2: i_64, rm: rm },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        FcvtLuS { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Fptoui { rslt: _2, ty1: f, val: _1, ty2: i_64, rm: rm },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        FcvtSL { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Sitofp { rslt: _1, ty1: i_64, val: _0, ty2: f },
            Fpext {rslt: _2, ty1: f, val: _1, ty2: d },
            Store { ty: d, val: _2, ptr: frd },
        }
        FcvtSLu { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Uitofp { rslt: _1, ty1: i_64, val: _0, ty2: f },
            Fpext {rslt: _2, ty1: f, val: _1, ty2: d },
            Store { ty: d, val: _2, ptr: frd },
        }

        // RV32D
        Fld { frd, imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Getmemptr { rslt: _2, addr: _1 },
            Bitcast { rslt: _3, ty1: i_8, val: _2, ty2: i_64 },
            Load { rslt: _4, ty: i_64, ptr: _3 },
            Bitcast { rslt: _5, ty1: i_64, val: _4, ty2: d },
            Store { ty: d, val: _5, ptr: frd },
        }
        Fsd { frs2, imm, rs1 } => {
            Load { rslt: _0, ty: d, ptr: frs2 },
            Bitcast { rslt: _1, ty1: d, val: _0, ty2: i_64 },
            Load { rslt: _2, ty: i_64, ptr: rs1 },
            Add { rslt: _3, ty: i_64, op1: _2, op2: imm },
            Getmemptr { rslt: _4, addr: _3 },
            Bitcast { rslt: _5, ty1: i_8, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _1, ptr: _5 },
        }
        FmaddD { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Load { rslt: _2, ty: d, ptr: frs3 },
            Fma { rslt: _3, ty: d, arg1: _0, arg2: _1, arg3: _2 },
            Store { ty: d, val: _3, ptr: frd },
        }
        FmsubD { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Load { rslt: _2, ty: d, ptr: frs3 },
            Fneg { rslt: _3, ty: d, op: _2 },
            Fma { rslt: _4, ty: d, arg1: _0, arg2: _1, arg3: _3 },
            Store { ty: d, val: _4, ptr: frd },
        }
        FnmsubD { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Load { rslt: _2, ty: d, ptr: frs3 },
            Fneg { rslt: _3, ty: d, op: _2 },
            Fma { rslt: _4, ty: d, arg1: _0, arg2: _1, arg3: _3 },
            Fneg { rslt: _5, ty: d, op: _4 },
            Store { ty: d, val: _5, ptr: frd },
        }
        FnmaddD { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Load { rslt: _2, ty: d, ptr: frs3 },
            Fma { rslt: _3, ty: d, arg1: _0, arg2: _1, arg3: _2 },
            Fneg { rslt: _4, ty: d, op: _3 },
            Store { ty: d, val: _4, ptr: frd },
        }
        FaddD { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fadd { rslt: _2, ty: d, op1: _0, op2: _1 },
            Store { ty: d, val: _2, ptr: frd },
        }
        FsubD { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fsub { rslt: _2, ty: d, op1: _0, op2: _1 },
            Store { ty: d, val: _2, ptr: frd },
        }
        FmulD { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fmul { rslt: _2, ty: d, op1: _0, op2: _1 },
            Store { ty: d, val: _2, ptr: frd },
        }
        FdivD { frd, frs1, frs2, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fdiv { rslt: _2, ty: d, op1: _0, op2: _1 },
            Store { ty: d, val: _2, ptr: frd },
        }
        FsqrtD { frd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Sqrt { rslt: _1, ty: d, arg: _0 },
            Store { ty: d, val: _1, ptr: frd },
        }
        FsgnjD { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Copysign { rslt: _2, ty: d, mag: _0, sgn: _1 },
            Store { ty: d, val: _2, ptr: frd },
        }
        FsgnjnD { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fneg { rslt: _2, ty: d, op: _1 },
            Copysign { rslt: _3, ty: d, mag: _0, sgn: _2 },
            Store { ty: d, val: _3, ptr: frd },
        }
        FsgnjxD { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Bitcast { rslt: _1, ty1: d, val: _0, ty2: i_64 },
            Load { rslt: _2, ty: d, ptr: frs2 },
            Bitcast { rslt: _3, ty1: d, val: _2, ty2: i_64 },
            Xor { rslt: _4, ty: i_64, op1: _1, op2: _3 },
            Bitcast { rslt: _5, ty1: i_64, val: _4, ty2: d },
            Copysign { rslt: _6, ty: d, mag: _0, sgn: _5 },
            Store { ty: d, val: _6, ptr: frd },
        }
        FminD { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Olt }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: d, op1: _0, op2: _1 },
            Store { ty: d, val: _3, ptr: frd },
        }
        FmaxD { frd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Olt }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, ty: d, op1: _1, op2: _0 },
            Store { ty: d, val: _3, ptr: frd },
        }
        FcvtSD { frd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Fpext {rslt: _2, ty1: f, val: _1, ty2: d },
            Store { ty: d, val: _2, ptr: frd },
        }
        FcvtDS { frd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Store { ty: d, val: _0, ptr: frd },
        }
        FeqD { rd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Oeq }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FltD { rd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Olt }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FleD { rd, frs1, frs2 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Load { rslt: _1, ty: d, ptr: frs2 },
            Fcmp { rslt: _2, fcond: { FCond::Ole }, op1: _0, op2: _1 },
            Zext { rslt: _3, ty1: i_1, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        FclassD { rd, frs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        FcvtWD { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptosi { rslt: _1, ty1: d, val: _0, ty2: i_32, rm: rm },
            Sext {rslt: _2, ty1: i_32, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        FcvtWuD { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptoui { rslt: _1, ty1: d, val: _0, ty2: i_32, rm: rm },
            Zext {rslt: _2, ty1: i_32, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        FcvtDW { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Sitofp { rslt: _2, ty1: i_32, val: _1, ty2: d },
            Store { ty: d, val: _2, ptr: frd },
        }
        FcvtDWu { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Uitofp { rslt: _2, ty1: i_32, val: _1, ty2: d },
            Store { ty: d, val: _2, ptr: frd },
        }

        // RV64D (in addition to RV32D)
        FcvtLD { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptosi { rslt: _1, ty1: d, val: _0, ty2: i_64, rm: rm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        FcvtLuD { rd, frs1, rm } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptoui { rslt: _1, ty1: d, val: _0, ty2: i_64, rm: rm },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        FmvXD { rd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Bitcast { rslt: _1, ty1: d, val: _0, ty2: i_64 },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        FcvtDL { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Sitofp { rslt: _1, ty1: i_64, val: _0, ty2: d },
            Store { ty: d, val: _1, ptr: frd },
        }
        FcvtDLu { frd, rs1, rm } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Uitofp { rslt: _1, ty1: i_64, val: _0, ty2: d },
            Store { ty: d, val: _1, ptr: frd },
        }
        FmvDX { frd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Bitcast { rslt: _1, ty1: i_64, val: _0, ty2: d },
            Store { ty: d, val: _1, ptr: frd },
        }

        // Pseudoinstructions
        Nop {} => {}
        Li { rd, imm } => {
            Store { ty: i_64, val: imm, ptr: rd },
        }
        Mv { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Store { ty: i_64, val: _0, ptr: rd },
        }
        Not { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Xor { rslt: _1, ty: i_64, op1: _0, op2: { Value::Imm(RV::Imm(-1)) } },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Neg { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Sub { rslt: _1, ty: i_64, op1: { Value::Imm(RV::Imm(0)) }, op2: _0 },
            Store { ty: i_64, val: _1, ptr: rd },
        }
        Negw { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Sub { rslt: _1, ty: i_64, op1: { Value::Imm(RV::Imm(0)) }, op2: _0 },
            Trunc { rslt: _2, ty1: i_64, val: _1, ty2: i_32 },
            Sext { rslt: _3, ty1: i_32, val: _2, ty2: i_64 },
            Store { ty: i_64, val: _3, ptr: rd },
        }
        SextW { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Sext { rslt: _2, ty1: i_32, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Seqz { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Eq }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Zext { rslt: _2, ty1: i_1, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Snez { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Ne }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Zext { rslt: _2, ty1: i_1, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Sltz { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Slt }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Zext { rslt: _2, ty1: i_1, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }
        Sgtz { rd, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Sgt }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Zext { rslt: _2, ty1: i_1, val: _1, ty2: i_64 },
            Store { ty: i_64, val: _2, ptr: rd },
        }

        FmvS { frd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Store { ty: d, val: _0, ptr: frd },
        }
        FabsS { frd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Fabs { rslt: _2, ty: f, arg: _1 },
            Fpext { rslt: _3, ty1: f, val: _2, ty2: d },
            Store { ty: d, val: _3, ptr: frd },
        }
        FnegS { frd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fptrunc { rslt: _1, ty1: d, val: _0, ty2: f },
            Fneg { rslt: _2, ty: f, op: _1 },
            Fpext { rslt: _3, ty1: f, val: _2, ty2: d },
            Store { ty: d, val: _3, ptr: frd },
        }
        FmvD { frd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Store { ty: d, val: _0, ptr: frd },
        }
        FabsD { frd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fabs { rslt: _1, ty: d, arg: _0 },
            Store { ty: d, val: _1, ptr: frd },
        }
        FnegD { frd, frs1 } => {
            Load { rslt: _0, ty: d, ptr: frs1 },
            Fneg { rslt: _1, ty: d, op: _0 },
            Store { ty: d, val: _1, ptr: frd },
        }

        Beqz { rs1, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Eq }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Select { rslt: _2, cond: _1, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _2 },
        }
        Bnez { rs1, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Ne }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Select { rslt: _2, cond: _1, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _2 },
        }
        Blez { rs1, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Sle }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Select { rslt: _2, cond: _1, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _2 },
        }
        Bgez { rs1, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Sge }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Select { rslt: _2, cond: _1, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _2 },
        }
        Bltz { rs1, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Slt }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Select { rslt: _2, cond: _1, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _2 },
        }
        Bgtz { rs1, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Icmp { rslt: _1, cond: { Cond::Sgt }, op1: _0, op2: { Value::Imm(RV::Imm(0)) } },
            Select { rslt: _2, cond: _1, ty: i_64, op1: addr, op2: next_pc },
            Ret { val: _2 },
        }

        J { addr } => {
            Ret { val: addr },
        }
        PseudoJal { addr } => {
            Store { ty: i_64, val: next_pc, ptr: { Value::Reg(RV::Reg::Ra) } },
            Ret { val: addr },
        }
        Jr { rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Ret { val: _0 },
        }
        PseudoJalr { rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Store { ty: i_64, val: next_pc, ptr: { Value::Reg(RV::Reg::Ra) } },
            Ret { val: _0 },
        }
        Ret {} => {
            Load { rslt: _0, ty: i_64, ptr: { Value::Reg(RV::Reg::Ra) } },
            Ret { val: _0 },
        }

        PseudoFence {} => {
            Fence { mo: { MO::SeqCst } },
        }

        Rdinstret { rd } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Rdcycle { rd } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Rdtime { rd } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }

        Csrr { rd, csr } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Csrw { csr, rs1 } => {}
        Csrs { csr, rs1 } => {}
        Csrc { csr, rs1 } => {}

        Csrwi { csr, imm } => {}
        Csrsi { csr, imm } => {}
        Csrci { csr, imm } => {}

        Frcsr { rd } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Fscsr { rd, rs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        PseudoFscsr { rs1 } => {}

        Frrm { rd } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Fsrm { rd, rs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        PseudoFsrm { rs1 } => {}

        Frflags { rd } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        Fsflags { rd, rs1 } => {
            Store { ty: i_64, val: { Value::Imm(RV::Imm(0)) }, ptr: rd },
        }
        PseudoFsflags { rs1 } => {}

        // Misc
        Unimp {} => {}
        OffsetJalr { imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Store { ty: i_64, val: next_pc, ptr: { Value::Reg(RV::Reg::Ra) } },
            Ret { val: _1 },
        }
        OffsetJr { imm, rs1 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Ret { val: _1 },
        }
    );
    let insts = insts
        .into_iter()
        .filter_map(|inst| match inst {
            Inst::Load {
                rslt,
                ty,
                ptr: Value::Reg(RV::Reg::Zero),
            } => Some(Inst::Add {
                rslt,
                ty,
                op1: Value::Imm(RV::Imm(0)),
                op2: Value::Imm(RV::Imm(0)),
            }),
            Inst::Store {
                ptr: Value::Reg(RV::Reg::Zero),
                ..
            } => None,
            inst => Some(inst),
        })
        .collect();
    InstBlock { rv_inst, insts }
}
