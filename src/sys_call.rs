mod x86_64;

pub fn build(arch: Option<String>) -> Option<String> {
    let (aux, defs) = match arch.as_deref() {
        Some("x86_64") => (x86_64::AUX, &x86_64::DEFS),
        None => return None,
        _ => panic!("Unknown architecture `{arch:?}`"),
    };

    let dispatcher = defs
        .iter()
        .map(|(name, nr, _)| format!("    i64 {nr}, label %sys_{name}"))
        .collect::<Vec<_>>()
        .join("\n");
    let callers = defs
        .iter()
        .map(|(name, _, _)| format!("sys_{name}:
  %sys_{name}_rslt = call i64 (i64, i64, i64, i64, i64, i64) @.sys_{name}(i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)
  store i64 %sys_{name}_rslt, i64* %raw_rslt_p
  br label %mod_errno"))
        .collect::<Vec<_>>()
        .join("\n");
    let funcs = defs
        .iter()
        .map(|(name, _, func)| format!("define i64 @.sys_{name}(i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) alwaysinline {{
{func}
  ret i64 %rslt
}}"))
        .collect::<Vec<_>>()
        .join("\n\n");
    Some(format!(
        "declare i64 @syscall(i64, ...)
declare i32* @__errno_location()

define i8* @.sys_get_mem_ptr(i64 %addr) {{
  %is_zero = icmp eq i64 0, %addr
  br i1 %is_zero, label %dynamic, label %test_static
test_static:
  %ptr = call i8* @.get_mem_ptr(i64 %addr)  
  ret i8* %ptr
dynamic:
  %dynamic_ptr = inttoptr i64 %addr to i8*
  ret i8* %dynamic_ptr
}}

{aux}

define i64 @.sys_call(i64 %nr, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) {{
  %raw_rslt_p = alloca i64
  %errno_p = call i32* @__errno_location()
  %errno = load i32, i32* %errno_p

  switch i64 %nr, label %not_found [
{dispatcher}
  ]

{callers}

mod_errno:
  %new_errno = load i32, i32* %errno_p
  store i32 %errno, i32* %errno_p
  %err_rslt_i32 = sub i32 0, %new_errno
  %err_rslt = sext i32 %err_rslt_i32 to i64
  %raw_rslt = load i64, i64* %raw_rslt_p
  %is_err = icmp eq i64 %raw_rslt, -1
  %rslt = select i1 %is_err, i64 %err_rslt, i64 %raw_rslt
  ret i64 %rslt

not_found:
  ret i64 -1
}}

{funcs}"
    ))
}
