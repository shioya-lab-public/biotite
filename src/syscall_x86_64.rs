use lazy_static::lazy_static;

pub const STRUCTS: &str = "%.SYS.sigaction = type { i8*, i8*, i32, i8* }
%.SYS.iovec = type { i8*, i64 }
%.SYS.dirent = type { i64, i64, i16, i8, i8* }
%.SYS.robust_list_head = type { i8*, i64, i8* }

%.x86_64.stat = type { i64, i64, i64, i32, i32, i32, i32, i64 }
%.riscv64.stat = type { i64, i64, i32, i32, i32, i32, i64 }

define void @.conv_stat(i8* %x86_64_statbuf_b) {
    %x86_64_statbuf = bitcast i8* %x86_64_statbuf_b to %.x86_64.stat*
    %x86_64_stat = load %.x86_64.stat, %.x86_64.stat* %x86_64_statbuf
    %_st_nlink = extractvalue %.x86_64.stat %x86_64_stat, 2
    %st_nlink = trunc i64 %_st_nlink to i32
    %st_mode = extractvalue %.x86_64.stat %x86_64_stat, 3
    %st_uid = extractvalue %.x86_64.stat %x86_64_stat, 4
    %st_gid = extractvalue %.x86_64.stat %x86_64_stat, 5
    %st_rdev = extractvalue %.x86_64.stat %x86_64_stat, 7

    %riscv64_statbuf = bitcast i8* %x86_64_statbuf_b to %.riscv64.stat*
    %riscv64_stat = load %.riscv64.stat, %.riscv64.stat* %riscv64_statbuf
    %riscv64_stat1 = insertvalue %.riscv64.stat %riscv64_stat, i32 %st_mode, 2
    %riscv64_stat2 = insertvalue %.riscv64.stat %riscv64_stat1, i32 %st_nlink, 3
    %riscv64_stat3 = insertvalue %.riscv64.stat %riscv64_stat2, i32 %st_uid, 4
    %riscv64_stat4 = insertvalue %.riscv64.stat %riscv64_stat3, i32 %st_gid, 5
    %riscv64_stat5 = insertvalue %.riscv64.stat %riscv64_stat4, i64 %st_rdev, 6
    store %.riscv64.stat %riscv64_stat5, %.riscv64.stat* %riscv64_statbuf
    ret void
}";

lazy_static! {
    pub static ref SYSCALLS: Vec<(&'static str, i32, &'static str)> = vec![
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
            "  %buf = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 0, i64 %arg1, i8* %buf, i64 %arg3)"
        ),
        (
            "write",
            64,
            "  %buf = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 1, i64 %arg1, i8* %buf, i64 %arg3)"
        ),
        (
            "openat",
            56,
            "  %filename = call i8* @.get_memory_ptr(i64 %arg2)
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
            "  %oldname = call i8* @.get_memory_ptr(i64 %arg2)
  %newname = call i8* @.get_memory_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 265, i64 %arg1, i8* %oldname, i64 %arg3, i8* %newname, i64 %arg5)"
        ),
        (
            "unlinkat",
            35,
            "  %pathname = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 263, i64 %arg1, i8* %pathname, i64 %arg3)"
        ),
        (
            "mkdirat",
            34,
            "  %pathname = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 258, i64 %arg1, i8* %pathname, i64 %arg3)"
        ),
        (
            "renameat",
            38,
            "  %oldname = call i8* @.get_memory_ptr(i64 %arg2)
  %newname = call i8* @.get_memory_ptr(i64 %arg4)
  %rslt = call i64 (i64, ...) @syscall(i64 264, i64 %arg1, i8* %oldname, i64 %arg3, i8* %newname)"
                  ),
        (
            "chdir",
            49,
            "  %filename = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 80, i8* %filename)"
        ),
        (
            "getcwd",
            17,
            "  %buf = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 79, i8* %buf, i64 %arg2)"
        ),
        (
            "fstat",
            80,
            "  %pad_addr = add i64 %arg2, 120
  %pad_b = call i8* @.get_memory_ptr(i64 %pad_addr)
  %pad = bitcast i8* %pad_b to i192*
  %pad_val = load i192, i192* %pad
  %statbuf = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 5, i64 %arg1, i8* %statbuf)
  call void @.conv_stat(i8* %statbuf)
  store i192 %pad_val, i192* %pad"
        ),
        (
            "fstatat",
            79,
            "  %pad_addr = add i64 %arg3, 120
  %pad_b = call i8* @.get_memory_ptr(i64 %pad_addr)
  %pad = bitcast i8* %pad_b to i192*
  %pad_val = load i192, i192* %pad
  %filename = call i8* @.get_memory_ptr(i64 %arg2)
  %statbuf = call i8* @.get_memory_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 262, i64 %arg1, i8* %filename, i8* %statbuf, i64 %arg4)
  call void @.conv_stat(i8* %statbuf)
  store i192 %pad_val, i192* %pad"
        ),
        (
            "faccessat",
            48,
            "  %filename = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 269, i64 %arg1, i8* %filename, i64 %arg3)"
        ),
        (
            "pread",
            67,
            "  %buf = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 17, i64 %arg1, i8* %buf, i64 %arg3, i64 %arg4)"
        ),
        (
            "pwrite",
            68,
            "  %buf = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 18, i64 %arg1, i8* %buf, i64 %arg3, i64 %arg4)"
        ),
        (
            "uname",
            160,
            "  %utsname = call i8* @.get_memory_ptr(i64 %arg1)
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
            "  %sysinfo = call i8* @.get_memory_ptr(i64 %arg1)
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
            "  %addr = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 11, i8* %addr, i64 %arg2)"
        ),
        (
            "mremap",
            216,
            "  %addr = call i8* @.get_memory_ptr(i64 %arg1)
  %new_addr = call i8* @.get_memory_ptr(i64 %arg5)
  %rslt = call i64 (i64, ...) @syscall(i64 25, i8* %addr, i64 %arg2, i64 %arg3, i64 %arg4, i8* %new_addr)"
        ),
        (
            "mprotect",
            226,
            "  %rslt = add i64 0, 0"
        ),
        (
            "prlimit64",
            261,
            "  %new_rlim = call i8* @.get_memory_ptr(i64 %arg3)
  %old_rlim = call i8* @.get_memory_ptr(i64 %arg4)
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
            "  %act_ptr_b = call i8* @.get_memory_ptr(i64 %arg2)
  %act_ptr = bitcast i8* %act_ptr_b to %.SYS.sigaction*
  %act = load %.SYS.sigaction, %.SYS.sigaction* %act_ptr
  %field_0 = extractvalue %.SYS.sigaction %act, 0
  %field_0_val = ptrtoint i8* %field_0 to i64
  %host_field_0 = call i8* @.get_memory_ptr(i64 %field_0_val)
  %act_0 = insertvalue %.SYS.sigaction %act, i8* %host_field_0, 0
  %field_1 = extractvalue %.SYS.sigaction %act, 1
  %field_1_val = ptrtoint i8* %field_1 to i64
  %host_field_1 = call i8* @.get_memory_ptr(i64 %field_1_val)
  %act_1 = insertvalue %.SYS.sigaction %act_0, i8* %host_field_1, 1
  %field_3 = extractvalue %.SYS.sigaction %act, 3
  %field_3_val = ptrtoint i8* %field_3 to i64
  %host_field_3 = call i8* @.get_memory_ptr(i64 %field_3_val)
  %act_3 = insertvalue %.SYS.sigaction %act_1, i8* %host_field_3, 3
  store %.SYS.sigaction %act_3, %.SYS.sigaction* %act_ptr
  %oldact = call i8* @.get_memory_ptr(i64 %arg3)
  %rslt = call i64 (i64, ...) @syscall(i64 13, i64 %arg1, i8* %act_ptr_b, i8* %oldact, i64 %arg4)"
        ),
        (
            "writev",
            66,
            "  %vecs_b = call i8* @.get_memory_ptr(i64 %arg2)
  %vecs = bitcast i8* %vecs_b to %.SYS.iovec*
  %i_ptr = alloca i64
  store i64 0, i64* %i_ptr
  br label %test
test:
  %i = load i64, i64* %i_ptr
  %cont = icmp slt i64 %i, %arg3
  br i1 %cont, label %trans, label %call
trans:
  %vec_ptr = getelementptr %.SYS.iovec, %.SYS.iovec* %vecs, i64 %i
  %vec = load %.SYS.iovec, %.SYS.iovec* %vec_ptr
  %field_0 = extractvalue %.SYS.iovec %vec, 0
  %field_0_val = ptrtoint i8* %field_0 to i64
  %host_field_0 = call i8* @.get_memory_ptr(i64 %field_0_val)
  %vec_0 = insertvalue %.SYS.iovec %vec, i8* %host_field_0, 0
  store %.SYS.iovec %vec_0, %.SYS.iovec* %vec_ptr
  br label %add
add:
  %new_i = add i64 %i, 1
  store i64 %new_i, i64* %i_ptr
  br label %test
call:
  %rslt = call i64 (i64, ...) @syscall(i64 20, i64 %arg1, i8* %vecs_b, i64 %arg3)"
        ),
        (
            "gettimeofday",
            169,
            "  %tv = call i8* @.get_memory_ptr(i64 %arg1)
  %tz = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 96, i8* %tv, i8* %tz)"
        ),
        (
            "times",
            153,
            "  %tbuf = call i8* @.get_memory_ptr(i64 %arg1)
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
            "  %dirent_ptr_b = call i8* @.get_memory_ptr(i64 %arg2)
  %dirent_ptr = bitcast i8* %dirent_ptr_b to %.SYS.dirent*
  %dirent = load %.SYS.dirent, %.SYS.dirent* %dirent_ptr
  %field_4 = extractvalue %.SYS.dirent %dirent, 4
  %field_4_val = ptrtoint i8* %field_4 to i64
  %host_field_4 = call i8* @.get_memory_ptr(i64 %field_4_val)
  %dirent_4 = insertvalue %.SYS.dirent %dirent, i8* %host_field_4, 4
  store %.SYS.dirent %dirent_4, %.SYS.dirent* %dirent_ptr
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
            "  %path = call i8* @.get_memory_ptr(i64 %arg2)
  %buf = call i8* @.get_memory_ptr(i64 %arg3)
  %_rslt = call i64 (i64, ...) @syscall(i64 267, i64 %arg1, i8* %path, i8* %buf, i64 %arg4)
  %is_err = icmp eq i64 %_rslt, -1
  %rslt = select i1 %is_err, i64 -22, i64 %_rslt"
        ),
        (
            "rt_sigprocmask",
            135,
            "  %set = call i8* @.get_memory_ptr(i64 %arg2)
  %oset = call i8* @.get_memory_ptr(i64 %arg3)
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
            "  %rlim = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 97, i64 %arg1, i8* %rlim)"
        ),
        (
            "setrlimit",
            164,
            "  %rlim = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 160, i64 %arg1, i8* %rlim)"
        ),
        (
            "getrusage",
            165,
            "  %ru = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 98, i64 %arg1, i8* %ru)"
        ),
        (
            "clock_gettime",
            113,
            "  %tp = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 228, i64 %arg1, i8* %tp)"
        ),
        (
            "set_tid_address",
            96,
            "  %tidptr = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 218, i8* %tidptr)"
        ),
        (
            "set_robust_list",
            99,
            "  %head_ptr_b = call i8* @.get_memory_ptr(i64 %arg1)
  %head_ptr = bitcast i8* %head_ptr_b to %.SYS.robust_list_head*
  %head = load %.SYS.robust_list_head, %.SYS.robust_list_head* %head_ptr
  %field_0 = extractvalue %.SYS.robust_list_head %head, 0
  %field_0_val = ptrtoint i8* %field_0 to i64
  %host_field_0 = call i8* @.get_memory_ptr(i64 %field_0_val)
  %head_0 = insertvalue %.SYS.robust_list_head %head, i8* %host_field_0, 0
  %field_2 = extractvalue %.SYS.robust_list_head %head, 2
  %field_2_val = ptrtoint i8* %field_2 to i64
  %host_field_2 = call i8* @.get_memory_ptr(i64 %field_2_val)
  %head_2 = insertvalue %.SYS.robust_list_head %head_0, i8* %host_field_2, 2
  store %.SYS.robust_list_head %head_2, %.SYS.robust_list_head* %head_ptr
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
            "  %path = call i8* @.get_memory_ptr(i64 %arg2)
  %buffer = call i8* @.get_memory_ptr(i64 %arg5)
  %rslt = call i64 (i64, ...) @syscall(i64 332, i64 %arg1, i8* %path, i64 %arg3, i64 %arg4, i8* %buffer)"
        ),
        (
            "open",
            1024,
            "  %filename = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 2, i8* %filename, i64 %arg2, i64 %arg3)"
        ),
        (
            "link",
            1025,
            "  %oldname = call i8* @.get_memory_ptr(i64 %arg1)
  %newname = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 86, i8* %oldname, i8* %newname)"
        ),
        (
            "unlink",
            1026,
            "  %pathname = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 87, i8* %pathname)"
        ),
        (
            "mkdir",
            1030,
            "  %pathname = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 83, i8* %pathname, i64 %arg2)"
        ),
        (
            "access",
            1033,
            "  %filename = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 21, i8* %filename, i64 %arg2)"
        ),
        (
            "stat",
            1038,
            "  %pad_addr = add i64 %arg2, 120
  %pad_b = call i8* @.get_memory_ptr(i64 %pad_addr)
  %pad = bitcast i8* %pad_b to i192*
  %pad_val = load i192, i192* %pad
  %filename = call i8* @.get_memory_ptr(i64 %arg1)
  %statbuf = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 4, i8* %filename, i8* %statbuf)
  call void @.conv_stat(i8* %statbuf)
  store i192 %pad_val, i192* %pad"
        ),
        (
            "lstat",
            1039,
            "  %pad_addr = add i64 %arg2, 120
  %pad_b = call i8* @.get_memory_ptr(i64 %pad_addr)
  %pad = bitcast i8* %pad_b to i192*
  %pad_val = load i192, i192* %pad
  %filename = call i8* @.get_memory_ptr(i64 %arg1)
  %statbuf = call i8* @.get_memory_ptr(i64 %arg2)
  %rslt = call i64 (i64, ...) @syscall(i64 6, i8* %filename, i8* %statbuf)
  call void @.conv_stat(i8* %statbuf)
  store i192 %pad_val, i192* %pad"
        ),
        (
            "time",
            1062,
            "  %tloc = call i8* @.get_memory_ptr(i64 %arg1)
  %rslt = call i64 (i64, ...) @syscall(i64 201, i8* %tloc)"
        ),
    ];
}
