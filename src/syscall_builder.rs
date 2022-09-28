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
  ret i64 %SYS_{name}_rslt"))
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

{structs}

define i64 @.system_call(i64 %nr, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) {{
  switch i64 %nr, label %fallback [
{dispatcher}
  ]

{callers}

fallback:
  ret i64 -1
}}

{funcs}")
}
