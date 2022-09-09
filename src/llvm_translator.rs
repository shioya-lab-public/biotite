use crate::llvm_isa::*;
use crate::llvm_macro::*;
use crate::riscv_isa as RV;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(rv_prog: RV::Program, src_funcs: HashMap<RV::Addr, String>) -> Program {
    Program {
        entry: rv_prog.entry,
        data_blocks: rv_prog.data_blocks,
        funcs: rv_prog
            .code_blocks
            .into_par_iter()
            .map(translate_rv_code_block)
            .collect(),
        src_funcs,
    }
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
    }
}

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
            Store { ty: i_64, val: next_pc, ptr: rd },
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
            Ret { val: _1 },
        }
        Beq { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Eq }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bne { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Ne }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Blt { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Slt }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bge { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Sge }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bltu { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Ult }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, op1: addr, op2: next_pc },
            Ret { val: _3 },
        }
        Bgeu { rs1, rs2, addr } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Load { rslt: _1, ty: i_64, ptr: rs2 },
            Icmp { rslt: _2, cond: { Cond::Uge }, op1: _0, op2: _1 },
            Select { rslt: _3, cond: _2, op1: addr, op2: next_pc },
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
            Fence { mo: { MO::AqRl } },
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
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Add { rslt: _2, ty: i_32, op1: _1, op2: imm },
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
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Add { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Subw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sub { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
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
            Fence { mo: { MO::AqRl } },
        }

        // RV32/RV64 Zicsr
        Csrrw { rd, csr, rs1 } => {}
        Csrrs { rd, csr, rs1 } => {}
        Csrrc { rd, csr, rs1 } => {}
        Csrrwi { rd, csr, imm } => {}
        Csrrsi { rd, csr, imm } => {}
        Csrrci { rd, csr, imm } => {}

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
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Mul { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Divw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Sdiv { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Divuw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Udiv { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Remw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Srem { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }
        Remuw { rd, rs1, rs2 } => {
            Load { rslt: _0, ty: i_64, ptr: rs1 },
            Trunc { rslt: _1, ty1: i_64, val: _0, ty2: i_32 },
            Load { rslt: _2, ty: i_64, ptr: rs2 },
            Trunc { rslt: _3, ty1: i_64, val: _2, ty2: i_32 },
            Urem { rslt: _4, ty: i_32, op1: _1, op2: _3 },
            Sext { rslt: _5, ty1: i_32, val: _4, ty2: i_64 },
            Store { ty: i_64, val: _5, ptr: rd },
        }

        // RV32A
        LrW { mo, rd, rs1 } => {}
        ScW { mo, rd, rs2, rs1 } => {}
        AmoswapW { mo, rd, rs2, rs1 } => {}
        AmoaddW { mo, rd, rs2, rs1 } => {}
        AmoxorW { mo, rd, rs2, rs1 } => {}
        AmoandW { mo, rd, rs2, rs1 } => {}
        AmoorW { mo, rd, rs2, rs1 } => {}
        AmominW { mo, rd, rs2, rs1 } => {}
        AmomaxW { mo, rd, rs2, rs1 } => {}
        AmominuW { mo, rd, rs2, rs1 } => {}
        AmomaxuW { mo, rd, rs2, rs1 } => {}

        // RV64A (in addition to RV32A)
        LrD { mo, rd, rs1 } => {}
        ScD { mo, rd, rs2, rs1 } => {}
        AmoswapD { mo, rd, rs2, rs1 } => {}
        AmoaddD { mo, rd, rs2, rs1 } => {}
        AmoxorD { mo, rd, rs2, rs1 } => {}
        AmoandD { mo, rd, rs2, rs1 } => {}
        AmoorD { mo, rd, rs2, rs1 } => {}
        AmominD { mo, rd, rs2, rs1 } => {}
        AmomaxD { mo, rd, rs2, rs1 } => {}
        AmominuD { mo, rd, rs2, rs1 } => {}
        AmomaxuD { mo, rd, rs2, rs1 } => {}

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
            Load { rslt: _0, ty: f, ptr: frs1 },
            Load { rslt: _1, ty: f, ptr: frs2 },
            Load { rslt: _2, ty: f, ptr: frs3 },
            Fmul { rslt: _3, ty: f, op1: _0, op2: _1 },
            Fadd { rslt: _4, ty: f, op1: _3, op2: _2 },
            Store { ty: d, val: _4, ptr: frd },
        }
        FmsubS { frd, frs1, frs2, frs3, rm } => {
            Load { rslt: _0, ty: f, ptr: frs1 },
            Load { rslt: _1, ty: f, ptr: frs2 },
            Load { rslt: _2, ty: f, ptr: frs3 },
            Fmul { rslt: _3, ty: f, op1: _0, op2: _1 },
            Fsub { rslt: _4, ty: f, op1: _3, op2: _2 },
            Store { ty: d, val: _4, ptr: frd },
        }
        //     Fld {
    //         frd,
    //         imm,
    //         rs1,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
    //         Getdataptr { rslt: _2, ty: i_64, addr: _1 },
    //         Bitcast { rslt: _3, ty: i_8, val: _2, ty2: i_64 },
    //         Load { rslt: _4, ty: i_64, ptr: _3 },
    //         Bitcast { rslt: _5, ty: i_64, val: _4, ty2: _d },
    //         Store { ty: _d, val: _5, ptr: frd },
    //         
    //     },
    //     Fsd {
    //         frs2,
    //         imm,
    //         rs1,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs2 },
    //         Bitcast { rslt: _1, ty: _d, val: _0, ty2: i_64 },
    //         Load { rslt: _2, ty: i_64, ptr: rs1 },
    //         Add { rslt: _3, ty: i_64, op1: _2, op2: imm },
    //         Getdataptr { rslt: _4, ty: i_64, addr: _3 },
    //         Bitcast { rslt: _5, ty: i_8, val: _4, ty2: i_64 },
    //         Store { ty: i_64, val: _1, ptr: _5 },
    //         
    //     },
        
        
    //     FmvXD {
    //         rd,
    //         frs1,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Bitcast { rslt: _1, ty: _d, val: _0, ty2: i_64 },
    //         Store { ty: i_64, val: _1, ptr: rd },
    //         
    //     },
    //     FmvDX {
    //         frd,
    //         rs1,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Bitcast { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FmvXW {
    //         rd,
    //         frs1,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Bitcast { rslt: _1, ty: _d, val: _0, ty2: i_64 },
    //         Store { ty: i_64, val: _1, ptr: rd },
    //         
    //     },
    //     FmvWX {
    //         frd,
    //         rs1,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Bitcast { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FmulD {
    //         frd,
    //         frs1,
    //         frs2,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fmul { rslt: _2, ty: _d, op1: _0, op2: _1 },
    //         Store { ty: _d, val: _2, ptr: frd },
    //         
    //     },
    //     FsubD {
    //         frd,
    //         frs1,
    //         frs2,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fsub { rslt: _2, ty: _d, op1: _0, op2: _1 },
    //         Store { ty: _d, val: _2, ptr: frd },
    //         
    //     },
    //     FaddD {
    //         frd,
    //         frs1,
    //         frs2,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fadd { rslt: _2, ty: _d, op1: _0, op2: _1 },
    //         Store { ty: _d, val: _2, ptr: frd },
    //         
    //     },
    //     FnegD {
    //         frd,
    //         frs1,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Fneg { rslt: _1, ty: _d, op1: _0 },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FmsubD {
    //         frd,
    //         frs1,
    //         frs2,
    //         frs3,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Load { rslt: _2, ty: _d, ptr: frs3 },
    //         Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
    //         Fsub { rslt: _4, ty: _d, op1: _3, op2: _2 },
    //         Store { ty: _d, val: _4, ptr: frd },
    //         
    //     },
    //     FnmsubD {
    //         frd,
    //         frs1,
    //         frs2,
    //         frs3,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Load { rslt: _2, ty: _d, ptr: frs3 },
    //         Fmul { rslt: _3, ty: _d, op1: _0, op2: _1 },
    //         Fneg { rslt: _4, ty: _d, op1: _3 },
    //         Fadd { rslt: _5, ty: _d, op1: _4, op2: _2 },
    //         Store { ty: _d, val: _5, ptr: frd },
    //         
    //     },
    //     FmulS {
    //         frd,
    //         frs1,
    //         frs2,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fmul { rslt: _2, ty: _d, op1: _0, op2: _1 },
    //         Store { ty: _d, val: _2, ptr: frd },
    //         
    //     },
    //     FdivD {
    //         frd,
    //         frs1,
    //         frs2,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fdiv { rslt: _2, ty: _d, op1: _0, op2: _1 },
    //         Store { ty: _d, val: _2, ptr: frd },
    //         
    //     },
    //     FdivS {
    //         frd,
    //         frs1,
    //         frs2,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fdiv { rslt: _2, ty: _d, op1: _0, op2: _1 },
    //         Store { ty: _d, val: _2, ptr: frd },
    //         
    //     },

    //     FcvtSL {
    //         frd,
    //         rs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Sitofp { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FcvtDWu {
    //         frd,
    //         rs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Uitofp { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FcvtDLu {
    //         frd,
    //         rs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Uitofp { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FcvtDW {
    //         frd,
    //         rs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Sitofp { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FcvtDL {
    //         frd,
    //         rs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Sitofp { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FcvtWD {
    //         rd,
    //         frs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Fptosi { rslt: _1, ty: _d, val: _0, ty2: i_64 },
    //         Store { ty: i_64, val: _1, ptr: rd },
    //         
    //     },
    //     FcvtLD {
    //         rd,
    //         frs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Fptosi { rslt: _1, ty: _d, val: _0, ty2: i_64 },
    //         Store { ty: i_64, val: _1, ptr: rd },
    //         
    //     },
    //     FcvtWuD {
    //         rd,
    //         frs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Fptoui { rslt: _1, ty: _d, val: _0, ty2: i_64 },
    //         Store { ty: i_64, val: _1, ptr: rd },
    //         
    //     },
    //     FcvtSW {
    //         frd,
    //         rs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Sitofp { rslt: _1, ty: i_64, val: _0, ty2: _d },
    //         Store { ty: _d, val: _1, ptr: frd },
    //         
    //     },
    //     FcvtDS {
    //         frd,
    //         frs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Store { ty: _d, val: _0, ptr: frd },
    //         
    //     },
    //     FcvtSD {
    //         frd,
    //         frs1,
    //         rm,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Store { ty: _d, val: _0, ptr: frd },
    //         
    //     },

    //     FmvD {
    //         frd,
    //         frs1,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Store { ty: _d, val: _0, ptr: frd },
    //         
    //     },
    //     FeqD {
    //         rd,
    //         frs1,
    //         frs2,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fcmp { rslt: _2, fcond: oeq, ty: _d, op1: _0, op2: _1 },
    //         Zext { rslt: _3, ty: i_1, val: _2, ty2: i_64 },
    //         Store { ty: i_64, val: _3, ptr: rd },
    //         
    //     },
    //     FltD {
    //         rd,
    //         frs1,
    //         frs2,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fcmp { rslt: _2, fcond: olt, ty: _d, op1: _0, op2: _1 },
    //         Zext { rslt: _3, ty: i_1, val: _2, ty2: i_64 },
    //         Store { ty: i_64, val: _3, ptr: rd },
    //         
    //     },
    //     FleD {
    //         rd,
    //         frs1,
    //         frs2,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fcmp { rslt: _2, fcond: ole, ty: _d, op1: _0, op2: _1 },
    //         Zext { rslt: _3, ty: i_1, val: _2, ty2: i_64 },
    //         Store { ty: i_64, val: _3, ptr: rd },
    //         
    //     },
    //     FeqS {
    //         rd,
    //         frs1,
    //         frs2,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fcmp { rslt: _2, fcond: oeq, ty: _d, op1: _0, op2: _1 },
    //         Zext { rslt: _3, ty: i_1, val: _2, ty2: i_64 },
    //         Store { ty: i_64, val: _3, ptr: rd },
    //         
    //     },
    //     FltS {
    //         rd,
    //         frs1,
    //         frs2,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fcmp { rslt: _2, fcond: olt, ty: _d, op1: _0, op2: _1 },
    //         Zext { rslt: _3, ty: i_1, val: _2, ty2: i_64 },
    //         Store { ty: i_64, val: _3, ptr: rd },
    //         
    //     },
    //     FleS {
    //         rd,
    //         frs1,
    //         frs2,
    //     } => {
    //         Load { rslt: _0, ty: _d, ptr: frs1 },
    //         Load { rslt: _1, ty: _d, ptr: frs2 },
    //         Fcmp { rslt: _2, fcond: ole, ty: _d, op1: _0, op2: _1 },
    //         Zext { rslt: _3, ty: i_1, val: _2, ty2: i_64 },
    //         Store { ty: i_64, val: _3, ptr: rd },
    //         
    //     },
    );


    

    
        
        

    //     // Pseudoinstructions
    //     Nop { address, raw } => {
    //         
    //     },
    //     Li {
    //         rd,
    //         imm,
    //     } => {
    //         Store { ty: i_64, val: imm, ptr: rd },
    //         
    //     },
    //     Mv {
    //         rd,
    //         rs1,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Store { ty: i_64, val: _0, ptr: rd },
    //         
    //     },
    //     Not {
    //         rd,
    //         rs1,
    //     } => {
    //         let imm = &Immediate(-1);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Xor { rslt: _1, ty: i_64, op1: _0, op2: imm },
    //             Store { ty: i_64, val: _1, ptr: rd },
    //             
    //         }
    //     }
    //     Neg {
    //         rd,
    //         rs1,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Sub { rslt: _1, ty: i_64, op1: imm, op2: _0 },
    //             Store { ty: i_64, val: _1, ptr: rd },
    //             
    //         }
    //     }
    //     Negw {
    //         rd,
    //         rs1,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Sub { rslt: _1, ty: i_64, op1: imm, op2: _0 },
    //             Trunc { rslt: _2, ty: i_64, val: _1, ty2: i_32 },
    //             Sext { rslt: _3, ty: i_32, val: _2, ty2: i_64 },
    //             Store { ty: i_64, val: _3, ptr: rd },
    //             
    //         }
    //     }
    //     SextW {
    //         rd,
    //         rs1,
    //     } => {
    //         Load { rslt: _0, ty: i_64, ptr: rs1 },
    //         Store { ty: i_64, val: _0, ptr: rd },
    //         
    //     },
    //     Seqz {
    //         rd,
    //         rs1,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: eq, ty: i_64, op1: _0, op2: imm },
    //             Zext { rslt: _2, ty: i_1, val: _1, ty2: i_64 },
    //             Store { ty: i_64, val: _2, ptr: rd },
    //             
    //         }
    //     }
    //     Snez {
    //         rd,
    //         rs1,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: ne, ty: i_64, op1: _0, op2: imm },
    //             Zext { rslt: _2, ty: i_1, val: _1, ty2: i_64 },
    //             Store { ty: i_64, val: _2, ptr: rd },
    //             
    //         }
    //     }
    //     Sltz {
    //         rd,
    //         rs1,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: slt, ty: i_64, op1: _0, op2: imm },
    //             Zext { rslt: _2, ty: i_1, val: _1, ty2: i_64 },
    //             Store { ty: i_64, val: _2, ptr: rd },
    //             
    //         }
    //     }
    //     Sgtz {
    //         rd,
    //         rs1,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: sgt, ty: i_64, op1: _0, op2: imm },
    //             Zext { rslt: _2, ty: i_1, val: _1, ty2: i_64 },
    //             Store { ty: i_64, val: _2, ptr: rd },
    //             
    //         }
    //     }

    //     Beqz {
    //         rs1,
    //         addr,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: eq, ty: i_64, op1: _0, op2: imm },
    //             ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
    //         }
    //     }
    //     Bnez {
    //         rs1,
    //         addr,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: ne, ty: i_64, op1: _0, op2: imm },
    //             ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
    //         }
    //     }
    //     Blez {
    //         rs1,
    //         addr,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: sle, ty: i_64, op1: _0, op2: imm },
    //             ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
    //         }
    //     }
    //     Bgez {
    //         rs1,
    //         addr,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: sge, ty: i_64, op1: _0, op2: imm },
    //             ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
    //         }
    //     }
    //     Bltz {
    //         rs1,
    //         addr,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: slt, ty: i_64, op1: _0, op2: imm },
    //             ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
    //         }
    //     }
    //     Bgtz {
    //         rs1,
    //         addr,
    //     } => {
    //         let imm = &Immediate(0);
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Icmp { rslt: _1, cond: sgt, ty: i_64, op1: _0, op2: imm },
    //             ConBr { cond: _1, iftrue: addr, iffalse: next_pc },
    //         }
    //     }

    //     // J { addr } => {
    //     //     Ret { ty: i_64, val: addr },
    //     // },
    //     J { addr } => {
    //         if func_targets.contains(&addr) {
    //             {
    //                 // Call { addr: addr },
    //                 Ret{ty: i_64, val: addr},
    //             }
    //         } else {
    //             {
    //                 UnconBr { addr: addr },
    //                 // Ret{ty: i_64, val: addr},
    //             }
    //         }
    //     }
    //     Jr { rs1 } => {
    //         // let default = Value::Address(Address(0x1));
    //         // let targets: Vec<_> = self
    //         //     .intra_targets.get(&func).cloned().unwrap_or_default()
    //         //     .into_iter()
    //         //     .map(Value::Address)
    //         //     // .filter_map(|addr| if &addr > address {Some(Value::Address(addr))} else {None})
    //         //     .collect();
    //         //     // if targets.len() >= 300 {
    //         //     //     println!("jr {:?} - {}", address, targets.len());
    //         //     // }
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             // Switch { ty: i_64, val: _0, dflt: default, tgts: targets },
    //         Ret { ty: i_64, val: _0 },
    //         }
    //     }
    //     OffsetJr {
    //         imm,
    //         rs1,
    //     } => {
    //         // let default = Value::Address(Address(0x1));
    //         // let targets: Vec<_> = self
    //         //     .intra_targets.get(&func).cloned().unwrap_or_default()
    //         //     .into_iter()
    //         //     .map(Value::Address)
    //         //     .collect();
    //         //     // if targets.len() >= 300 {
    //         //     //     println!("offsetjr {:?}", address);
    //         //     // }
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
    //             // Switch { ty: i_64, val: _1, dflt: default, tgts: targets },
    //         Ret { ty: i_64, val: _1 },
    //         }
    //     }
    //     PseudoJalr { rs1 } => {
    //         // let default = Value::Address(Address(0x1));
    //         // let targets: Vec<_> = self
    //         //     .inter_targets
    //         //     .clone()
    //         //     .into_iter()
    //         //     .map(Value::Address)
    //         //     .collect();

    //         //     // if targets.len() >= 300 {
    //         //     //     println!("PseudoJalr {:?}", address);
    //         //     // }
    //         let imm = &Immediate(0);
    //         let rs2 = &Register::Ra;
    //         {
    //             Store { ty: i_64, val: next_pc, ptr: rs2 },
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             Add { rslt: _1, ty: i_64, op1: _0, op2: imm },
    //             // Switch { ty: i_64, val: _1, dflt: default, tgts: targets },
    //         Ret { ty: i_64, val: _1 },
    //         }
    //     }
    //     Ret { address, raw } => {
    //         // let default = Value::Address(Address(0x1));
    //         // let mut targets: Vec<_> = self
    //         //     .ret_targets.get(&func).cloned().unwrap_or_default();
    //         // let rel_targets: Vec<_> = self
    //         //     .rel_ret_targets
    //         //     .clone();
    //         // targets.extend(rel_targets);

    //         // self.inherit(&func, &mut targets);

    //         // targets.sort_unstable();
    //         // targets.dedup();
    //         // let targets:Vec<_> =     targets.into_iter()
    //         //     .map(Value::Address)
    //         //     .collect();

    //         let rs1 = &Register::Ra;
    //         {
    //             Load { rslt: _0, ty: i_64, ptr: rs1 },
    //             // Switch { ty: i_64, val: _0, dflt: default, tgts: targets },
    //         Ret { ty: i_64, val: _0 },
    //         }
    //     }

    //     PseudoFence { address, raw } => {
    //         Fence { ord: acq_rel },
    //         
    //     },

    //     // Misc
    //     Unimp { address, .. } => Vec::new(), // panic!("Encounter `unimp` at `{}`", address),
    //     Unknown { address, .. } => Vec::new(), // panic!("Encounter `unknown` at `{}`", address),


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

#[cfg(test)]
mod tests {
    use super::Translator;
    use crate::llvm_isa::{CodeBlock, Instruction, InstructionBlock, Type, Value};
    use crate::riscv_isa::{
        Abi, Address, CodeBlock as RiscvCodeBlock, DataBlock, Immediate,
        Instruction as RiscvInstruction, Program as RiscvProgram, Raw, Register,
    };
    use std::fs;

    macro_rules! build_tests {
        ( $( $func:ident ( $rv_inst:ident { $( $field:ident: $value:expr ),* }, $( $inst:literal, )* ), )* ) => {
            $(
                #[test]
                fn $func() {
                    use RiscvInstruction as RI;

                    let rv_program = RiscvProgram {
                        abi: Abi::Lp64d,
                        data_blocks: Vec::new(),
                        code_blocks: vec![RiscvCodeBlock {
                            section: String::from(".text"),
                            symbol: String::from("_start"),
                            address: Address(0x0),
                            instructions: vec![
                                $rv_inst {
                                    address: Address(0x0),
                                    raw: Raw::new(""),
                                    $(
                                        $field: $value,
                                    )*
                                },
                            ],
                        }],
                    };
                    let program = Translator::new().run(rv_program);
                    let inst_strs: Vec<_> = program.code_blocks[0].instruction_blocks[0]
                        .instructions
                        .iter()
                        .map(|i| i.to_string())
                        .collect();
                    assert_eq!(inst_strs, vec![$($inst, )*]);
                }
            )*
        };
    }

    #[test]
    fn enforce_zero() {
        use Instruction::*;
        use RiscvInstruction as RI;

        let rv_program = RiscvProgram {
            abi: Abi::Lp64d,
            data_blocks: Vec::new(),
            code_blocks: vec![RiscvCodeBlock {
                section: String::from(".text"),
                symbol: String::from("_start"),
                address: Address(0x0),
                instructions: vec![Addi {
                    address: Address(0x0),
                    raw: Raw::new(""),
                    rd: Register::Zero,
                    rs1: Register::Zero,
                    imm: Immediate(0),
                }],
            }],
        };
        let program = Translator::new().run(rv_program);
        assert_eq!(
            program.code_blocks,
            vec![CodeBlock {
                section: String::from(".text"),
                symbol: String::from("_start"),
                address: Address(0x0),
                instruction_blocks: vec![InstructionBlock {
                    riscv_instruction: Addi {
                        address: Address(0x0),
                        raw: Raw::new(""),
                        rd: Register::Zero,
                        rs1: Register::Zero,
                        imm: Immediate(0)
                    },
                    instructions: vec![
                        Add {
                            rslt: Value::Temp(Address(0x0), 0),
                            ty: Type::I64,
                            op1: Value::Immediate(Immediate(0)),
                            op2: Value::Immediate(Immediate(0))
                        },
                        Add {
                            rslt: Value::Temp(Address(0x0), 1),
                            ty: Type::I64,
                            op1: Value::Temp(Address(0x0), 0),
                            op2: Value::Immediate(Immediate(0))
                        },
                    ],
                }],
            }]
        );
    }

    build_tests! {
        lui(Lui { rd: Register::T0, imm: Immediate(4) },
            "%t_0_0 = shl i64 4, 12",
            "store i64 %t_0_0, i64* %t0",
        ),
    }
}
