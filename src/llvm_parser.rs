use crate::riscv_isa::{Address, Program};
use llvm_sys::bit_reader::LLVMParseBitcode2;
use llvm_sys::core::*;
use llvm_sys::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

fn to_c_str(s: &'static str) -> *const c_char {
    CString::new(s).unwrap().as_c_str().as_ptr()
}

fn from_c_str(s: *const c_char) -> String {
    let mut bytes = Vec::new();
    let mut i = 0;
    loop {
        let byte = unsafe { *s.offset(i as isize) as u8 };
        if byte == 0 {
            break;
        } else {
            bytes.push(byte);
            i += 1;
        }
    }
    CString::new(bytes).unwrap().into_string().unwrap()
}

pub fn parse(ir: &Vec<u8>, rv_prog: &Program) -> (HashMap<Address, String>, String) {
    unsafe {
        let ir_mem_buf = LLVMCreateMemoryBufferWithMemoryRange(
            ir.as_ptr() as *const i8,
            ir.len(),
            to_c_str("IR Mem Buf"),
            1,
        );
        let mut ir_mod = LLVMModuleCreateWithName(to_c_str("IR Mod"));
        let err = LLVMParseBitcode2(ir_mem_buf, &mut ir_mod);
        if err != 0 {
            panic!("Fail to parse LLVM IR")
        }

        let mut func_table = HashMap::new();
        let mut func_mod = String::new();

        let mut func = LLVMGetFirstFunction(ir_mod);
        while {
            let mut func_name_len = 0;
            let func_name = LLVMGetValueName2(func, &mut func_name_len);
            let func_name = from_c_str(func_name);
            let mut addr = Address(0);
            for func in &rv_prog.code_blocks {
                if func.symbol == func_name {
                    addr = func.address;
                    break;
                }
            }
            if let Address(0) = addr {
            } else {
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
                                        let inst_s = from_c_str(LLVMPrintValueToString(inst));
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
                                let inst_s = from_c_str(LLVMPrintValueToString(inst));
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
                                        let val_ty =
                                            from_c_str(LLVMPrintTypeToString(LLVMTypeOf(val)));
                                        let val_s = {
                                            let s = from_c_str(LLVMPrintValueToString(val));
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
                                        let dest_name =
                                            from_c_str(LLVMGetValueName2(dest, &mut len));
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
                                    }
                                    LLVMOpcode::LLVMLoad => {
                                        let i =
                                            var_vec.iter().position(|i| i == &Some(inst)).unwrap();
                                        let ty =
                                            from_c_str(LLVMPrintTypeToString(LLVMTypeOf(inst)));
                                        let src = LLVMGetOperand(inst, 0);
                                        let mut len = 0;
                                        let src_name = from_c_str(LLVMGetValueName2(src, &mut len));
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
                                let inst_s = from_c_str(LLVMPrintValueToString(inst));
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
                                        let val_ty =
                                            from_c_str(LLVMPrintTypeToString(LLVMTypeOf(val)));
                                        let val_s = {
                                            let s = from_c_str(LLVMPrintValueToString(val));
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
                                        let dest_name =
                                            from_c_str(LLVMGetValueName2(dest, &mut len));
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
                                            let s = from_c_str(LLVMPrintValueToString(dest));
                                            let dest_name =
                                                s.trim().split(' ').collect::<Vec<_>>()[1];
                                            func_s.push_str(&format!(
                                                "  store {val_s}, {val_ty}* {dest_name}"
                                            ));
                                        }
                                    }
                                    LLVMOpcode::LLVMLoad => {
                                        let i =
                                            var_vec.iter().position(|i| i == &Some(inst)).unwrap();
                                        let ty =
                                            from_c_str(LLVMPrintTypeToString(LLVMTypeOf(inst)));
                                        let src = LLVMGetOperand(inst, 0);
                                        let mut len = 0;
                                        let src_name = from_c_str(LLVMGetValueName2(src, &mut len));
                                        if !src_name.is_empty() {
                                            let src_i = rv_prog
                                                .data_blocks
                                                .iter()
                                                .position(|b| b.symbol == src_name)
                                                .unwrap();
                                            let Address(src_addr) =
                                                rv_prog.data_blocks[src_i].address;
                                            let data_len = 141912; // Fix me
                                            func_s.push_str(&format!(
                                                "  %ptr_{t} = getelementptr [{data_len} x i8], [{data_len} x i8]* @data_0, i64 0, i64 {src_addr}
      %cast_ptr_{t} = bitcast i8* %ptr_{t} to {ty}*
      %{i} = load {ty}, {ty}* %cast_ptr_{t}
    "
                                            ));
                                            t += 1;
                                        } else {
                                            let s = from_c_str(LLVMPrintValueToString(src));
                                            let src_name =
                                                s.trim().split(' ').collect::<Vec<_>>()[1];
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
                    func_table.insert(addr, func_name);
                    func_mod.push_str(&func_s);
                }
            }

            func = LLVMGetNextFunction(func);
            !func.is_null()
        } {}

        (func_table, func_mod)
    }
}
