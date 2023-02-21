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

define void @.mem_copy(i8* %0, i8* %1, i64 %2) {
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

declare i64 @getauxval(i64)
%struct.Elf64_Phdr = type { i32, i32, i64, i64, i64, i64, i64, i64 }
@.entries = constant [23 x i64] [i64 0, i64 1, i64 2, i64 4, i64 5, i64 6, i64 7, i64 8, i64 9, i64 10, i64 11, i64 12, i64 13, i64 14, i64 15, i64 16, i64 17, i64 23, i64 24, i64 25, i64 26, i64 31, i64 51]

define void @.init_auxv(i64* %0, i8* %1, i64 %2, i64 %3) {
  %5 = call i64 @getauxval(i64 3)
  %6 = call i64 @getauxval(i64 5)
  %7 = icmp ne i64 %5, 0
  %8 = icmp ne i64 %6, 0
  %9 = select i1 %7, i1 %8, i1 false
  br i1 %9, label %12, label %10

10:                                               ; preds = %17, %4
  %11 = phi i64* [ %0, %4 ], [ %19, %17 ]
  br label %44

12:                                               ; preds = %4
  %13 = icmp sgt i64 %6, 0
  br i1 %13, label %14, label %17

14:                                               ; preds = %12
  %15 = bitcast i8* %1 to %struct.Elf64_Phdr*
  %16 = inttoptr i64 %5 to %struct.Elf64_Phdr*
  br label %20

17:                                               ; preds = %38, %12
  %18 = getelementptr i64, i64* %0, i64 1
  store i64 3, i64* %0
  %19 = getelementptr i64, i64* %0, i64 2
  store i64 %2, i64* %18
  br label %10

20:                                               ; preds = %14, %38
  %21 = phi %struct.Elf64_Phdr* [ %40, %38 ], [ %16, %14 ]
  %22 = phi i64 [ %41, %38 ], [ 0, %14 ]
  %23 = phi %struct.Elf64_Phdr* [ %39, %38 ], [ %15, %14 ]
  %24 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %21, i64 0, i32 0
  %25 = load i32, i32* %24
  switch i32 %25, label %35 [
    i32 7, label %26
    i32 1685382482, label %30
  ]

26:                                               ; preds = %20
  %27 = bitcast %struct.Elf64_Phdr* %23 to i8*
  %28 = bitcast %struct.Elf64_Phdr* %21 to i8*
  call void @.mem_copy(i8* %27, i8* %28, i64 56)
  %29 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %23, i64 0, i32 3
  store i64 %3, i64* %29
  br label %38

30:                                               ; preds = %20
  %31 = bitcast %struct.Elf64_Phdr* %23 to i8*
  %32 = bitcast %struct.Elf64_Phdr* %21 to i8*
  call void @.mem_copy(i8* %31, i8* %32, i64 56)
  %33 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %23, i64 0, i32 3
  store i64 %3, i64* %33
  %34 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %23, i64 0, i32 6
  store i64 2760, i64* %34
  br label %38

35:                                               ; preds = %20
  %36 = bitcast %struct.Elf64_Phdr* %23 to i8*
  %37 = bitcast %struct.Elf64_Phdr* %21 to i8*
  call void @.mem_copy(i8* %36, i8* %37, i64 56)
  br label %38

38:                                               ; preds = %26, %35, %30
  %39 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %23, i64 1
  %40 = getelementptr %struct.Elf64_Phdr, %struct.Elf64_Phdr* %21, i64 1
  %41 = add i64 %22, 1
  %42 = icmp eq i64 %41, %6
  br i1 %42, label %17, label %20

43:                                               ; preds = %54
  ret void

44:                                               ; preds = %10, %54
  %45 = phi i64* [ %55, %54 ], [ %11, %10 ]
  %46 = phi i64 [ %56, %54 ], [ 0, %10 ]
  %47 = getelementptr [23 x i64], [23 x i64]* @.entries, i64 0, i64 %46
  %48 = load i64, i64* %47
  %49 = call i64 @getauxval(i64 %48)
  %50 = icmp eq i64 %49, 0
  br i1 %50, label %54, label %51

51:                                               ; preds = %44
  %52 = getelementptr i64, i64* %45, i64 1
  store i64 %48, i64* %45
  %53 = getelementptr i64, i64* %45, i64 2
  store i64 %49, i64* %52
  br label %54

54:                                               ; preds = %51, %44
  %55 = phi i64* [ %53, %51 ], [ %45, %44 ]
  %56 = add i64 %46, 1
  %57 = icmp eq i64 %56, 23
  br i1 %57, label %43, label %44
}
