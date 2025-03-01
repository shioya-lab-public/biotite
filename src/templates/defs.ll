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

; Function Attrs: nofree norecurse nosync nounwind memory(readwrite, inaccessiblemem: none) uwtable
define dso_local void @.mem_copy(ptr nocapture noundef writeonly %0, ptr nocapture noundef readonly %1, i64 noundef %2) alwaysinline {
  %4 = icmp sgt i64 %2, 0
  br i1 %4, label %6, label %5

5:                                                ; preds = %6, %3
  ret void

6:                                                ; preds = %3, %6
  %7 = phi i64 [ %13, %6 ], [ 0, %3 ]
  %8 = phi ptr [ %12, %6 ], [ %0, %3 ]
  %9 = phi ptr [ %10, %6 ], [ %1, %3 ]
  %10 = getelementptr inbounds i8, ptr %9, i64 1
  %11 = load i8, ptr %9, align 1
  %12 = getelementptr inbounds i8, ptr %8, i64 1
  store i8 %11, ptr %8, align 1
  %13 = add nuw nsw i64 %7, 1
  %14 = icmp eq i64 %13, %2
  br i1 %14, label %5, label %6
}

; Function Attrs: nofree norecurse nosync nounwind memory(readwrite, inaccessiblemem: none) uwtable
define dso_local nonnull ptr @.copy_envp(ptr nocapture noundef readonly %0, ptr noundef writeonly %1) {
  %3 = load ptr, ptr %0, align 8
  %4 = icmp eq ptr %3, null
  br i1 %4, label %13, label %5

5:                                                ; preds = %2, %5
  %6 = phi ptr [ %11, %5 ], [ %3, %2 ]
  %7 = phi ptr [ %10, %5 ], [ %1, %2 ]
  %8 = phi ptr [ %9, %5 ], [ %0, %2 ]
  %9 = getelementptr inbounds ptr, ptr %8, i64 1
  %10 = getelementptr inbounds ptr, ptr %7, i64 1
  store ptr %6, ptr %7, align 8
  %11 = load ptr, ptr %9, align 8
  %12 = icmp eq ptr %11, null
  br i1 %12, label %13, label %5

13:                                               ; preds = %5, %2
  %14 = phi ptr [ %1, %2 ], [ %10, %5 ]
  %15 = getelementptr inbounds ptr, ptr %14, i64 1
  ret ptr %15
}

; Function Attrs: nounwind
declare i64 @getauxval(i64 noundef)
%struct.Elf64_Phdr = type { i32, i32, i64, i64, i64, i64, i64, i64 }
@.entries = private constant [23 x i64] [i64 0, i64 1, i64 2, i64 4, i64 5, i64 6, i64 7, i64 8, i64 9, i64 10, i64 11, i64 12, i64 13, i64 14, i64 15, i64 16, i64 17, i64 23, i64 24, i64 25, i64 26, i64 31, i64 51], align 16

; Function Attrs: nounwind uwtable
define dso_local void @.init_auxv(ptr nocapture noundef writeonly %0, ptr nocapture noundef writeonly %1, i64 noundef %2, i64 noundef %3, i64 noundef %4) {
  %6 = tail call i64 @getauxval(i64 noundef 3)
  %7 = tail call i64 @getauxval(i64 noundef 5)
  %8 = icmp ne i64 %6, 0
  %9 = icmp ne i64 %7, 0
  %10 = select i1 %8, i1 %9, i1 false
  br i1 %10, label %13, label %11

11:                                               ; preds = %17, %5
  %12 = phi ptr [ %0, %5 ], [ %19, %17 ]
  br label %36

13:                                               ; preds = %5
  %14 = icmp sgt i64 %7, 0
  br i1 %14, label %15, label %17

15:                                               ; preds = %13
  %16 = inttoptr i64 %6 to ptr
  br label %20

17:                                               ; preds = %30, %13
  %18 = getelementptr inbounds i64, ptr %0, i64 1
  store i64 3, ptr %0, align 8
  %19 = getelementptr inbounds i64, ptr %0, i64 2
  store i64 %2, ptr %18, align 8
  br label %11

20:                                               ; preds = %15, %30
  %21 = phi ptr [ %32, %30 ], [ %16, %15 ]
  %22 = phi i64 [ %33, %30 ], [ 0, %15 ]
  %23 = phi ptr [ %31, %30 ], [ %1, %15 ]
  %24 = load i32, ptr %21, align 8
  switch i32 %24, label %29 [
    i32 7, label %25
    i32 1685382482, label %25
  ]

25:                                               ; preds = %20, %20
  tail call void @.mem_copy(ptr noundef nonnull align 8 dereferenceable(56) %23, ptr noundef nonnull align 8 dereferenceable(56) %21, i64 56)
  %26 = getelementptr inbounds %struct.Elf64_Phdr, ptr %23, i64 0, i32 3
  store i64 %3, ptr %26, align 8
  %27 = getelementptr inbounds %struct.Elf64_Phdr, ptr %23, i64 0, i32 5
  store i64 %4, ptr %27, align 8
  %28 = getelementptr inbounds %struct.Elf64_Phdr, ptr %23, i64 0, i32 6
  store i64 %4, ptr %28, align 8
  br label %30

29:                                               ; preds = %20
  tail call void @.mem_copy(ptr noundef nonnull align 8 dereferenceable(56) %23, ptr noundef nonnull align 8 dereferenceable(56) %21, i64 56)
  br label %30

30:                                               ; preds = %25, %29
  %31 = getelementptr inbounds %struct.Elf64_Phdr, ptr %23, i64 1
  %32 = getelementptr inbounds %struct.Elf64_Phdr, ptr %21, i64 1
  %33 = add nuw nsw i64 %22, 1
  %34 = icmp eq i64 %33, %7
  br i1 %34, label %17, label %20

35:                                               ; preds = %46
  ret void

36:                                               ; preds = %11, %46
  %37 = phi ptr [ %47, %46 ], [ %12, %11 ]
  %38 = phi i64 [ %48, %46 ], [ 0, %11 ]
  %39 = getelementptr inbounds [23 x i64], ptr @.entries, i64 0, i64 %38
  %40 = load i64, ptr %39, align 8
  %41 = tail call i64 @getauxval(i64 noundef %40)
  %42 = icmp eq i64 %41, 0
  br i1 %42, label %46, label %43

43:                                               ; preds = %36
  %44 = getelementptr inbounds i64, ptr %37, i64 1
  store i64 %40, ptr %37, align 8
  %45 = getelementptr inbounds i64, ptr %37, i64 2
  store i64 %41, ptr %44, align 8
  br label %46

46:                                               ; preds = %43, %36
  %47 = phi ptr [ %45, %43 ], [ %37, %36 ]
  %48 = add nuw nsw i64 %38, 1
  %49 = icmp eq i64 %48, 23
  br i1 %49, label %35, label %36
}
