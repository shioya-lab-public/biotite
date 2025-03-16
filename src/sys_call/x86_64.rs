//! System call implementation for x86_64.

/// Auxiliary things that will be parts of the output LLVM IR.
pub const AUX: &str = "%.sys_stat_x86_64 = type { i64, i64, i64, i32, i32, i32, i32, i64 }
%.sys_stat_riscv64gc = type { i64, i64, i32, i32, i32, i32, i64 }

define void @.sys_conv_stat(ptr %statbuf) alwaysinline {
  %stat_x86_64 = load %.sys_stat_x86_64, ptr %statbuf
  %st_nlink_i64 = extractvalue %.sys_stat_x86_64 %stat_x86_64, 2
  %st_nlink = trunc i64 %st_nlink_i64 to i32
  %st_mode = extractvalue %.sys_stat_x86_64 %stat_x86_64, 3
  %st_uid = extractvalue %.sys_stat_x86_64 %stat_x86_64, 4
  %st_gid = extractvalue %.sys_stat_x86_64 %stat_x86_64, 5
  %st_rdev = extractvalue %.sys_stat_x86_64 %stat_x86_64, 7

  %stat_riscv64gc = load %.sys_stat_riscv64gc, ptr %statbuf
  %stat_riscv64gc_1 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc, i32 %st_mode, 2
  %stat_riscv64gc_2 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_1, i32 %st_nlink, 3
  %stat_riscv64gc_3 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_2, i32 %st_uid, 4
  %stat_riscv64gc_4 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_3, i32 %st_gid, 5
  %stat_riscv64gc_5 = insertvalue %.sys_stat_riscv64gc %stat_riscv64gc_4, i64 %st_rdev, 6
  store %.sys_stat_riscv64gc %stat_riscv64gc_5, ptr %statbuf

  ret void
}";

/// Each system call is defined using three components:
/// - A unique name
/// - The RISC-V system call number
/// - The implementation (a LLVM IR snippet)
///
/// In the implementation, "@syscall" and `%arg1` to `%arg6` are defined,
/// and the return value of `@syscall` must be named `%rslt`.
///
/// Notice if the image mapping optimization is disabled,
/// in the implmentation you must also perform address translation
/// for all pointer components contained in the return value.
pub const DEFS: &[(&str, i32, &str)] = &[
    (
        "exit",
        93,
        "  %rslt = call i64 (i64, ...) @syscall(i64 60, i64 %arg1)",
    ),
    (
        "exit_group",
        94,
        "  %rslt = call i64 (i64, ...) @syscall(i64 231, i64 %arg1)",
    ),
    (
        "getpid",
        172,
        "  %rslt = call i64 (i64, ...) @syscall(i64 39)",
    ),
    (
        "kill",
        129,
        "  %rslt = call i64 (i64, ...) @syscall(i64 62, i64 %arg1, i64 %arg2)",
    ),
    (
        "tgkill",
        131,
        "  %rslt = call i64 (i64, ...) @syscall(i64 234, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "read",
        63,
        "  %rslt = call i64 (i64, ...) @syscall(i64 0, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "write",
        64,
        "  %rslt = call i64 (i64, ...) @syscall(i64 1, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "openat",
        56,
        "  %rslt = call i64 (i64, ...) @syscall(i64 257, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "close",
        57,
        "  %rslt = call i64 (i64, ...) @syscall(i64 3, i64 %arg1)",
    ),
    (
        "lseek",
        62,
        "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "brk",
        214,
        "  %rslt = call i64 (i64, ...) @syscall(i64 12, i64 %arg1)",
    ),
    (
        "linkat",
        37,
        "  %rslt = call i64 (i64, ...) @syscall(i64 265, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5)",
    ),
    (
        "unlinkat",
        35,
        "  %rslt = call i64 (i64, ...) @syscall(i64 263, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "mkdirat",
        34,
        "  %rslt = call i64 (i64, ...) @syscall(i64 258, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "renameat",
        38,
        "  %rslt = call i64 (i64, ...) @syscall(i64 264, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "chdir",
        49,
        "  %rslt = call i64 (i64, ...) @syscall(i64 80, i64 %arg1)",
    ),
    (
        "getcwd",
        17,
        "  %rslt = call i64 (i64, ...) @syscall(i64 79, i64 %arg1, i64 %arg2)",
    ),
    (
        "fstat",
        80,
        "  %pad_addr = add i64 %arg2, 120
  %pad_ptr = inttoptr i64 %pad_addr to ptr
  %pad = load i192, ptr %pad_ptr
  %rslt = call i64 (i64, ...) @syscall(i64 5, i64 %arg1, i64 %arg2)
  %statbuf = inttoptr i64 %arg2 to ptr
  call void @.sys_conv_stat(ptr %statbuf)
  store i192 %pad, ptr %pad_ptr",
    ),
    (
        "newfstatat",
        79,
        "  %pad_addr = add i64 %arg3, 120
  %pad_ptr = inttoptr i64 %pad_addr to ptr
  %pad = load i192, ptr %pad_ptr
  %rslt = call i64 (i64, ...) @syscall(i64 262, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)
  %statbuf = inttoptr i64 %arg3 to ptr
  call void @.sys_conv_stat(ptr %statbuf)
  store i192 %pad, ptr %pad_ptr",
    ),
    (
        "faccessat",
        48,
        "  %rslt = call i64 (i64, ...) @syscall(i64 269, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "pread64",
        67,
        "  %rslt = call i64 (i64, ...) @syscall(i64 17, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "pwrite64",
        68,
        "  %rslt = call i64 (i64, ...) @syscall(i64 18, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "uname",
        160,
        "  %rslt = call i64 (i64, ...) @syscall(i64 63, i64 %arg1)",
    ),
    (
        "getuid",
        174,
        "  %rslt = call i64 (i64, ...) @syscall(i64 102)",
    ),
    (
        "geteuid",
        175,
        "  %rslt = call i64 (i64, ...) @syscall(i64 107)",
    ),
    (
        "getgid",
        176,
        "  %rslt = call i64 (i64, ...) @syscall(i64 104)",
    ),
    (
        "getegid",
        177,
        "  %rslt = call i64 (i64, ...) @syscall(i64 108)",
    ),
    (
        "gettid",
        178,
        "  %rslt = call i64 (i64, ...) @syscall(i64 186)",
    ),
    (
        "sysinfo",
        179,
        "  %rslt = call i64 (i64, ...) @syscall(i64 99, i64 %arg1)",
    ),
    (
        "mmap",
        222,
        "  %rslt = call i64 (i64, ...) @syscall(i64 9, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)",
    ),
    (
        "munmap",
        215,
        "  %rslt = call i64 (i64, ...) @syscall(i64 11, i64 %arg1, i64 %arg2)",
    ),
    (
        "mremap",
        216,
        "  %rslt = call i64 (i64, ...) @syscall(i64 25, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5)",
    ),
    (
        "mprotect",
        226,
        "  ; Always return 0, because it fails for legal input in RISC-V
  %rslt = add i64 0, 0",
    ),
    (
        "prlimit64",
        261,
        "  %rslt = call i64 (i64, ...) @syscall(i64 302, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "rt_sigaction",
        134,
        "  %rslt = call i64 (i64, ...) @syscall(i64 13, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "writev",
        66,
        "  %rslt = call i64 (i64, ...) @syscall(i64 20, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "gettimeofday",
        169,
        "  %rslt = call i64 (i64, ...) @syscall(i64 96, i64 %arg1, i64 %arg2)",
    ),
    (
        "times",
        153,
        "  %rslt = call i64 (i64, ...) @syscall(i64 100, i64 %arg1)",
    ),
    (
        "fcntl",
        25,
        "  %rslt = call i64 (i64, ...) @syscall(i64 72, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "ftruncate",
        46,
        "  %rslt = call i64 (i64, ...) @syscall(i64 77, i64 %arg1, i64 %arg2)",
    ),
    (
        "getdents64",
        61,
        "  %rslt = call i64 (i64, ...) @syscall(i64 217, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "dup",
        23,
        "  %rslt = call i64 (i64, ...) @syscall(i64 32, i64 %arg1)",
    ),
    (
        "dup3",
        24,
        "  %rslt = call i64 (i64, ...) @syscall(i64 292, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "readlinkat",
        78,
        "  %rslt = call i64 (i64, ...) @syscall(i64 267, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "rt_sigprocmask",
        135,
        "  %rslt = call i64 (i64, ...) @syscall(i64 14, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "ioctl",
        29,
        "  %rslt = call i64 (i64, ...) @syscall(i64 16, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "getrlimit",
        163,
        "  %rslt = call i64 (i64, ...) @syscall(i64 97, i64 %arg1, i64 %arg2)",
    ),
    (
        "setrlimit",
        164,
        "  %rslt = call i64 (i64, ...) @syscall(i64 160, i64 %arg1, i64 %arg2)",
    ),
    (
        "getrusage",
        165,
        "  %rslt = call i64 (i64, ...) @syscall(i64 98, i64 %arg1, i64 %arg2)",
    ),
    (
        "clock_gettime",
        113,
        "  %rslt = call i64 (i64, ...) @syscall(i64 228, i64 %arg1, i64 %arg2)",
    ),
    (
        "set_tid_address",
        96,
        "  %rslt = call i64 (i64, ...) @syscall(i64 218, i64 %arg1)",
    ),
    (
        "set_robust_list",
        99,
        "  %rslt = call i64 (i64, ...) @syscall(i64 273, i64 %arg1, i64 %arg2)",
    ),
    (
        "madvise",
        233,
        "  %rslt = call i64 (i64, ...) @syscall(i64 28, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "statx",
        291,
        "  %rslt = call i64 (i64, ...) @syscall(i64 332, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5)",
    ),
    (
        "open",
        1024,
        "  %rslt = call i64 (i64, ...) @syscall(i64 2, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "link",
        1025,
        "  %rslt = call i64 (i64, ...) @syscall(i64 86, i64 %arg1, i64 %arg2)",
    ),
    (
        "unlink",
        1026,
        "  %rslt = call i64 (i64, ...) @syscall(i64 87, i64 %arg1)",
    ),
    (
        "mkdir",
        1030,
        "  %rslt = call i64 (i64, ...) @syscall(i64 83, i64 %arg1, i64 %arg2)",
    ),
    (
        "access",
        1033,
        "  %rslt = call i64 (i64, ...) @syscall(i64 21, i64 %arg1, i64 %arg2)",
    ),
    (
        "stat",
        1038,
        "  %pad_addr = add i64 %arg2, 120
  %pad_ptr = inttoptr i64 %pad_addr to ptr
  %pad = load i192, ptr %pad_ptr
  %rslt = call i64 (i64, ...) @syscall(i64 4, i64 %arg1, i64 %arg2)
  %statbuf = inttoptr i64 %arg2 to ptr
  call void @.sys_conv_stat(ptr %statbuf)
  store i192 %pad, ptr %pad_ptr",
    ),
    (
        "lstat",
        1039,
        "  %pad_addr = add i64 %arg2, 120
  %pad_ptr = inttoptr i64 %pad_addr to ptr
  %pad = load i192, ptr %pad_ptr
  %rslt = call i64 (i64, ...) @syscall(i64 6, i64 %arg1, i64 %arg2)
  %statbuf = inttoptr i64 %arg2 to ptr
  call void @.sys_conv_stat(ptr %statbuf)
  store i192 %pad, ptr %pad_ptr",
    ),
    (
        "time",
        1062,
        "  %rslt = call i64 (i64, ...) @syscall(i64 201, i64 %arg1)",
    ),
    (
        "renameat2",
        276,
        "  %rslt = call i64 (i64, ...) @syscall(i64 316, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5)",
    ),
    (
        "clone",
        220,
        "  %rslt = call i64 (i64, ...) @syscall(i64 56, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5)",
    ),
    (
        "getrandom",
        278,
        "  %rslt = call i64 (i64, ...) @syscall(i64 318, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "pipe2",
        59,
        "  %rslt = call i64 (i64, ...) @syscall(i64 293, i64 %arg1, i64 %arg2)",
    ),
    (
        "wait4",
        260,
        "  %rslt = call i64 (i64, ...) @syscall(i64 61, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "execve",
        221,
        "  %rslt = call i64 (i64, ...) @syscall(i64 59, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "ppoll",
        73,
        "  %rslt = call i64 (i64, ...) @syscall(i64 271, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5)",
    ),
    (
        "clock_nanosleep",
        115,
        "  %rslt = call i64 (i64, ...) @syscall(i64 230, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4)",
    ),
    (
        "truncate",
        45,
        "  %rslt = call i64 (i64, ...) @syscall(i64 76, i64 %arg1, i64 %arg2)",
    ),
    (
        "fchmodat",
        53,
        "  %rslt = call i64 (i64, ...) @syscall(i64 268, i64 %arg1, i64 %arg2, i64 %arg3)",
    ),
    (
        "nanosleep",
        101,
        "  %rslt = call i64 (i64, ...) @syscall(i64 35, i64 %arg1, i64 %arg2)",
    ),
    (
        "pselect6",
        72,
        "  %rslt = call i64 (i64, ...) @syscall(i64 270, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)",
    ),
    (
        "timer_create",
        107,
        "  %rslt = call i64 (i64, ...) @syscall(i64 222, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)",
    ),
    (
        "timer_gettime",
        108,
        "  %rslt = call i64 (i64, ...) @syscall(i64 224, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)",
    ),
    (
        "timer_getoverrun",
        109,
        "  %rslt = call i64 (i64, ...) @syscall(i64 225, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)",
    ),
    (
        "timer_settime",
        110,
        "  %rslt = call i64 (i64, ...) @syscall(i64 223, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)",
    ),
    (
        "timer_delete",
        111,
        "  %rslt = call i64 (i64, ...) @syscall(i64 226, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)",
    ),
];
