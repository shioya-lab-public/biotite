use once_cell::sync::Lazy;

pub const AUX: &str = "%.sys_dirent = type { i64, i64, i16, i8, i8* }
%.sys_iovec = type { i8*, i64 }
%.sys_robust_list_head = type { i8*, i64, i8* }
%.sys_sigaction = type { i8*, i8*, i32, i8* }";

pub static DEFS: Lazy<Vec<(&str, i32, &str)>> = Lazy::new(|| {
    vec![
        (
            "exit",
            93,
            "  %rslt = call i64 (i64, ...) @syscall(i64 93, i64 %arg1)"
        ),
        (
            "exit_group",
            94,
            "  %rslt = call i64 (i64, ...) @syscall(i64 94, i64 %arg1)"
        ),
        (
            "getpid",
            172,
            "  %rslt = call i64 (i64, ...) @syscall(i64 172)"
        ),
        (
            "kill",
            129,
            "  %rslt = call i64 (i64, ...) @syscall(i64 129, i64 %arg1, i64 %arg2)"
        ),
        (
            "tgkill",
            131,
            "  %rslt = call i64 (i64, ...) @syscall(i64 131, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "read",
            63,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 63, i64 %arg1, i8* %buf, i64 %arg3)"
        ),
        (
            "write",
            64,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 64, i64 %arg1, i8* %buf, i64 %arg3)"
        ),
        (
            "openat",
            56,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 56, i64 %arg1, i8* %filename, i64 %arg3, i64 %arg4)"
        ),
        (
            "close",
            57,
            "  %rslt = call i64 (i64, ...) @syscall(i64 57, i64 %arg1)"
        ),
        (
            "lseek",
            62,
            "  %rslt = call i64 (i64, ...) @syscall(i64 62, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "brk",
            214,
            "  %rslt = call i64 (i64, ...) @syscall(i64 214, i64 %arg1)"
        ),
        (
            "linkat",
            37,
            "  %oldname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %newname = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 37, i64 %arg1, i8* %oldname, i64 %arg3, i8* %newname, i64 %arg5)"
        ),
        (
            "unlinkat",
            35,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 35, i64 %arg1, i8* %pathname, i64 %arg3)"
        ),
        (
            "mkdirat",
            34,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 34, i64 %arg1, i8* %pathname, i64 %arg3)"
        ),
        (
            "renameat",
            38,
            "  %oldname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %newname = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 38, i64 %arg1, i8* %oldname, i64 %arg3, i8* %newname)"
                  ),
        (
            "chdir",
            49,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 49, i8* %filename)"
        ),
        (
            "getcwd",
            17,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 17, i8* %buf, i64 %arg2)"
        ),
        (
            "fstat",
            80,
            "  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 80, i64 %arg1, i8* %statbuf)"
        ),
        (
            "fstatat",
            79,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 79, i64 %arg1, i8* %filename, i8* %statbuf, i64 %arg4)"
        ),
        (
            "faccessat",
            48,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 48, i64 %arg1, i8* %filename, i64 %arg3)"
        ),
        (
            "pread",
            67,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 67, i64 %arg1, i8* %buf, i64 %arg3, i64 %arg4)"
        ),
        (
            "pwrite",
            68,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 68, i64 %arg1, i8* %buf, i64 %arg3, i64 %arg4)"
        ),
        (
            "uname",
            160,
            "  %utsname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 160, i8* %utsname)"
        ),
        (
            "getuid",
            174,
            "  %rslt = call i64 (i64, ...) @syscall(i64 174)"
        ),
        (
            "geteuid",
            175,
            "  %rslt = call i64 (i64, ...) @syscall(i64 175)"
        ),
        (
            "getgid",
            176,
            "  %rslt = call i64 (i64, ...) @syscall(i64 176)"
        ),
        (
            "getegid",
            177,
            "  %rslt = call i64 (i64, ...) @syscall(i64 177)"
        ),
        (
            "gettid",
            178,
            "  %rslt = call i64 (i64, ...) @syscall(i64 178)"
        ),
        (
            "sysinfo",
            179,
            "  %sysinfo = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 179, i8* %sysinfo)"
        ),
        (
            "mmap",
            222,
            "  ; Ignore the address hint in `arg1`
  %rslt = call i64 (i64, ...) @syscall(i64 222, i64 0, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)"
        ),
        (
            "munmap",
            215,
            "  %addr = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 215, i8* %addr, i64 %arg2)"
        ),
        (
            "mremap",
            216,
            "  %addr = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %new_addr = call i8* @.sys_get_mem_ptr(i64 %arg5)
  %rslt = call i64 (i64, ...) @syscall(i64 216, i8* %addr, i64 %arg2, i64 %arg3, i64 %arg4, i8* %new_addr)"
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
  %rslt = call i64 (i64, ...) @syscall(i64 261, i64 %arg1, i64 %arg2, i8* %new_rlim, i8* %old_rlim)"
        ),
        (
            "getmainvars",
            2011,
            "  ; There is no enough information for this system call
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
  %rslt = call i64 (i64, ...) @syscall(i64 134, i64 %arg1, i8* %act_ptr_b, i8* %oldact, i64 %arg4)"
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
  %rslt = call i64 (i64, ...) @syscall(i64 66, i64 %arg1, i8* %vecs_b, i64 %arg3)"
        ),
        (
            "gettimeofday",
            169,
            "  %tv = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %tz = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 169, i8* %tv, i8* %tz)"
        ),
        (
            "times",
            153,
            "  %tbuf = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 153, i8* %tbuf)"
        ),
        (
            "fcntl",
            25,
            "  %rslt = call i64 (i64, ...) @syscall(i64 25, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "ftruncate",
            46,
            "  %rslt = call i64 (i64, ...) @syscall(i64 46, i64 %arg1, i64 %arg2)"
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
  %rslt = call i64 (i64, ...) @syscall(i64 61, i64 %arg1, i8* %dirent_ptr_b, i64 %arg3)"
        ),
        (
            "dup",
            23,
            "  %rslt = call i64 (i64, ...) @syscall(i64 23, i64 %arg1)"
        ),
        (
            "dup3",
            24,
            "  %rslt = call i64 (i64, ...) @syscall(i64 24, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "readlinkat",
            78,
            "  %path = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %buf = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 78, i64 %arg1, i8* %path, i8* %buf, i64 %arg4)"
        ),
        (
            "rt_sigprocmask",
            135,
            "  %set = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %oset = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 135, i64 %arg1, i8* %set, i8* %oset, i64 %arg4)"
        ),
        (
            "ioctl",
            29,
            "  %arg = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 29, i64 %arg1, i64 %arg2, i8* %arg)"
        ),
        (
            "getrlimit",
            163,
            "  %rlim = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 163, i64 %arg1, i8* %rlim)"
        ),
        (
            "setrlimit",
            164,
            "  %rlim = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 164, i64 %arg1, i8* %rlim)"
        ),
        (
            "getrusage",
            165,
            "  %ru = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 165, i64 %arg1, i8* %ru)"
        ),
        (
            "clock_gettime",
            113,
            "  %tp = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 113, i64 %arg1, i8* %tp)"
        ),
        (
            "set_tid_address",
            96,
            "  %tidptr = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 96, i8* %tidptr)"
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
  %rslt = call i64 (i64, ...) @syscall(i64 99, i8* %head_ptr_b, i64 %arg2)"
        ),
        (
            "madvise",
            233,
            "  %rslt = call i64 (i64, ...) @syscall(i64 233, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "statx",
            291,
            "  %path = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %buffer = call i8* @.sys_get_mem_ptr(i64 %arg5)
  %rslt = call i64 (i64, ...) @syscall(i64 291, i64 %arg1, i8* %path, i64 %arg3, i64 %arg4, i8* %buffer)"
        ),
        (
            "open",
            1024,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 1024, i8* %filename, i64 %arg2, i64 %arg3)"
        ),
        (
            "link",
            1025,
            "  %oldname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %newname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 1025, i8* %oldname, i8* %newname)"
        ),
        (
            "unlink",
            1026,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 1026, i8* %pathname)"
        ),
        (
            "mkdir",
            1030,
            "  %pathname = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 1030, i8* %pathname, i64 %arg2)"
        ),
        (
            "access",
            1033,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 1033, i8* %filename, i64 %arg2)"
        ),
        (
            "stat",
            1038,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 1038, i8* %filename, i8* %statbuf)"
        ),
        (
            "lstat",
            1039,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %statbuf = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 1039, i8* %filename, i8* %statbuf)"
        ),
        (
            "time",
            1062,
            "  %tloc = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 1062, i8* %tloc)"
        ),
        (
            "renameat2",
            276,
            "  %oldname = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %newname = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 276, i64 %arg1, i8* %oldname, i64 %arg3, i8* %newname, i64 %arg5)"
        ),
        (
            "clone",
            220,
            "  %parent_tidptr = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %child_tidptr = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 220, i64 %arg1, i64 %arg2, i8* %parent_tidptr, i8* %child_tidptr, i64 %arg5)"
        ),
        (
            "getrandom",
            278,
            "  %buf = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 278, i8* %buf, i64 %arg2, i64 %arg3)"
        ),
        (
            "pipe2",
            59,
            "  %fildes = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 59, i8* %fildes, i64 %arg2)"
        ),
        (
            "wait4",
            260,
            "  %stat_addr = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %ru = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 260, i64 %arg1, i8* %stat_addr, i64 %arg3, i8* %ru)"
        ),
        (
            "execve",
            221,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %argv = call i8** @.trans_mem_ptr_vec(i64 %arg2)
  %envp = call i8** @.trans_mem_ptr_vec(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 221, i8* %filename, i8** %argv, i8** %envp)"
        ),
        (
            "ppoll",
            73,
            "  %ufds = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %tsp = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %sigmask = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 73, i8* %ufds, i64 %arg2, i8* %tsp, i8* %sigmask, i64 %arg5)"
        ),
        (
            "clock_nanosleep",
            115,
            "  %rqtp = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %rmtp = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 115, i64 %arg1, i64 %arg2, i8* %rqtp, i8* %rmtp)"
        ),
        (
            "truncate",
            45,
            "  %path = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 45, i8* %path, i64 %arg2)"
        ),
        (
            "fchmodat",
            53,
            "  %filename = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 53, i64 %arg1, i8* %filename, i64 %arg3)"
        ),
        (
            "nanosleep",
            101,
            "  %rqtp = call i8* @.sys_get_mem_ptr(i64 %arg1)
  %rmtp = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 101, i8* %rqtp, i8* %rmtp)"
        ),
        (
            "pselect6",
            72,
            "  %readfds = call i8* @.sys_get_mem_ptr(i64 %arg2)
  %writefds = call i8* @.sys_get_mem_ptr(i64 %arg3)
  %exceptfds = call i8* @.sys_get_mem_ptr(i64 %arg4)
  %timeout = call i8* @.sys_get_mem_ptr(i64 %arg5)
  %sigmask = call i8* @.sys_get_mem_ptr(i64 %arg6)
  %rslt = call i64 (i64, ...) @syscall(i64 72, i64 %arg1, i8* %readfds, i8* %writefds, i8* %exceptfds, i8* %timeout, i8* %sigmask)"
        ),
    ]
});
