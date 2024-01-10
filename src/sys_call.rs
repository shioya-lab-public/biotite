mod x86_64;

pub fn run(arch: &str) -> String {
    let (aux, defs) = match arch {
        "x86_64" => (x86_64::AUX, x86_64::DEFS),
        arch => panic!("Unknown architecture `{arch}`"),
    };
    let disp = defs
        .iter()
        .map(|(name, nr, _)| format!("    i64 {nr}, label %sys_{name}"))
        .collect::<Vec<_>>()
        .join("\n");
    let callers = defs
        .iter()
        .map(|(name, _, _)| format!("sys_{name}:
  %sys_{name}_rslt = call i64 (i64, i64, i64, i64, i64, i64) @.sys_{name}(i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)
  store i64 %sys_{name}_rslt, ptr %raw_rslt_ptr
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
    format!(
        "declare i64 @syscall(i64, ...)
declare ptr @__errno_location()

{aux}

define i64 @.sys_call(i64 %nr, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) {{
  %raw_rslt_ptr = alloca i64
  %errno_ptr = call ptr @__errno_location()
  %errno = load i32, ptr %errno_ptr

  switch i64 %nr, label %not_found [
{disp}
  ]

{callers}

mod_errno:
  %new_errno = load i32, ptr %errno_ptr
  store i32 %errno, ptr %errno_ptr
  %err_rslt_i32 = sub i32 0, %new_errno
  %err_rslt = sext i32 %err_rslt_i32 to i64
  %raw_rslt = load i64, ptr %raw_rslt_ptr
  %is_err = icmp eq i64 %raw_rslt, -1
  %rslt = select i1 %is_err, i64 %err_rslt, i64 %raw_rslt
  ret i64 %rslt

not_found:
  ret i64 -1
}}

{funcs}"
    )
}
