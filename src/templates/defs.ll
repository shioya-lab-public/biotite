@.zero = global i64 zeroinitializer
@.ra = global i64 zeroinitializer
@.sp = global i64 zeroinitializer
@.gp = global i64 zeroinitializer
@.tp = global i64 zeroinitializer
@.t0 = global i64 zeroinitializer
@.t1 = global i64 zeroinitializer
@.t2 = global i64 zeroinitializer
@.s0 = global i64 zeroinitializer
@.s1 = global i64 zeroinitializer
@.a0 = global i64 zeroinitializer
@.a1 = global i64 zeroinitializer
@.a2 = global i64 zeroinitializer
@.a3 = global i64 zeroinitializer
@.a4 = global i64 zeroinitializer
@.a5 = global i64 zeroinitializer
@.a6 = global i64 zeroinitializer
@.a7 = global i64 zeroinitializer
@.s2 = global i64 zeroinitializer
@.s3 = global i64 zeroinitializer
@.s4 = global i64 zeroinitializer
@.s5 = global i64 zeroinitializer
@.s6 = global i64 zeroinitializer
@.s7 = global i64 zeroinitializer
@.s8 = global i64 zeroinitializer
@.s9 = global i64 zeroinitializer
@.s10 = global i64 zeroinitializer
@.s11 = global i64 zeroinitializer
@.t3 = global i64 zeroinitializer
@.t4 = global i64 zeroinitializer
@.t5 = global i64 zeroinitializer
@.t6 = global i64 zeroinitializer

@.ft0 = global double zeroinitializer
@.ft1 = global double zeroinitializer
@.ft2 = global double zeroinitializer
@.ft3 = global double zeroinitializer
@.ft4 = global double zeroinitializer
@.ft5 = global double zeroinitializer
@.ft6 = global double zeroinitializer
@.ft7 = global double zeroinitializer
@.fs0 = global double zeroinitializer
@.fs1 = global double zeroinitializer
@.fa0 = global double zeroinitializer
@.fa1 = global double zeroinitializer
@.fa2 = global double zeroinitializer
@.fa3 = global double zeroinitializer
@.fa4 = global double zeroinitializer
@.fa5 = global double zeroinitializer
@.fa6 = global double zeroinitializer
@.fa7 = global double zeroinitializer
@.fs2 = global double zeroinitializer
@.fs3 = global double zeroinitializer
@.fs4 = global double zeroinitializer
@.fs5 = global double zeroinitializer
@.fs6 = global double zeroinitializer
@.fs7 = global double zeroinitializer
@.fs8 = global double zeroinitializer
@.fs9 = global double zeroinitializer
@.fs10 = global double zeroinitializer
@.fs11 = global double zeroinitializer
@.ft8 = global double zeroinitializer
@.ft9 = global double zeroinitializer
@.ft10 = global double zeroinitializer
@.ft11 = global double zeroinitializer

@.rs = global i64 zeroinitializer

declare float @llvm.sqrt.float(float)
declare double @llvm.sqrt.double(double)
declare float @llvm.fma.float(float, float, float)
declare double @llvm.fma.double(double, double, double)
declare float @llvm.fabs.float(float)
declare double @llvm.fabs.double(double)
declare float @llvm.copysign.float(float, float)
declare double @llvm.copysign.double(double, double)

define void @.mem_copy(ptr %0, ptr %1, i64 %2) {
  %4 = icmp sgt i64 %2, 0
  br i1 %4, label %6, label %5

5:                                                ; preds = %6, %3
  ret void

6:                                                ; preds = %3, %6
  %7 = phi i64 [ %13, %6 ], [ 0, %3 ]
  %8 = phi ptr [ %12, %6 ], [ %0, %3 ]
  %9 = phi ptr [ %10, %6 ], [ %1, %3 ]
  %10 = getelementptr i8, ptr %9, i64 1
  %11 = load i8, ptr %9, align 1
  %12 = getelementptr i8, ptr %8, i64 1
  store i8 %11, ptr %8, align 1
  %13 = add i64 %7, 1
  %14 = icmp eq i64 %13, %2
  br i1 %14, label %5, label %6
}

define ptr @.copy_envp(ptr %0, ptr %1) {
  %3 = load ptr, ptr %0, align 8
  %4 = icmp eq ptr %3, null
  br i1 %4, label %13, label %5

5:                                                ; preds = %2, %5
  %6 = phi ptr [ %11, %5 ], [ %3, %2 ]
  %7 = phi ptr [ %10, %5 ], [ %1, %2 ]
  %8 = phi ptr [ %9, %5 ], [ %0, %2 ]
  %9 = getelementptr ptr, ptr %8, i64 1
  %10 = getelementptr ptr, ptr %7, i64 1
  store ptr %6, ptr %7, align 8
  %11 = load ptr, ptr %9, align 8
  %12 = icmp eq ptr %11, null
  br i1 %12, label %13, label %5

13:                                               ; preds = %5, %2
  %14 = phi ptr [ %1, %2 ], [ %10, %5 ]
  %15 = getelementptr ptr, ptr %14, i64 1
  ret ptr %15
}

%struct.Elf64_Phdr = type { i32, i32, i64, i64, i64, i64, i64, i64 }
@.entries = private constant [23 x i64] [i64 0, i64 1, i64 2, i64 4, i64 5, i64 6, i64 7, i64 8, i64 9, i64 10, i64 11, i64 12, i64 13, i64 14, i64 15, i64 16, i64 17, i64 23, i64 24, i64 25, i64 26, i64 31, i64 51], align 16
declare i64 @getauxval(i64)

define void @.init_auxv(ptr %0, ptr %1, i64 %2, i64 %3, i64 %4) {
  %6 = icmp eq i64 %3, 0
  br i1 %6, label %51, label %7

7:                                                ; preds = %5
  %8 = call i64 @getauxval(i64 3)
  %9 = call i64 @getauxval(i64 5)
  %10 = icmp ne i64 %8, 0
  %11 = icmp ne i64 %9, 0
  %12 = select i1 %10, i1 %11, i1 false
  br i1 %12, label %15, label %13

13:                                               ; preds = %19, %7
  %14 = phi ptr [ %0, %7 ], [ %21, %19 ]
  br label %37

15:                                               ; preds = %7
  %16 = icmp sgt i64 %9, 0
  br i1 %16, label %17, label %19

17:                                               ; preds = %15
  %18 = inttoptr i64 %8 to ptr
  br label %22

19:                                               ; preds = %32, %15
  %20 = getelementptr i64, ptr %0, i64 1
  store i64 3, ptr %0, align 8
  %21 = getelementptr i64, ptr %0, i64 2
  store i64 %2, ptr %20, align 8
  br label %13

22:                                               ; preds = %17, %32
  %23 = phi ptr [ %33, %32 ], [ %18, %17 ]
  %24 = phi i64 [ %35, %32 ], [ 0, %17 ]
  %25 = phi ptr [ %34, %32 ], [ %1, %17 ]
  %26 = load i32, ptr %23, align 8
  switch i32 %26, label %31 [
    i32 7, label %27
    i32 1685382482, label %27
  ]

27:                                               ; preds = %22, %22
  call void @.mem_copy(ptr align 8 %25, ptr align 8 %23, i64 56)
  %28 = getelementptr %struct.Elf64_Phdr, ptr %25, i64 0, i32 3
  store i64 %3, ptr %28, align 8
  %29 = getelementptr %struct.Elf64_Phdr, ptr %25, i64 0, i32 5
  store i64 %4, ptr %29, align 8
  %30 = getelementptr %struct.Elf64_Phdr, ptr %25, i64 0, i32 6
  store i64 %4, ptr %30, align 8
  br label %32

31:                                               ; preds = %22
  call void @.mem_copy(ptr align 8 %25, ptr align 8 %23, i64 56)
  br label %32

32:                                               ; preds = %27, %31
  %33 = getelementptr %struct.Elf64_Phdr, ptr %23, i64 1
  %34 = getelementptr %struct.Elf64_Phdr, ptr %25, i64 1
  %35 = add i64 %24, 1
  %36 = icmp eq i64 %35, %9
  br i1 %36, label %19, label %22

37:                                               ; preds = %13, %47
  %38 = phi ptr [ %48, %47 ], [ %14, %13 ]
  %39 = phi i64 [ %49, %47 ], [ 0, %13 ]
  %40 = getelementptr [23 x i64], ptr @.entries, i64 0, i64 %39
  %41 = load i64, ptr %40, align 8
  %42 = call i64 @getauxval(i64 %41)
  %43 = icmp eq i64 %42, 0
  br i1 %43, label %47, label %44

44:                                               ; preds = %37
  %45 = getelementptr i64, ptr %38, i64 1
  store i64 %41, ptr %38, align 8
  %46 = getelementptr i64, ptr %38, i64 2
  store i64 %42, ptr %45, align 8
  br label %47

47:                                               ; preds = %44, %37
  %48 = phi ptr [ %46, %44 ], [ %38, %37 ]
  %49 = add i64 %39, 1
  %50 = icmp eq i64 %49, 23
  br i1 %50, label %51, label %37

51:                                               ; preds = %47, %5
  ret void
}