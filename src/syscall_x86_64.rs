use lazy_static::lazy_static;

pub const STRUCTS: &str = "



%struct.stat = type { i64, i64, i64, i32, i32, i32, i32, i64, i64, i64, i64, %struct.timespec, %struct.timespec, %struct.timespec, [3 x i64] }
%struct.timespec = type { i64, i64 }

%struct.tms = type { i64, i64, i64, i64 }

%struct.utsname = type { i8*, i8*, i8*, i8*, i8*, i8* }
%struct.iovec = type { i8*, i64 }";

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
            "  %statbuf_b = call i8* @.get_memory_ptr(i64 %arg2)
            %stat = bitcast i8* %statbuf_b to %struct.stat*
            %SYS_fstat_rslt = call i64 (i64, ...) @syscall(i64 5, i64 %arg1, %struct.stat* %stat)
            ret i64 %SYS_fstat_rslt"
        ),
        (
            "fstatat",
            79,
            "  %SYS_fstatat_dfd = trunc i64 %arg1 to i32
            %SYS_fstatat_filename = call i8* @.get_memory_ptr(i64 %arg2)
            %SYS_fstatat_statbuf_b = call i8* @.get_memory_ptr(i64 %arg3)
            %SYS_fstatat_statbuf = bitcast i8* %SYS_fstatat_statbuf_b to %struct.stat*
            %SYS_fstatat_flag = trunc i64 %arg4 to i32
            %SYS_fstatat_rslt = call i64 (i64, ...) @syscall(i64 262, i32 %SYS_fstatat_dfd, i8* %SYS_fstatat_filename, %struct.stat* %SYS_fstatat_statbuf, i32 %SYS_fstatat_flag)
            ret i64 %SYS_fstatat_rslt"
        ),
        (
            "faccessat",
            48,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "pread",
            67,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "pwrite",
            68,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "uname",
            160,
            "  %utsname_ptr_b = call i8* @.get_memory_ptr(i64 %arg1)
            %utsname_ptr = bitcast i8* %utsname_ptr_b to %struct.utsname*
            %SYS_uname_rslt = call i64 (i64, ...) @syscall(i64 63, %struct.utsname* %utsname_ptr)
            ret i64 %SYS_uname_rslt"
        ),
        (
            "getuid",
            174,
            "  %SYS_getuid_rslt = call i64 (i64, ...) @syscall(i64 102)
            ret i64 %SYS_getuid_rslt"
        ),
        (
            "geteuid",
            175,
            "  %SYS_geteuid_rslt = call i64 (i64, ...) @syscall(i64 107)
            ret i64 %SYS_geteuid_rslt"
        ),
        (
            "getgid",
            176,
            "  %SYS_getgid_rslt = call i64 (i64, ...) @syscall(i64 104)
            ret i64 %SYS_getgid_rslt"
        ),
        (
            "getegid",
            177,
            "  %SYS_getegid_rslt = call i64 (i64, ...) @syscall(i64 108)
            ret i64 %SYS_getegid_rslt"
        ),
        (
            "gettid",
            178,
            "  %SYS_gettid_rslt = call i64 (i64, ...) @syscall(i64 186)
            ret i64 %SYS_gettid_rslt"
        ),
        (
            "sysinfo",
            179,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "mmap",
            222,
            "  %SYS_mmap_rslt = call i64 (i64, ...) @syscall(i64 9, i64 0, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6)
            ret i64 %SYS_mmap_rslt"
        ),
        (
            "munmap",
            215,
            "  %SYS_munmap_rslt = call i64 (i64, ...) @syscall(i64 11, i64 %arg1, i64 %arg2)
            ret i64 %SYS_munmap_rslt"
        ),
        (
            "mremap",
            216,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "mprotect",
            226,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "prlimit64",
            261,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "getmainvars",
            2011,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "rt_sigaction",
            134,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "writev",
            66,
            "  %SYS_writev_vec_b = call i8* @.get_memory_ptr(i64 %arg2)
            %SYS_writev_vec = bitcast i8* %SYS_writev_vec_b to %struct.iovec*
            %i_ptr = alloca i64
            store i64 0, i64* %i_ptr
            br label %test
          test:
            %i = load i64, i64* %i_ptr
            %cont = icmp slt i64 %i, %arg3
            br i1 %cont, label %trans, label %call
          trans:
            %guest_vec_ptr = getelementptr %struct.iovec, %struct.iovec* %SYS_writev_vec, i64 %i
            %guest_vec = load %struct.iovec, %struct.iovec* %guest_vec_ptr
            %guest_base = extractvalue %struct.iovec %guest_vec, 0
            %guest_base_val = ptrtoint i8* %guest_base to i64
            %host_base = call i8* @.get_memory_ptr(i64 %guest_base_val)
            %host_vec = insertvalue %struct.iovec %guest_vec, i8* %host_base, 0
            store %struct.iovec %host_vec, %struct.iovec* %guest_vec_ptr
            br label %add
          add:
            %new_i = add i64 %i, 1
            store i64 %new_i, i64* %i_ptr
            br label %test
          call:
            %SYS_writev_rslt = call i64 (i64, ...) @syscall(i64 20, i64 %arg1, %struct.iovec* %SYS_writev_vec, i64 %arg3)
            ret i64 %SYS_writev_rslt"
        ),
        (
            "gettimeofday",
            169,
            "  %tms_ptr = call i8* @.get_memory_ptr(i64 %arg1)
            %tms = bitcast i8* %tms_ptr to %struct.tms*
            %SYS_gettimeofday_rslt = call i64 (i64, ...) @syscall(i64 96, %struct.tms* %tms, i64 %arg2)
            ret i64 %SYS_gettimeofday_rslt"
        ),
        (
            "times",
            153,
            "  %SYS_times_tms_ptr = call i8* @.get_memory_ptr(i64 %arg1)
            %SYS_times_tms = bitcast i8* %SYS_times_tms_ptr to %struct.tms*
            %SYS_times_rslt = call i64 (i64, ...) @syscall(i64 100, %struct.tms* %SYS_times_tms)
            ret i64 %SYS_times_rslt"
        ),
        (
            "fcntl",
            25,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "ftruncate",
            46,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "getdents",
            61,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "dup",
            23,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "dup3",
            24,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "readlinkat",
            78,
            "  %SYS_readlinkat_dfd = trunc i64 %arg1 to i32
            %SYS_readlinkat_path = call i8* @.get_memory_ptr(i64 %arg2)
            %SYS_readlinkat_buf = call i8* @.get_memory_ptr(i64 %arg3)
            %SYS_readlinkat_bufsiz = trunc i64 %arg4 to i32
            %SYS_readlinkat_rslt = call i64 (i64, ...) @syscall(i64 267, i32 %SYS_readlinkat_dfd, i8* %SYS_readlinkat_path, i8* %SYS_readlinkat_buf, i32 %SYS_readlinkat_bufsiz)
            ret i64 %SYS_readlinkat_rslt"
        ),
        (
            "rt_sigprocmask",
            135,
            "  %SYS_rt_sigprocmask_how = trunc i64 %arg1 to i32
            %SYS_rt_sigprocmask_set_b = call i8* @.get_memory_ptr(i64 %arg2)
            %SYS_rt_sigprocmask_set = bitcast i8* %SYS_rt_sigprocmask_set_b to i64*
            %SYS_rt_sigprocmask_oset_b = call i8* @.get_memory_ptr(i64 %arg3)
            %SYS_rt_sigprocmask_oset = bitcast i8* %SYS_rt_sigprocmask_oset_b to i64*
            %SYS_rt_sigprocmask_rslt = call i64 (i64, ...) @syscall(i64 14, i32 %SYS_rt_sigprocmask_how, i64* %SYS_rt_sigprocmask_set, i64* %SYS_rt_sigprocmask_oset, i64 %arg4)
            ret i64 %SYS_rt_sigprocmask_rslt"
        ),
        (
            "ioctl",
            29,
            "  %SYS_ioctl_fd = trunc i64 %arg1 to i32
            %SYS_ioctl_cmd = trunc i64 %arg2 to i32
            %SYS_ioctl_rslt = call i64 (i64, ...) @syscall(i64 16, i32 %SYS_ioctl_fd, i32 %SYS_ioctl_cmd, i64 %arg3)
            ret i64 %SYS_ioctl_rslt"
        ),
        (
            "getrlimit",
            163,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "setrlimit",
            164,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "getrusage",
            165,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "clock_gettime",
            113,
            "  %which_clock = trunc i64 %arg1 to i32
            %tp_b = call i8* @.get_memory_ptr(i64 %arg2)
            %tp = bitcast i8* %tp_b to %struct.timespec*
            %SYS_clock_gettime_rslt = call i64 (i64, ...) @syscall(i64 228, i32 %which_clock, %struct.timespec* %tp)
            ret i64 %SYS_clock_gettime_rslt"
        ),
        (
            "set_tid_address",
            96,
            "  %SYS_set_tid_address_tidptr_b = call i8* @.get_memory_ptr(i64 %arg1)
            %SYS_set_tid_address_tidptr = bitcast i8* %SYS_set_tid_address_tidptr_b to i32*
            %SYS_set_tid_address_rslt = call i64 (i64, ...) @syscall(i64 218, i32* %SYS_set_tid_address_tidptr)
            ret i64 %SYS_set_tid_address_rslt"
        ),
        (
            "set_robust_list",
            99,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "madvise",
            233,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "statx",
            291,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "open",
            1024,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "link",
            1025,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "unlink",
            1026,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "mkdir",
            1030,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "access",
            1033,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "stat",
            1038,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "lstat",
            1039,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
        (
            "time",
            1062,
            "  %rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)"
        ),
    ];
}
