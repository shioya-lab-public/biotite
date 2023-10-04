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

declare float @llvm.sqrt.float(float %arg)
declare double @llvm.sqrt.double(double %arg)
declare float @llvm.fma.float(float %arg1, float %arg2, float %arg3)
declare double @llvm.fma.double(double %arg1, double %arg2, double %arg3)
declare float @llvm.fabs.float(float %arg)
declare double @llvm.fabs.double(double %arg)
declare float @llvm.copysign.float(float %mag, float %sgn)
declare double @llvm.copysign.double(double %mag, double %sgn)

define void @.mem_copy(i8* %0, i8* %1, i64 %2) alwaysinline {
  %4 = icmp sgt i64 %2, 0
  br i1 %4, label %6, label %5

5:                                                ; preds = %6, %3
  ret void

6:                                                ; preds = %3, %6
  %7 = phi i64 [ %13, %6 ], [ 0, %3 ]
  %8 = phi i8* [ %12, %6 ], [ %0, %3 ]
  %9 = phi i8* [ %10, %6 ], [ %1, %3 ]
  %10 = getelementptr i8, i8* %9, i64 1
  %11 = load i8, i8* %9
  %12 = getelementptr i8, i8* %8, i64 1
  store i8 %11, i8* %8
  %13 = add i64 %7, 1
  %14 = icmp eq i64 %13, %2
  br i1 %14, label %5, label %6
}

define i8* @.copy_envp(i8** %0, i8** %1) {
  %3 = load i8*, i8** %0
  %4 = icmp eq i8* %3, null
  br i1 %4, label %13, label %5

5:                                                ; preds = %2, %5
  %6 = phi i8* [ %11, %5 ], [ %3, %2 ]
  %7 = phi i8** [ %10, %5 ], [ %1, %2 ]
  %8 = phi i8** [ %9, %5 ], [ %0, %2 ]
  %9 = getelementptr i8*, i8** %8, i64 1
  %10 = getelementptr i8*, i8** %7, i64 1
  store i8* %6, i8** %7
  %11 = load i8*, i8** %9
  %12 = icmp eq i8* %11, null
  br i1 %12, label %13, label %5

13:                                               ; preds = %5, %2
  %14 = phi i8** [ %1, %2 ], [ %10, %5 ]
  %15 = getelementptr i8*, i8** %14, i64 1
  %16 = bitcast i8** %15 to i8*
  ret i8* %16
}

declare i64 @getauxval(i64)
%struct.Elf64_Phdr = type { i32, i32, i64, i64, i64, i64, i64, i64 }
@.entries = constant [23 x i64] [i64 0, i64 1, i64 2, i64 4, i64 5, i64 6, i64 7, i64 8, i64 9, i64 10, i64 11, i64 12, i64 13, i64 14, i64 15, i64 16, i64 17, i64 23, i64 24, i64 25, i64 26, i64 31, i64 51]

define void @.init_auxv(i64* %0, i8* %1, i64 %2, i64 %3, i64 %4) {
  %6 = call i64 @getauxval(i64 3)
  %7 = call i64 @getauxval(i64 5)
  %8 = icmp ne i64 %6, 0
  %9 = icmp ne i64 %7, 0
  %10 = select i1 %8, i1 %9, i1 false
  br i1 %10, label %13, label %11

11:                                               ; preds = %18, %5
  %12 = phi i64* [ %0, %5 ], [ %20, %18 ]
  br label %42

13:                                               ; preds = %5
  %14 = icmp sgt i64 %7, 0
  br i1 %14, label %15, label %18

15:                                               ; preds = %13
  %16 = bitcast i8* %1 to %struct.Elf64_Phdr*
  %17 = inttoptr i64 %6 to %struct.Elf64_Phdr*
  br label %21

18:                                               ; preds = %36, %13
  %19 = getelementptr i64, i64* %0, i64 1
  store i64 3, i64* %0
  %20 = getelementptr i64, i64* %0, i64 2
  store i64 %2, i64* %19
  br label %11

21:                                               ; preds = %15, %36
  %22 = phi %struct.Elf64_Phdr* [ %38, %36 ], [ %17, %15 ]
  %23 = phi i64 [ %39, %36 ], [ 0, %15 ]
  %24 = phi %struct.Elf64_Phdr* [ %37, %36 ], [ %16, %15 ]
  %25 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %22, i64 0, i32 0
  %26 = load i32, i32* %25
  switch i32 %26, label %33 [
    i32 7, label %27
    i32 1685382482, label %27
  ]

27:                                               ; preds = %21, %21
  %28 = bitcast %struct.Elf64_Phdr* %24 to i8*
  %29 = bitcast %struct.Elf64_Phdr* %22 to i8*
  call void @.mem_copy(i8* %28, i8* %29, i64 56)
  %30 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %24, i64 0, i32 3
  store i64 %3, i64* %30
  %31 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %24, i64 0, i32 5
  store i64 %4, i64* %31
  %32 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %24, i64 0, i32 6
  store i64 %4, i64* %32
  br label %36

33:                                               ; preds = %21
  %34 = bitcast %struct.Elf64_Phdr* %24 to i8*
  %35 = bitcast %struct.Elf64_Phdr* %22 to i8*
  call void @.mem_copy(i8* %34, i8* %35, i64 56)
  br label %36

36:                                               ; preds = %27, %33
  %37 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %24, i64 1
  %38 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %22, i64 1
  %39 = add i64 %23, 1
  %40 = icmp eq i64 %39, %7
  br i1 %40, label %18, label %21

41:                                               ; preds = %52
  ret void

42:                                               ; preds = %11, %52
  %43 = phi i64* [ %53, %52 ], [ %12, %11 ]
  %44 = phi i64 [ %54, %52 ], [ 0, %11 ]
  %45 = getelementptr [23 x i64], [23 x i64]* @.entries, i64 0, i64 %44
  %46 = load i64, i64* %45
  %47 = call i64 @getauxval(i64 %46)
  %48 = icmp eq i64 %47, 0
  br i1 %48, label %52, label %49

49:                                               ; preds = %42
  %50 = getelementptr i64, i64* %43, i64 1
  store i64 %46, i64* %43
  %51 = getelementptr i64, i64* %43, i64 2
  store i64 %47, i64* %50
  br label %52

52:                                               ; preds = %49, %42
  %53 = phi i64* [ %51, %49 ], [ %43, %42 ]
  %54 = add i64 %44, 1
  %55 = icmp eq i64 %54, 23
  br i1 %55, label %41, label %42
}
