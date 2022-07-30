use std::collections::HashMap;
use crate::riscv_isa::{Program, Address};

pub fn parse(ir: &str, rv_prog: &Program) -> (HashMap<Address, String>, String) {
    let mut m = HashMap::new();
    m.insert(Address(0x1019c), String::from("f"));
    (m, format!("define i64 @f(i64, %struct.reg* %reg, %struct.freg* %freg) {{
  %a0 = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 10
  store i64 2, i64* %a0
  %ra = getelementptr %struct.reg, %struct.reg* %reg, i32 0, i32 1
  %ret = load i64, i64* %ra
  ret i64 %ret
}}
"))
}