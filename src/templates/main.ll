{func_decls}

define i32 @main(i32 %argc, ptr %argv, ptr %envp) {{
  ; Initialize the stack pointer.
  store i64 {sp}, ptr @.sp

  ; Initialize `argc`.
  %argc_dest = call ptr @.get_mem_ptr(i64 {sp})
  store i32 %argc, ptr %argc_dest

  ; Initialize `argv`.
  %argv_addr = add i64 {sp}, 8
  %argv_dest = call ptr @.get_mem_ptr(i64 %argv_addr)
  %argc_i64 = sext i32 %argc to i64
  %argv_byte_cnt = mul i64 %argc_i64, 8
  call void @.mem_copy(ptr %argv_dest, ptr %argv, i64 %argv_byte_cnt)

  ; Initialize `envp`.
  %argv_offset = add i64 %argv_byte_cnt, 8
  %envp_addr = add i64 %argv_addr, %argv_offset
  %envp_dest = call ptr @.get_mem_ptr(i64 %envp_addr)
  %auxv = call ptr @.copy_envp(ptr %envp, ptr %envp_dest)

  ; Initialize `auxv`.
  %phdr = call ptr @.get_mem_ptr(i64 {phdr})
  call void @.init_auxv(ptr %auxv, ptr %phdr, i64 {phdr}, i64 {tdata_addr}, i64 {tdata_len})

  ; Load the entry address.
  %entry_ptr= alloca i64
  store i64 {entry}, ptr %entry_ptr
  br label %loop

loop:
  %entry = load i64, ptr %entry_ptr
  %func_addr_ptr = getelementptr [{disp_len} x i64], ptr @.disp, i64 0, i64 %entry
  %func_addr = load i64, ptr %func_addr_ptr
  %func = inttoptr i64 %func_addr to ptr
  %next = call i64 %func(i64 %entry)
  store i64 %next, ptr %entry_ptr
  br label %loop
}}

{native_mem_utils}

{mem}

{disps}

{rounding_funcs}

{defs}

{sys_call}
