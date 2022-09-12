declare i64 @syscall(i64, ...)

%struct.tms = type { i64, i64, i64, i64 }

%struct.stat = type { i64, i64, i64, i32, i32, i32, i32, i64, i64, i64, i64, %struct.timespec, %struct.timespec, %struct.timespec, [3 x i64] }
%struct.timespec = type { i64, i64 }

define i64 @.system_call(i64 %nr, i64 %arg1, i64 %arg2, i64 %arg3, i64 %arg4, i64 %arg5, i64 %arg6) {
  switch i64 %nr, label %fallback [
    i64 93, label %SYS_exit
    i64 169, label %SYS_gettimeofday
    i64 214, label %SYS_brk
    i64 57, label %SYS_close
    i64 80, label %SYS_fstat
    i64 62, label %SYS_lseek
    i64 63, label %SYS_read
    i64 64, label %SYS_write
  ]

SYS_exit:
  %SYS_exit_rslt = call i64 (i64, ...) @syscall(i64 60, i64 %arg1)
  ret i64 %SYS_exit_rslt

SYS_gettimeofday:
  %tms_ptr = call i8* @.get_memory_ptr(i64 %arg1)
  %tms = bitcast i8* %tms_ptr to %struct.tms*
  %SYS_gettimeofday_rslt = call i64 (i64, ...) @syscall(i64 96, %struct.tms* %tms, i64 %arg2)
  ret i64 %SYS_gettimeofday_rslt

SYS_brk:
  %SYS_brk_rslt = call i64 (i64, ...) @syscall(i64 12, i64 %arg1)
  ret i64 %SYS_brk_rslt

SYS_close:
  %SYS_close_rslt = call i64 (i64, ...) @syscall(i64 3, i64 %arg1)
  ret i64 %SYS_close_rslt

SYS_fstat:
  %stat_ptr = call i8* @.get_memory_ptr(i64 %arg2)
  %stat = bitcast i8* %stat_ptr to %struct.stat*
  %SYS_fstat_rslt = call i64 (i64, ...) @syscall(i64 5, i64 %arg1, %struct.stat* %stat)
  ret i64 %SYS_fstat_rslt

SYS_lseek:
  %SYS_lseek_rslt = call i64 (i64, ...) @syscall(i64 8, i64 %arg1, i64 %arg2, i64 %arg3)
  ret i64 %SYS_lseek_rslt

SYS_read:
  %read_buf = call i8* @.get_memory_ptr(i64 %arg2)
  %SYS_read_rslt = call i64 (i64, ...) @syscall(i64 0, i64 %arg1, i8* %read_buf, i64 %arg3)
  ret i64 %SYS_read_rslt

SYS_write:
  %write_buf = call i8* @.get_memory_ptr(i64 %arg2)
  %SYS_write_rslt = call i64 (i64, ...) @syscall(i64 1, i64 %arg1, i8* %write_buf, i64 %arg3)
  ret i64 %SYS_write_rslt

fallback:
  unreachable
}
