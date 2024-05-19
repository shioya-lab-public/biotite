; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i32 @.rounding_float_i32_fptosi_sitofp(float, i1)
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i32 @.rounding_float_i32_fptoui_uitofp(float, i1)
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i64 @.rounding_float_i64_fptosi_sitofp(float, i1)
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i64 @.rounding_float_i64_fptoui_uitofp(float, i1)
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i32 @.rounding_double_i32_fptosi_sitofp(double, i1)
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i32 @.rounding_double_i32_fptoui_uitofp(double, i1)
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i64 @.rounding_double_i64_fptosi_sitofp(double, i1)
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
declare i64 @.rounding_double_i64_fptoui_uitofp(double, i1)

@.zero = external global i64
@.ra = external global i64
@.sp = external global i64
@.gp = external global i64
@.tp = external global i64
@.t0 = external global i64
@.t1 = external global i64
@.t2 = external global i64
@.s0 = external global i64
@.s1 = external global i64
@.a0 = external global i64
@.a1 = external global i64
@.a2 = external global i64
@.a3 = external global i64
@.a4 = external global i64
@.a5 = external global i64
@.a6 = external global i64
@.a7 = external global i64
@.s2 = external global i64
@.s3 = external global i64
@.s4 = external global i64
@.s5 = external global i64
@.s6 = external global i64
@.s7 = external global i64
@.s8 = external global i64
@.s9 = external global i64
@.s10 = external global i64
@.s11 = external global i64
@.t3 = external global i64
@.t4 = external global i64
@.t5 = external global i64
@.t6 = external global i64

@.ft0 = external global double
@.ft1 = external global double
@.ft2 = external global double
@.ft3 = external global double
@.ft4 = external global double
@.ft5 = external global double
@.ft6 = external global double
@.ft7 = external global double
@.fs0 = external global double
@.fs1 = external global double
@.fa0 = external global double
@.fa1 = external global double
@.fa2 = external global double
@.fa3 = external global double
@.fa4 = external global double
@.fa5 = external global double
@.fa6 = external global double
@.fa7 = external global double
@.fs2 = external global double
@.fs3 = external global double
@.fs4 = external global double
@.fs5 = external global double
@.fs6 = external global double
@.fs7 = external global double
@.fs8 = external global double
@.fs9 = external global double
@.fs10 = external global double
@.fs11 = external global double
@.ft8 = external global double
@.ft9 = external global double
@.ft10 = external global double
@.ft11 = external global double

@.rs = external global i64

declare float @llvm.sqrt.float(float)
declare double @llvm.sqrt.double(double)
declare float @llvm.fma.float(float, float, float)
declare double @llvm.fma.double(double, double, double)
declare float @llvm.fabs.float(float)
declare double @llvm.fabs.double(double)
declare float @llvm.copysign.float(float, float)
declare double @llvm.copysign.double(double, double)

declare i64 @.sys_call(i64, i64, i64, i64, i64, i64, i64)
