pub fn run(arch: &str) -> String {
    let (structs, syscalls) = match arch {
        "x86_64" => (
            crate::syscall_x86_64::STRUCTS,
            &crate::syscall_x86_64::SYSCALLS,
        ),
        _ => panic!("Unknown arch `{arch}`"),
    };
    let dispatcher = syscalls
        .iter()
        .map(|(name, nr, _)| format!("    i64 {nr}, label %SYS_{name}"))
        .collect::<Vec<_>>()
        .join("\n");
    let callers = syscalls
        .iter()
        .map(|(name, _, _)| format!("SYS_{name}:
  %SYS_{name}_rslt = call i64 (i64, i64, i64, i64, i64, i64) @.SYS_{name}(i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)
  store i64 %SYS_{name}_rslt, i64* %raw_rslt_p
  br label %mod_errno"))
        .collect::<Vec<_>>()
        .join("\n");
    let funcs = syscalls
        .iter()
        .map(|(name, _, func)| format!("define i64 @.SYS_{name}(i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) alwaysinline {{
{func}
  ret i64 %rslt
}}"))
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("declare i64 @syscall(i64, ...)
declare i32* @__errno_location()

{structs}

define i64 @.system_call(i64 %nr, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) {{
  %raw_rslt_p = alloca i64
  %errno_p = call i32* @__errno_location()
  %errno = load i32, i32* %errno_p

  switch i64 %nr, label %fallback [
{dispatcher}
  ]

{callers}

mod_errno:
  %new_errno = load i32, i32* %errno_p
  store i32 %errno, i32* %errno_p
  %_err_rslt = sub i32 0, %new_errno
  %err_rslt = sext i32 %_err_rslt to i64
  %raw_rslt = load i64, i64* %raw_rslt_p
  %is_err = icmp eq i64 %raw_rslt, -1
  %rslt = select i1 %is_err, i64 %err_rslt, i64 %raw_rslt
  ret i64 %rslt

fallback:
  ret i64 -1
}}

{funcs}")
}
