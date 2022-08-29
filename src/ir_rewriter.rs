use crate::riscv_isa::{Address, Program};
use llvm_sys::bit_reader::LLVMParseBitcode2;
use llvm_sys::core::*;
use llvm_sys::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

// let (mut parsed_funcs, mut parsed_irs) = (HashMap::new(), Vec::new());
// for ir in irs {
//     let (funcs, ir) = llvm_parser::parse(ir, &rv_program);
//     parsed_funcs.extend(funcs);
//     parsed_irs.push(ir);
// }

pub struct Rewriter {

}

impl Rewriter {
    pub fn new(jobs: usize) -> Self {
        Rewriter {}
    }
    pub fn run(&self, rv_prog: &Program, ir: &Vec<Vec<u8>>) -> HashMap<Address, String> {
        HashMap::new()
    }
}
pub fn parse(ir: &Vec<u8>, rv_prog: &Program) -> (HashMap<Address, String>, String) {
    unsafe {
        let memory_buffer =
            LLVMCreateMemoryBufferWithMemoryRange(ir.as_ptr() as *const i8, ir.len(), null(), 1);
        let mut module = LLVMModuleCreateWithName(null());
        if LLVMParseBitcode2(memory_buffer, &mut module) != 0 {
            panic!("Fail to parse LLVM bitcode");
        }

        let mut func_table = HashMap::new();
        let mut func_src = String::new();

        let mut func = LLVMGetFirstFunction(module);
        loop {
            let func_name = to_string(LLVMGetValueName2(func, &mut 0));
            let func_addr = match rv_prog
                .code_blocks
                .iter()
                .position(|b| b.symbol == func_name)
            {
                Some(i) => rv_prog.code_blocks[i].address,
                None => continue,
            };
            let func_ty = LLVMGetElementType(LLVMTypeOf(func));

            let mut var_vec = Vec::new();
            let mut t = 0;

            // Parameter types
            let param_count = LLVMCountParamTypes(func_ty);
            let mut param_tys = vec![LLVMVoidType(); param_count as usize];
            LLVMGetParamTypes(func_ty, param_tys.as_mut_ptr());
            let param_kinds: Vec<_> = param_tys.iter().map(|ty| LLVMGetTypeKind(*ty)).collect();

            // Return type
            let ret_ty = LLVMGetReturnType(func_ty);
            let ret_kind = LLVMGetTypeKind(ret_ty);

            let func_s = match (ret_kind, param_count, &param_kinds[..]) {
                (LLVMTypeKind::LLVMIntegerTypeKind, 1, [LLVMTypeKind::LLVMIntegerTypeKind]) => {
                    match (
                        LLVMGetIntTypeWidth(ret_ty),
                        LLVMGetIntTypeWidth(param_tys[0]),
                    ) {
                        (32, 32) => {
                            var_vec.push(None);
                            let mut func_s = format!("define i64 @{func_name}(i64 %target, %struct.reg* %reg, %struct.freg* %freg) {{
entry:
%arg_1_ptr = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 10
%arg_1 = load i64, i64* %arg_1_ptr
%0 = trunc i64 %arg_1 to i32
br label %1
");
                            let mut block = LLVMGetFirstBasicBlock(func);
                            while {
                                func_s += &format!("{}:\n", var_vec.len());
                                var_vec.push(None);
                                let mut inst = LLVMGetFirstInstruction(block);
                                while {
                                    let inst_s = to_string(LLVMPrintValueToString(inst));
                                    if inst_s.starts_with("  %") {
                                        var_vec.push(Some(inst));
                                    }
                                    match LLVMGetInstructionOpcode(inst) {
                                        LLVMOpcode::LLVMRet => {
                                            let ret = LLVMGetOperand(inst, 0);
                                            let i = var_vec
                                                .iter()
                                                .position(|i| i == &Some(ret))
                                                .unwrap();
                                            func_s.push_str(&format!(
                                                "  %i64_{i} = sext i32 %{i} to i64
%a0_{t} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 10
store i64 %i64_{i}, i64* %a0_{t}
%ra_{t} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 1
%ret_{t} = load i64, i64* %ra_{t}
ret i64 %ret_{t}"
                                            ));
                                            t += 1;
                                        }
                                        _ => func_s.push_str(&inst_s),
                                    }
                                    func_s.push('\n');
                                    inst = LLVMGetNextInstruction(inst);
                                    !inst.is_null()
                                } {}
                                block = LLVMGetNextBasicBlock(block);
                                !block.is_null()
                            } {}
                            func_s.push_str("}\n");
                            Some(func_s)
                        }
                        _ => None,
                    }
                }
                (LLVMTypeKind::LLVMVoidTypeKind, 0, []) => {
                    let mut func_s = format!("define i64 @{func_name}(i64 %target, %struct.reg* %reg, %struct.freg* %freg) {{
entry:
br label %0
");
                    let mut block = LLVMGetFirstBasicBlock(func);
                    while {
                        func_s += &format!("{}:\n", var_vec.len());
                        var_vec.push(None);
                        let mut inst = LLVMGetFirstInstruction(block);
                        while {
                            let inst_s = to_string(LLVMPrintValueToString(inst));
                            if inst_s.starts_with("  %") {
                                var_vec.push(Some(inst));
                            }
                            match LLVMGetInstructionOpcode(inst) {
                                LLVMOpcode::LLVMRet => {
                                    func_s.push_str(&format!(
                                        "  %ra_{t} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 1
%ret_{t} = load i64, i64* %ra_{t}
ret i64 %ret_{t}"
                                    ));
                                    t += 1;
                                }
                                LLVMOpcode::LLVMStore => {
                                    let val = LLVMGetOperand(inst, 0);
                                    let val_ty = to_string(LLVMPrintTypeToString(LLVMTypeOf(val)));
                                    let val_s = {
                                        let s = to_string(LLVMPrintValueToString(val));
                                        if s.starts_with("  %") {
                                            format!(
                                                "{val_ty} {}",
                                                s.trim().split(' ').next().unwrap()
                                            )
                                        } else {
                                            s
                                        }
                                    };
                                    let dest = LLVMGetOperand(inst, 1);
                                    let mut len = 0;
                                    let dest_name = to_string(LLVMGetValueName2(dest, &mut len));
                                    let dest_i = rv_prog
                                        .data_blocks
                                        .iter()
                                        .position(|b| b.symbol == dest_name)
                                        .unwrap();
                                    let Address(dest_addr) = rv_prog.data_blocks[dest_i].address;
                                    let data_len = 141912; // Fix me
                                    func_s.push_str(&format!(
                                        "  %ptr_{t} = getelementptr [{data_len} x i8], [{data_len} x i8]* @data_0, i64 0, i64 {dest_addr}
%cast_ptr_{t} = bitcast i8* %ptr_{t} to {val_ty}*
store {val_s}, {val_ty}* %cast_ptr_{t}
"
                                    ));
                                    t += 1;
                                }
                                LLVMOpcode::LLVMLoad => {
                                    let i = var_vec.iter().position(|i| i == &Some(inst)).unwrap();
                                    let ty = to_string(LLVMPrintTypeToString(LLVMTypeOf(inst)));
                                    let src = LLVMGetOperand(inst, 0);
                                    let mut len = 0;
                                    let src_name = to_string(LLVMGetValueName2(src, &mut len));
                                    let src_i = rv_prog
                                        .data_blocks
                                        .iter()
                                        .position(|b| b.symbol == src_name)
                                        .unwrap();
                                    let Address(src_addr) = rv_prog.data_blocks[src_i].address;
                                    let data_len = 141912; // Fix me
                                    func_s.push_str(&format!(
                                        "  %ptr_{t} = getelementptr [{data_len} x i8], [{data_len} x i8]* @data_0, i64 0, i64 {src_addr}
%cast_ptr_{t} = bitcast i8* %ptr_{t} to {ty}*
%{i} = load {ty}, {ty}* %cast_ptr_{t}
"
                                    ));
                                    t += 1;
                                }
                                _ => func_s.push_str(&inst_s),
                            }
                            func_s.push('\n');
                            inst = LLVMGetNextInstruction(inst);
                            !inst.is_null()
                        } {}
                        block = LLVMGetNextBasicBlock(block);
                        !block.is_null()
                    } {}
                    func_s.push_str("}\n");
                    Some(func_s)
                }
                (LLVMTypeKind::LLVMVoidTypeKind, 1, [LLVMTypeKind::LLVMPointerTypeKind]) => {
                    var_vec.push(None);
                    let data_len = 141912; // Fix me
                    let mut func_s = format!("define i64 @{func_name}(i64 %target, %struct.reg* %reg, %struct.freg* %freg) {{
entry:
%arg_1_ptr = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 10
%arg_1 = load i64, i64* %arg_1_ptr
%host_ptr_1 = getelementptr [{data_len} x i8], [{data_len} x i8]* @data_0, i64 0, i64 %arg_1
%0 = bitcast i8* %host_ptr_1 to i32*
br label %1
");
                    let mut block = LLVMGetFirstBasicBlock(func);
                    while {
                        func_s += &format!("{}:\n", var_vec.len());
                        var_vec.push(None);
                        let mut inst = LLVMGetFirstInstruction(block);
                        while {
                            let inst_s = to_string(LLVMPrintValueToString(inst));
                            if inst_s.starts_with("  %") {
                                var_vec.push(Some(inst));
                            }
                            match LLVMGetInstructionOpcode(inst) {
                                LLVMOpcode::LLVMRet => {
                                    func_s.push_str(&format!(
                                        "  %ra_{t} = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 1
%ret_{t} = load i64, i64* %ra_{t}
ret i64 %ret_{t}"
                                    ));
                                    t += 1;
                                }
                                LLVMOpcode::LLVMStore => {
                                    let val = LLVMGetOperand(inst, 0);
                                    let val_ty = to_string(LLVMPrintTypeToString(LLVMTypeOf(val)));
                                    let val_s = {
                                        let s = to_string(LLVMPrintValueToString(val));
                                        if s.starts_with("  %") {
                                            format!(
                                                "{val_ty} {}",
                                                s.trim().split(' ').next().unwrap()
                                            )
                                        } else {
                                            s
                                        }
                                    };
                                    let dest = LLVMGetOperand(inst, 1);
                                    let mut len = 0;
                                    let dest_name = to_string(LLVMGetValueName2(dest, &mut len));
                                    if !dest_name.is_empty() {
                                        let dest_i = rv_prog
                                            .data_blocks
                                            .iter()
                                            .position(|b| b.symbol == dest_name)
                                            .unwrap();
                                        let Address(dest_addr) =
                                            rv_prog.data_blocks[dest_i].address;
                                        let data_len = 141912; // Fix me
                                        func_s.push_str(&format!(
                                        "  %ptr_{t} = getelementptr [{data_len} x i8], [{data_len} x i8]* @data_0, i64 0, i64 {dest_addr}
%cast_ptr_{t} = bitcast i8* %ptr_{t} to {val_ty}*
store {val_s}, {val_ty}* %cast_ptr_{t}
"
                                    ));
                                        t += 1;
                                    } else {
                                        let s = to_string(LLVMPrintValueToString(dest));
                                        let dest_name = s.trim().split(' ').collect::<Vec<_>>()[1];
                                        func_s.push_str(&format!(
                                            "  store {val_s}, {val_ty}* {dest_name}"
                                        ));
                                    }
                                }
                                LLVMOpcode::LLVMLoad => {
                                    let i = var_vec.iter().position(|i| i == &Some(inst)).unwrap();
                                    let ty = to_string(LLVMPrintTypeToString(LLVMTypeOf(inst)));
                                    let src = LLVMGetOperand(inst, 0);
                                    let mut len = 0;
                                    let src_name = to_string(LLVMGetValueName2(src, &mut len));
                                    if !src_name.is_empty() {
                                        let src_i = rv_prog
                                            .data_blocks
                                            .iter()
                                            .position(|b| b.symbol == src_name)
                                            .unwrap();
                                        let Address(src_addr) = rv_prog.data_blocks[src_i].address;
                                        let data_len = 141912; // Fix me
                                        func_s.push_str(&format!(
                                            "  %ptr_{t} = getelementptr [{data_len} x i8], [{data_len} x i8]* @data_0, i64 0, i64 {src_addr}
    %cast_ptr_{t} = bitcast i8* %ptr_{t} to {ty}*
    %{i} = load {ty}, {ty}* %cast_ptr_{t}
"
                                        ));
                                        t += 1;
                                    } else {
                                        let s = to_string(LLVMPrintValueToString(src));
                                        let src_name = s.trim().split(' ').collect::<Vec<_>>()[1];
                                        func_s.push_str(&format!(
                                            "  %{i} = load {ty}, {ty}* {src_name}"
                                        ));
                                    }
                                }
                                _ => func_s.push_str(&inst_s),
                            }
                            func_s.push('\n');
                            inst = LLVMGetNextInstruction(inst);
                            !inst.is_null()
                        } {}
                        block = LLVMGetNextBasicBlock(block);
                        !block.is_null()
                    } {}
                    func_s.push_str("}\n");
                    Some(func_s)
                }
                _ => None,
            };

            if let Some(func_s) = func_s {
                func_table.insert(func_addr, func_name);
                func_src.push_str(&func_s);
            }

            func = LLVMGetNextFunction(func);
            if func.is_null() {
                break;
            }
        }

        (func_table, func_src)
    }
}

unsafe fn to_string(c_str: *const c_char) -> String {
    let mut bytes = Vec::new();
    let mut i = 0;
    loop {
        match *c_str.offset(i) as u8 {
            0 => break,
            byte => {
                bytes.push(byte);
                i += 1;
            }
        }
    }
    CString::new(bytes).unwrap().into_string().unwrap()
}
