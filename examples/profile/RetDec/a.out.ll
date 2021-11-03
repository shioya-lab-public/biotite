source_filename = "test"
target datalayout = "e-m:e-p:64:64-i64:64-f80:128-n8:16:32:64-S128"

@0 = external local_unnamed_addr global i64
@global_var_403ff8 = local_unnamed_addr global i64 0
@global_var_402008 = local_unnamed_addr constant i64 4198714
@global_var_403e50 = local_unnamed_addr global i64 4198656
@global_var_403e58 = local_unnamed_addr global i64 4198608
@global_var_404028 = local_unnamed_addr global i8 0

define i64 @f(i64 %arg1) local_unnamed_addr {
dec_label_pc_401110:
  %stack_var_-12.0.in.reg2mem = alloca i64, !insn.addr !0
  %sext = mul i64 %arg1, 4294967296
  %0 = add i64 %arg1, 4294967295, !insn.addr !1
  %1 = and i64 %0, 4294967295, !insn.addr !2
  store i64 %1, i64* @0, align 8, !insn.addr !3
  %trunc = trunc i64 %0 to i32
  store i64 %sext, i64* %stack_var_-12.0.in.reg2mem, !insn.addr !4
  switch i32 %trunc, label %dec_label_pc_40117b [
    i32 0, label %dec_label_pc_40113a
    i32 1, label %dec_label_pc_401148
    i32 2, label %dec_label_pc_401156
    i32 3, label %dec_label_pc_401164
    i32 4, label %dec_label_pc_401172
  ], !insn.addr !4

dec_label_pc_40113a:                              ; preds = %dec_label_pc_401110
  %sext6 = add i64 %sext, 4294967296
  store i64 %sext6, i64* %stack_var_-12.0.in.reg2mem, !insn.addr !5
  br label %dec_label_pc_40117b, !insn.addr !5

dec_label_pc_401148:                              ; preds = %dec_label_pc_401110
  %sext5 = add i64 %sext, 8589934592
  store i64 %sext5, i64* %stack_var_-12.0.in.reg2mem, !insn.addr !6
  br label %dec_label_pc_40117b, !insn.addr !6

dec_label_pc_401156:                              ; preds = %dec_label_pc_401110
  %sext4 = add i64 %sext, 12884901888
  store i64 %sext4, i64* %stack_var_-12.0.in.reg2mem, !insn.addr !7
  br label %dec_label_pc_40117b, !insn.addr !7

dec_label_pc_401164:                              ; preds = %dec_label_pc_401110
  %sext3 = add i64 %sext, 17179869184
  store i64 %sext3, i64* %stack_var_-12.0.in.reg2mem, !insn.addr !8
  br label %dec_label_pc_40117b, !insn.addr !8

dec_label_pc_401172:                              ; preds = %dec_label_pc_401110
  %sext2 = add i64 %sext, 21474836480
  store i64 %sext2, i64* %stack_var_-12.0.in.reg2mem, !insn.addr !9
  br label %dec_label_pc_40117b, !insn.addr !9

dec_label_pc_40117b:                              ; preds = %dec_label_pc_401172, %dec_label_pc_401164, %dec_label_pc_401156, %dec_label_pc_401148, %dec_label_pc_40113a, %dec_label_pc_401110
  %stack_var_-12.0.in.reload = load i64, i64* %stack_var_-12.0.in.reg2mem
  %2 = udiv i64 %stack_var_-12.0.in.reload, 4294967296
  ret i64 %2, !insn.addr !10
}

define i64 @main(i64 %argc, i8** %argv) local_unnamed_addr {
dec_label_pc_401180:
  %storemerge1.reg2mem = alloca i32, !insn.addr !11
  store i32 0, i32* %storemerge1.reg2mem
  br label %dec_label_pc_4011a3

dec_label_pc_4011a3:                              ; preds = %dec_label_pc_401180, %dec_label_pc_4011a3
  %storemerge1.reload = load i32, i32* %storemerge1.reg2mem
  %0 = call i64 @f(i64 1), !insn.addr !12
  %1 = add nuw nsw i32 %storemerge1.reload, 1, !insn.addr !13
  %exitcond = icmp eq i32 %1, 100000000
  store i32 %1, i32* %storemerge1.reg2mem, !insn.addr !14
  br i1 %exitcond, label %dec_label_pc_4011bb, label %dec_label_pc_4011a3, !insn.addr !14

dec_label_pc_4011bb:                              ; preds = %dec_label_pc_4011a3
  ret i64 0, !insn.addr !15
}

!0 = !{i64 4198672}
!1 = !{i64 4198682}
!2 = !{i64 4198685}
!3 = !{i64 4198704}
!4 = !{i64 4198712}
!5 = !{i64 4198723}
!6 = !{i64 4198737}
!7 = !{i64 4198751}
!8 = !{i64 4198765}
!9 = !{i64 4198776}
!10 = !{i64 4198783}
!11 = !{i64 4198784}
!12 = !{i64 4198824}
!13 = !{i64 4198832}
!14 = !{i64 4198813}
!15 = !{i64 4198850}
