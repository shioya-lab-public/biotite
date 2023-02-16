use once_cell::sync::Lazy;

pub const AUX: &str = "%.sys_dirent = type { i64, i64, i16, i8, i8* }
%.sys_iovec = type { i8*, i64 }
%.sys_robust_list_head = type { i8*, i64, i8* }
%.sys_sigaction = type { i8*, i8*, i32, i8* }
%.sys_stat_x86_64 = type { i64, i64, i64, i32, i32, i32, i32, i64 }
%.sys_stat_riscv64gc = type { i64, i64, i32, i32, i32, i32, i64 }

define void @.sys_conv_stat(i8* %statbuf_b_x86_64) {
  %statbuf_x86_64 = bitcast i8* %statbuf_b_x86_64 to %.sys_stat_x86_64*
  %stat_x86_64 = load %.sys_stat_x86_64, %.sys_stat_x86_64* %statbuf_x86_64
  %st_nlink_i64 = extractvalue %.sys_stat_x86_64 %stat_x86_64, 2
  %st_nlink = trunc i64 %st_nlink_i64 to i32
  %st_mode = extractvalue %.sys_stat_x86_64 %stat_x86_64, 3
  %st_uid = extractvalue %.sys_stat_x86_64 %stat_x86_64, 4
  %st_gid = extractvalue %.sys_stat_x86_64 %stat_x86_64, 5
  %st_rdev = extractvalue %.sys_stat_x86_64 %stat_x86_64, 7

  %statbuf_riscv64gc = bitcast i8* %statbuf_b_x86_64 to %.sys_stat_riscv64gc*
  %stat_riscv64gc = load %.sys_stat_riscv64gc, %.sys_stat_riscv64gc* %statbuf_riscv64gc
  %stat_riscv64gc_1 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc, i32 %st_mode, 2
  %stat_riscv64gc_2 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_1, i32 %st_nlink, 3
  %stat_riscv64gc_3 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_2, i32 %st_uid, 4
  %stat_riscv64gc_4 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_3, i32 %st_gid, 5
  %stat_riscv64gc_5 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_4, i64 %st_rdev, 6
  store %.sys_stat_riscv64gc %stat_riscv64gc_5, %.sys_stat_riscv64gc* %statbuf_riscv64gc
  ret void
}";

pub static DEFS: Lazy<Vec<(&str, i32, &str)>> = Lazy::new(|| {
    vec![
        (
            "exit",
            93,
            "  %rslt = call i64 (i64, ...) @syscall(i64 60, i64 %arg1)"
        ),
        (
            "exit_group",
            94,
            "  %rslt = call i64 (i64, ...) @syscall(i64 231, i64 %arg1)"
        ),
        (
            "getpid",
            172,
            "  %rslt = call i64 (i64, ...) @syscall(i64 39)"
        ),
        (
            "kill",
            129,
            "  %rslt = call i64 (i64, ...) @syscall(i64 62, i64 %arg1, i64 %arg2)"
        ),
        (
            "tgkill",
            131,
            "  %rslt = call i64 (i64, ...) @syscall(i64 234, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "read",
            63,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 0, i64 %arg1, i8* %buf, i64 %arg3)"
        ),
        (
            "write",
            64,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 1, i64 %arg1, i8* %buf, i64 %arg3)"
        ),
        (
            "openat",
            56,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 257, i64 %arg1, i8* %filename, i64 %arg3, i64 %arg4)"
        ),
        (
            "close",
            57,
            "  %rslt = call i64 (i64, ...) @syscall(i64 3, i64 %arg1)"
        ),
        (
            "lseek",
            62,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "brk",
            214,
            "  %rslt = call i64 (i64, ...) @syscall(i64 12, i64 %arg1)"
        ),
        (
            "linkat",
            37,
            "  %oldname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %newname = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 265, i64 %arg1, i8* %oldname, i64 %arg3, i8* %newname, i64 %arg5)"
        ),
        (
            "unlinkat",
            35,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 263, i64 %arg1, i8* %pathname, i64 %arg3)"
        ),
        (
            "mkdirat",
            34,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 258, i64 %arg1, i8* %pathname, i64 %arg3)"
        ),
        (
            "renameat",
            38,
            "  %oldname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %newname = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 264, i64 %arg1, i8* %oldname, i64 %arg3, i8* %newname)"
                  ),
        (
            "chdir",
            49,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 80, i8* %filename)"
        ),
        (
            "getcwd",
            17,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 79, i8* %buf, i64 %arg2)"
        ),
        (
            "fstat",
            80,
            "  %pad_addr = add i64 %arg2, 120
  %pad_b = call i8* @.sys_get_mem_ptr(i64 %pad_addr)
  %pad_ptr = bitcast i8* %pad_b to i192*
  %pad = load i192, i192* %pad_ptr
  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 5, i64 %arg1, i8* %statbuf)
  call void @.sys_conv_stat(i8* %statbuf)
  store i192 %pad, i192* %pad_ptr"
        ),
        (
            "fstatat",
            79,
            "  %pad_addr = add i64 %arg3, 120
  %pad_b = call i8* @.sys_get_mem_ptr(i64 %pad_addr)
  %pad_ptr = bitcast i8* %pad_b to i192*
  %pad = load i192, i192* %pad_ptr
  %filename = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 262, i64 %arg1, i8* %filename, i8* %statbuf, i64 %arg4)
  call void @.sys_conv_stat(i8* %statbuf)
  store i192 %pad, i192* %pad_ptr"
        ),
        (
            "faccessat",
            48,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 269, i64 %arg1, i8* %filename, i64 %arg3)"
        ),
        (
            "pread",
            67,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 17, i64 %arg1, i8* %buf, i64 %arg3, i64 %arg4)"
        ),
        (
            "pwrite",
            68,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 18, i64 %arg1, i8* %buf, i64 %arg3, i64 %arg4)"
        ),
        (
            "uname",
            160,
            "  %utsname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 63, i8* %utsname)"
        ),
        (
            "getuid",
            174,
            "  %rslt = call i64 (i64, ...) @syscall(i64 102)"
        ),
        (
            "geteuid",
            175,
            "  %rslt = call i64 (i64, ...) @syscall(i64 107)"
        ),
        (
            "getgid",
            176,
            "  %rslt = call i64 (i64, ...) @syscall(i64 104)"
        ),
        (
            "getegid",
            177,
            "  %rslt = call i64 (i64, ...) @syscall(i64 108)"
        ),
        (
            "gettid",
            178,
            "  %rslt = call i64 (i64, ...) @syscall(i64 186)"
        ),
        (
            "sysinfo",
            179,
            "  %sysinfo = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 99, i8* %sysinfo)"
        ),
        (
            "mmap",
            222,
            "  ; Ignore the address hint in `arg1`
  %rslt = call i64 (i64, ...) @syscall(i64 9, i64 0, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)"
        ),
        (
            "munmap",
            215,
            "  %addr = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 11, i8* %addr, i64 %arg2)"
        ),
        (
            "mremap",
            216,
            "  %addr = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %new_addr = call i8* @.sys_get_mem_ptr(i64 %arg5)
  %rslt = call i64 (i64, ...) @syscall(i64 25, i8* %addr, i64 %arg2, i64 %arg3, i64 %arg4, i8* %new_addr)"
        ),
        (
            "mprotect",
            226,
            "  ; Always return 0, because it fails for legal input in RISC-V
  %rslt = add i64 0, 0"
        ),
        (
            "prlimit64",
            261,
            "  %new_rlim = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %old_rlim = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 302, i64 %arg1, i64 %arg2, i8* %new_rlim, i8* %old_rlim)"
        ),
        (
            "getmainvars",
            2011,
            "  ; This system call is not available in x86_64
  %rslt = add i64 0, -1"
        ),
        (
            "rt_sigaction",
            134,
            "  %act_ptr_b = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %act_ptr = bitcast i8* %act_ptr_b to %.sys_sigaction*
  %act = load %.sys_sigaction, %.sys_sigaction* %act_ptr
  %field_0 = extractvalue %.sys_sigaction %act, 0
  %field_0_val = ptrtoint i8* %field_0 to i64
  %host_field_0 = call i8* @.sys_get_mem_ptr(i64 %field_0_val)
  %act_0 = insertvalue %.sys_sigaction %act, i8* %host_field_0, 0
  %field_1 = extractvalue %.sys_sigaction %act, 1
  %field_1_val = ptrtoint i8* %field_1 to i64
  %host_field_1 = call i8* @.sys_get_mem_ptr(i64 %field_1_val)
  %act_1 = insertvalue %.sys_sigaction %act_0, i8* %host_field_1, 1
  %field_3 = extractvalue %.sys_sigaction %act, 3
  %field_3_val = ptrtoint i8* %field_3 to i64
  %host_field_3 = call i8* @.sys_get_mem_ptr(i64 %field_3_val)
  %act_3 = insertvalue %.sys_sigaction %act_1, i8* %host_field_3, 3
  store %.sys_sigaction %act_3, %.sys_sigaction* %act_ptr
  %oldact = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 13, i64 %arg1, i8* %act_ptr_b, i8* %oldact, i64 %arg4)"
        ),
        (
            "writev",
            66,
            "  %vecs_b = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %vecs = bitcast i8* %vecs_b to %.sys_iovec*
  %i_ptr = alloca i64
  store i64 0, i64* %i_ptr
  br label %test
test:
  %i = load i64, i64* %i_ptr
  %cont = icmp slt i64 %i, %arg3
  br i1 %cont, label %trans, label %call
trans:
  %vec_ptr = getelementptr %.sys_iovec, %.sys_iovec* %vecs, i64 %i
  %vec = load %.sys_iovec, %.sys_iovec* %vec_ptr
  %field_0 = extractvalue %.sys_iovec %vec, 0
  %field_0_val = ptrtoint i8* %field_0 to i64
  %host_field_0 = call i8* @.sys_get_mem_ptr(i64 %field_0_val)
  %vec_0 = insertvalue %.sys_iovec %vec, i8* %host_field_0, 0
  store %.sys_iovec %vec_0, %.sys_iovec* %vec_ptr
  %new_i = add i64 %i, 1
  store i64 %new_i, i64* %i_ptr
  br label %test
call:
  %rslt = call i64 (i64, ...) @syscall(i64 20, i64 %arg1, i8* %vecs_b, i64 %arg3)"
        ),
        (
            "gettimeofday",
            169,
            "  %tv = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %tz = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 96, i8* %tv, i8* %tz)"
        ),
        (
            "times",
            153,
            "  %tbuf = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 100, i8* %tbuf)"
        ),
        (
            "fcntl",
            25,
            "  %rslt = call i64 (i64, ...) @syscall(i64 72, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "ftruncate",
            46,
            "  %rslt = call i64 (i64, ...) @syscall(i64 77, i64 %arg1, i64 %arg2)"
        ),
        (
            "getdents",
            61,
            "  %dirent_ptr_b = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %dirent_ptr = bitcast i8* %dirent_ptr_b to %.sys_dirent*
  %dirent = load %.sys_dirent, %.sys_dirent* %dirent_ptr
  %field_4 = extractvalue %.sys_dirent %dirent, 4
  %field_4_val = ptrtoint i8* %field_4 to i64
  %host_field_4 = call i8* @.sys_get_mem_ptr(i64 %field_4_val)
  %dirent_4 = insertvalue %.sys_dirent %dirent, i8* %host_field_4, 4
  store %.sys_dirent %dirent_4, %.sys_dirent* %dirent_ptr
  %rslt = call i64 (i64, ...) @syscall(i64 217, i64 %arg1, i8* %dirent_ptr_b, i64 %arg3)"
        ),
        (
            "dup",
            23,
            "  %rslt = call i64 (i64, ...) @syscall(i64 32, i64 %arg1)"
        ),
        (
            "dup3",
            24,
            "  %rslt = call i64 (i64, ...) @syscall(i64 292, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "readlinkat",
            78,
            "  %path = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %buf = call i8* @.sys_get_mem_ptr(i64 %arg3)
  ; Change the error value from -1 to -22, as RISC-V seems to assume this particular value
  %_rslt = call i64 (i64, ...) @syscall(i64 267, i64 %arg1, i8* %path, i8* %buf, i64 %arg4)
  %is_err = icmp eq i64 %_rslt, -1
  %rslt = select i1 %is_err, i64 -22, i64 %_rslt"
        ),
        (
            "rt_sigprocmask",
            135,
            "  %set = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %oset = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 14, i64 %arg1, i8* %set, i8* %oset, i64 %arg4)"
        ),
        (
            "ioctl",
            29,
            "  %rslt = call i64 (i64, ...) @syscall(i64 16, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "getrlimit",
            163,
            "  %rlim = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 97, i64 %arg1, i8* %rlim)"
        ),
        (
            "setrlimit",
            164,
            "  %rlim = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 160, i64 %arg1, i8* %rlim)"
        ),
        (
            "getrusage",
            165,
            "  %ru = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 98, i64 %arg1, i8* %ru)"
        ),
        (
            "clock_gettime",
            113,
            "  %tp = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 228, i64 %arg1, i8* %tp)"
        ),
        (
            "set_tid_address",
            96,
            "  %tidptr = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 218, i8* %tidptr)"
        ),
        (
            "set_robust_list",
            99,
            "  %head_ptr_b = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %head_ptr = bitcast i8* %head_ptr_b to %.sys_robust_list_head*
  %head = load %.sys_robust_list_head, %.sys_robust_list_head* %head_ptr
  %field_0 = extractvalue %.sys_robust_list_head %head, 0
  %field_0_val = ptrtoint i8* %field_0 to i64
  %host_field_0 = call i8* @.sys_get_mem_ptr(i64 %field_0_val)
  %head_0 = insertvalue %.sys_robust_list_head %head, i8* %host_field_0, 0
  %field_2 = extractvalue %.sys_robust_list_head %head, 2
  %field_2_val = ptrtoint i8* %field_2 to i64
  %host_field_2 = call i8* @.sys_get_mem_ptr(i64 %field_2_val)
  %head_2 = insertvalue %.sys_robust_list_head %head_0, i8* %host_field_2, 2
  store %.sys_robust_list_head %head_2, %.sys_robust_list_head* %head_ptr
  %rslt = call i64 (i64, ...) @syscall(i64 273, i8* %head_ptr_b, i64 %arg2)"
        ),
        (
            "madvise",
            233,
            "  %rslt = call i64 (i64, ...) @syscall(i64 28, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "statx",
            291,
            "  %path = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %buffer = call i8* @.sys_get_mem_ptr(i64 %arg5)
  %rslt = call i64 (i64, ...) @syscall(i64 332, i64 %arg1, i8* %path, i64 %arg3, i64 %arg4, i8* %buffer)"
        ),
        (
            "open",
            1024,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 2, i8* %filename, i64 %arg2, i64 %arg3)"
        ),
        (
            "link",
            1025,
            "  %oldname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %newname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 86, i8* %oldname, i8* %newname)"
        ),
        (
            "unlink",
            1026,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 87, i8* %pathname)"
        ),
        (
            "mkdir",
            1030,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 83, i8* %pathname, i64 %arg2)"
        ),
        (
            "access",
            1033,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 21, i8* %filename, i64 %arg2)"
        ),
        (
            "stat",
            1038,
            "  %pad_addr = add i64 %arg2, 120
  %pad_b = call i8* @.sys_get_mem_ptr(i64 %pad_addr)
  %pad_ptr = bitcast i8* %pad_b to i192*
  %pad = load i192, i192* %pad_ptr
  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 4, i8* %filename, i8* %statbuf)
  call void @.sys_conv_stat(i8* %statbuf)
  store i192 %pad, i192* %pad_ptr"
        ),
        (
            "lstat",
            1039,
            "  %pad_addr = add i64 %arg2, 120
  %pad_b = call i8* @.sys_get_mem_ptr(i64 %pad_addr)
  %pad_ptr = bitcast i8* %pad_b to i192*
  %pad = load i192, i192* %pad_ptr
  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 6, i8* %filename, i8* %statbuf)
  call void @.sys_conv_stat(i8* %statbuf)
  store i192 %pad, i192* %pad_ptr"
        ),
        (
            "time",
            1062,
            "  %tloc = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 201, i8* %tloc)"
        ),
    ]
});
