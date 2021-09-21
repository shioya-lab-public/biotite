; ModuleID = 'examples/test.c'
source_filename = "examples/test.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @s(i32 %0) #0 {
  %2 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  %3 = load i32, i32* %2, align 4
  switch i32 %3, label %19 [
    i32 1, label %4
    i32 2, label %7
    i32 3, label %10
    i32 4, label %13
    i32 5, label %16
  ]

4:                                                ; preds = %1
  %5 = load i32, i32* %2, align 4
  %6 = add nsw i32 %5, 1
  store i32 %6, i32* %2, align 4
  br label %19

7:                                                ; preds = %1
  %8 = load i32, i32* %2, align 4
  %9 = add nsw i32 %8, 2
  store i32 %9, i32* %2, align 4
  br label %19

10:                                               ; preds = %1
  %11 = load i32, i32* %2, align 4
  %12 = add nsw i32 %11, 3
  store i32 %12, i32* %2, align 4
  br label %19

13:                                               ; preds = %1
  %14 = load i32, i32* %2, align 4
  %15 = add nsw i32 %14, 4
  store i32 %15, i32* %2, align 4
  br label %19

16:                                               ; preds = %1
  %17 = load i32, i32* %2, align 4
  %18 = add nsw i32 %17, 5
  store i32 %18, i32* %2, align 4
  br label %19

19:                                               ; preds = %1, %16, %13, %10, %7, %4
  %20 = load i32, i32* %2, align 4
  ret i32 %20
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 0, i32* %1, align 4
  store i32 0, i32* %2, align 4
  store i32 0, i32* %3, align 4
  br label %5

5:                                                ; preds = %32, %0
  %6 = load i32, i32* %3, align 4
  %7 = icmp slt i32 %6, 100000000
  br i1 %7, label %8, label %35

8:                                                ; preds = %5
  store i32 0, i32* %2, align 4
  store i32 0, i32* %4, align 4
  br label %9

9:                                                ; preds = %15, %8
  %10 = load i32, i32* %4, align 4
  %11 = icmp slt i32 %10, 1
  br i1 %11, label %12, label %18

12:                                               ; preds = %9
  %13 = load i32, i32* %2, align 4
  %14 = add nsw i32 %13, 1
  store i32 %14, i32* %2, align 4
  br label %15

15:                                               ; preds = %12
  %16 = load i32, i32* %4, align 4
  %17 = add nsw i32 %16, 1
  store i32 %17, i32* %4, align 4
  br label %9, !llvm.loop !2

18:                                               ; preds = %9
  br label %19

19:                                               ; preds = %22, %18
  %20 = load i32, i32* %2, align 4
  %21 = icmp slt i32 %20, 2
  br i1 %21, label %22, label %25

22:                                               ; preds = %19
  %23 = load i32, i32* %2, align 4
  %24 = add nsw i32 %23, 1
  store i32 %24, i32* %2, align 4
  br label %19, !llvm.loop !4

25:                                               ; preds = %19
  br label %26

26:                                               ; preds = %25
  %27 = load i32, i32* %2, align 4
  %28 = add nsw i32 %27, 1
  store i32 %28, i32* %2, align 4
  br label %29

29:                                               ; preds = %26
  %30 = load i32, i32* %2, align 4
  %31 = call i32 @s(i32 %30)
  store i32 %31, i32* %2, align 4
  br label %32

32:                                               ; preds = %29
  %33 = load i32, i32* %3, align 4
  %34 = add nsw i32 %33, 1
  store i32 %34, i32* %3, align 4
  br label %5, !llvm.loop !5

35:                                               ; preds = %5
  %36 = load i32, i32* %2, align 4
  ret i32 %36
}

attributes #0 = { noinline nounwind optnone uwtable "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{!"clang version 12.0.0 (https://github.com/llvm/llvm-project/ b978a93635b584db380274d7c8963c73989944a1)"}
!2 = distinct !{!2, !3}
!3 = !{!"llvm.loop.mustprogress"}
!4 = distinct !{!4, !3}
!5 = distinct !{!5, !3}
