mod x86_64;

use indoc::formatdoc;

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
        .map(|(name, _, _)| formatdoc!("
            sys_{name}:
              %sys_{name}_rslt = call i64 (i64, i64, i64, i64, i64, i64) @.sys_{name}(i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)
              store i64 %sys_{name}_rslt, ptr %raw_rslt_ptr
              br label %mod_errno"))
        .collect::<Vec<_>>()
        .join("\n");
    let funcs = defs
        .iter()
        .map(|(name, _, func)| formatdoc!("
            define i64 @.sys_{name}(i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) alwaysinline {{
            {func}
              ret i64 %rslt
            }}"))
        .collect::<Vec<_>>()
        .join("\n\n");
    format!(
        include_str!("templates/sys_call.ll"),
        aux = aux,
        disp = disp,
        callers = callers,
        funcs = funcs
    )
}
