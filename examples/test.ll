
; Dump registers:
; declare dso_local i32 @printf(i8*, ...)
; @.str = private unnamed_addr constant [19 x i8] c"*** Debug ***: %d\0A\00", align 1
; %val = load i64, i64* @reg.zero
; call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.str, i64 0, i64 0), i64 %val)

declare float @llvm.sqrt.f32(float %value)
declare double @llvm.sqrt.f64(double %value)
declare float @llvm.fma.f32(float %a, float %b, float %c)
declare double @llvm.fma.f64(double %a, double %b, double %c)
declare float @llvm.fabs.f32(float %value)
declare double @llvm.fabs.f64(double %value)
declare float @llvm.minimum.f32(float %op1, float %op2)
declare double @llvm.minimum.f64(double %op1, double %op2)
declare float @llvm.maximum.f32(float %op1, float %op2)
declare double @llvm.maximum.f64(double %op1, double %op2)
declare float @llvm.copysign.f32(float %mag, float %sign)
declare double @llvm.copysign.f64(double %mag, double %sign)

declare dso_local i64 @syscall(i64, ...)

@reg.zero = global i64 zeroinitializer
@reg.ra = global i64 zeroinitializer
@reg.sp = global i64 10240
@reg.gp = global i64 zeroinitializer
@reg.tp = global i64 zeroinitializer
@reg.t0 = global i64 zeroinitializer
@reg.t1 = global i64 zeroinitializer
@reg.t2 = global i64 zeroinitializer
@reg.s0 = global i64 zeroinitializer
@reg.s1 = global i64 zeroinitializer
@reg.a0 = global i64 zeroinitializer
@reg.a1 = global i64 zeroinitializer
@reg.a2 = global i64 zeroinitializer
@reg.a3 = global i64 zeroinitializer
@reg.a4 = global i64 zeroinitializer
@reg.a5 = global i64 zeroinitializer
@reg.a6 = global i64 zeroinitializer
@reg.a7 = global i64 zeroinitializer
@reg.s2 = global i64 zeroinitializer
@reg.s3 = global i64 zeroinitializer
@reg.s4 = global i64 zeroinitializer
@reg.s5 = global i64 zeroinitializer
@reg.s6 = global i64 zeroinitializer
@reg.s7 = global i64 zeroinitializer
@reg.s8 = global i64 zeroinitializer
@reg.s9 = global i64 zeroinitializer
@reg.s10 = global i64 zeroinitializer
@reg.s11 = global i64 zeroinitializer
@reg.t3 = global i64 zeroinitializer
@reg.t4 = global i64 zeroinitializer
@reg.t5 = global i64 zeroinitializer
@reg.t6 = global i64 zeroinitializer

@reg.ft0 = global double zeroinitializer
@reg.ft1 = global double zeroinitializer
@reg.ft2 = global double zeroinitializer
@reg.ft3 = global double zeroinitializer
@reg.ft4 = global double zeroinitializer
@reg.ft5 = global double zeroinitializer
@reg.ft6 = global double zeroinitializer
@reg.ft7 = global double zeroinitializer
@reg.fs0 = global double zeroinitializer
@reg.fs1 = global double zeroinitializer
@reg.fa0 = global double zeroinitializer
@reg.fa1 = global double zeroinitializer
@reg.fa2 = global double zeroinitializer
@reg.fa3 = global double zeroinitializer
@reg.fa4 = global double zeroinitializer
@reg.fa5 = global double zeroinitializer
@reg.fa6 = global double zeroinitializer
@reg.fa7 = global double zeroinitializer
@reg.fs2 = global double zeroinitializer
@reg.fs3 = global double zeroinitializer
@reg.fs4 = global double zeroinitializer
@reg.fs5 = global double zeroinitializer
@reg.fs6 = global double zeroinitializer
@reg.fs7 = global double zeroinitializer
@reg.fs8 = global double zeroinitializer
@reg.fs9 = global double zeroinitializer
@reg.fs10 = global double zeroinitializer
@reg.fs11 = global double zeroinitializer
@reg.ft8 = global double zeroinitializer
@reg.ft9 = global double zeroinitializer
@reg.ft10 = global double zeroinitializer
@reg.ft11 = global double zeroinitializer

@reg.stack = global [10240 x i8] zeroinitializer

define void @f() {
Entry:
  br label %L0
L0:
  %temp_0 = load i64, i64* @reg.sp
  %temp_1 = add i64 %temp_0, -32
  store i64 %temp_1, i64* @reg.sp
  %temp_2 = load i64, i64* @reg.sp
  %temp_3 = add i64 %temp_2, 24
  %temp_4 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_3
  %temp_5 = bitcast i8* %temp_4 to i64*
  %temp_6 = load i64, i64* @reg.s0
  store i64 %temp_6, i64* %temp_5
  %temp_7 = load i64, i64* @reg.sp
  %temp_8 = add i64 %temp_7, 32
  store i64 %temp_8, i64* @reg.s0
  %temp_9 = load i64, i64* @reg.a0
  store i64 %temp_9, i64* @reg.a5
  %temp_10 = load i64, i64* @reg.s0
  %temp_11 = add i64 %temp_10, -20
  %temp_12 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_11
  %temp_13 = bitcast i8* %temp_12 to i32*
  %temp_14 = load i64, i64* @reg.a5
  %temp_15 = trunc i64 %temp_14 to i32
  store i32 %temp_15, i32* %temp_13
  %temp_16 = load i64, i64* @reg.s0
  %temp_17 = add i64 %temp_16, -20
  %temp_18 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_17
  %temp_19 = bitcast i8* %temp_18 to i32*
  %temp_20 = load i32, i32* %temp_19
  %temp_21 = sext i32 %temp_20 to i64
  store i64 %temp_21, i64* @reg.a5
  %temp_22 = load i64, i64* @reg.a5
  %temp_23 = trunc i64 %temp_22 to i32
  %temp_24 = sext i32 %temp_23 to i64
  store i64 %temp_24, i64* @reg.a4
  store i64 5, i64* @reg.a5
  %temp_25 = load i64, i64* @reg.a5
  %temp_26 = load i64, i64* @reg.a4
  %temp_27 = icmp ult i64 %temp_25, %temp_26
  br i1 %temp_27, label %L7, label %L1
L1:
  %temp_28 = load i64, i64* @reg.s0
  %temp_29 = add i64 %temp_28, -20
  %temp_30 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_29
  %temp_31 = bitcast i8* %temp_30 to i32*
  %temp_32 = load i32, i32* %temp_31
  %temp_33 = zext i32 %temp_32 to i64
  store i64 %temp_33, i64* @reg.a5
  %temp_34 = load i64, i64* @reg.a5
  %temp_35 = shl i64 %temp_34, 2
  store i64 %temp_35, i64* @reg.a4
  %temp_36 = shl i64 16, 12
  store i64 %temp_36, i64* @reg.a5
  %temp_37 = load i64, i64* @reg.a5
  %temp_38 = add i64 %temp_37, 1336
  store i64 %temp_38, i64* @reg.a5
  %temp_39 = load i64, i64* @reg.a5
  %temp_40 = load i64, i64* @reg.a4
  %temp_41 = add i64 %temp_39, %temp_40
  store i64 %temp_41, i64* @reg.a5
  %temp_42 = load i64, i64* @reg.a5
  switch i64 %temp_42, label %Unreachable43 [ i64 66880, label %L3 i64 66872, label %L7 i64 66876, label %L2 i64 66884, label %L4 i64 66888, label %L5 i64 66892, label %L6 ]
Unreachable43:
  unreachable
L2:
  %temp_44 = load i64, i64* @reg.s0
  %temp_45 = add i64 %temp_44, -20
  %temp_46 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_45
  %temp_47 = bitcast i8* %temp_46 to i32*
  %temp_48 = load i32, i32* %temp_47
  %temp_49 = sext i32 %temp_48 to i64
  store i64 %temp_49, i64* @reg.a5
  %temp_50 = load i64, i64* @reg.a5
  %temp_51 = trunc i64 %temp_50 to i32
  %temp_52 = add i32 %temp_51, 1
  %temp_53 = sext i32 %temp_52 to i64
  store i64 %temp_53, i64* @reg.a5
  %temp_54 = load i64, i64* @reg.s0
  %temp_55 = add i64 %temp_54, -20
  %temp_56 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_55
  %temp_57 = bitcast i8* %temp_56 to i32*
  %temp_58 = load i64, i64* @reg.a5
  %temp_59 = trunc i64 %temp_58 to i32
  store i32 %temp_59, i32* %temp_57
  br label %L3
L3:
  %temp_60 = load i64, i64* @reg.s0
  %temp_61 = add i64 %temp_60, -20
  %temp_62 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_61
  %temp_63 = bitcast i8* %temp_62 to i32*
  %temp_64 = load i32, i32* %temp_63
  %temp_65 = sext i32 %temp_64 to i64
  store i64 %temp_65, i64* @reg.a5
  %temp_66 = load i64, i64* @reg.a5
  %temp_67 = trunc i64 %temp_66 to i32
  %temp_68 = add i32 %temp_67, 2
  %temp_69 = sext i32 %temp_68 to i64
  store i64 %temp_69, i64* @reg.a5
  %temp_70 = load i64, i64* @reg.s0
  %temp_71 = add i64 %temp_70, -20
  %temp_72 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_71
  %temp_73 = bitcast i8* %temp_72 to i32*
  %temp_74 = load i64, i64* @reg.a5
  %temp_75 = trunc i64 %temp_74 to i32
  store i32 %temp_75, i32* %temp_73
  br label %L7
L4:
  %temp_76 = load i64, i64* @reg.s0
  %temp_77 = add i64 %temp_76, -20
  %temp_78 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_77
  %temp_79 = bitcast i8* %temp_78 to i32*
  %temp_80 = load i32, i32* %temp_79
  %temp_81 = sext i32 %temp_80 to i64
  store i64 %temp_81, i64* @reg.a5
  %temp_82 = load i64, i64* @reg.a5
  %temp_83 = trunc i64 %temp_82 to i32
  %temp_84 = add i32 %temp_83, 3
  %temp_85 = sext i32 %temp_84 to i64
  store i64 %temp_85, i64* @reg.a5
  %temp_86 = load i64, i64* @reg.s0
  %temp_87 = add i64 %temp_86, -20
  %temp_88 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_87
  %temp_89 = bitcast i8* %temp_88 to i32*
  %temp_90 = load i64, i64* @reg.a5
  %temp_91 = trunc i64 %temp_90 to i32
  store i32 %temp_91, i32* %temp_89
  br label %L7
L5:
  %temp_92 = load i64, i64* @reg.s0
  %temp_93 = add i64 %temp_92, -20
  %temp_94 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_93
  %temp_95 = bitcast i8* %temp_94 to i32*
  %temp_96 = load i32, i32* %temp_95
  %temp_97 = sext i32 %temp_96 to i64
  store i64 %temp_97, i64* @reg.a5
  %temp_98 = load i64, i64* @reg.a5
  %temp_99 = trunc i64 %temp_98 to i32
  %temp_100 = add i32 %temp_99, 4
  %temp_101 = sext i32 %temp_100 to i64
  store i64 %temp_101, i64* @reg.a5
  %temp_102 = load i64, i64* @reg.s0
  %temp_103 = add i64 %temp_102, -20
  %temp_104 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_103
  %temp_105 = bitcast i8* %temp_104 to i32*
  %temp_106 = load i64, i64* @reg.a5
  %temp_107 = trunc i64 %temp_106 to i32
  store i32 %temp_107, i32* %temp_105
  br label %L7
L6:
  %temp_108 = load i64, i64* @reg.s0
  %temp_109 = add i64 %temp_108, -20
  %temp_110 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_109
  %temp_111 = bitcast i8* %temp_110 to i32*
  %temp_112 = load i32, i32* %temp_111
  %temp_113 = sext i32 %temp_112 to i64
  store i64 %temp_113, i64* @reg.a5
  %temp_114 = load i64, i64* @reg.a5
  %temp_115 = trunc i64 %temp_114 to i32
  %temp_116 = add i32 %temp_115, 5
  %temp_117 = sext i32 %temp_116 to i64
  store i64 %temp_117, i64* @reg.a5
  %temp_118 = load i64, i64* @reg.s0
  %temp_119 = add i64 %temp_118, -20
  %temp_120 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_119
  %temp_121 = bitcast i8* %temp_120 to i32*
  %temp_122 = load i64, i64* @reg.a5
  %temp_123 = trunc i64 %temp_122 to i32
  store i32 %temp_123, i32* %temp_121
  br label %L7
L7:
  %temp_124 = load i64, i64* @reg.s0
  %temp_125 = add i64 %temp_124, -20
  %temp_126 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_125
  %temp_127 = bitcast i8* %temp_126 to i32*
  %temp_128 = load i32, i32* %temp_127
  %temp_129 = sext i32 %temp_128 to i64
  store i64 %temp_129, i64* @reg.a5
  %temp_130 = load i64, i64* @reg.a5
  store i64 %temp_130, i64* @reg.a0
  %temp_131 = load i64, i64* @reg.sp
  %temp_132 = add i64 %temp_131, 24
  %temp_133 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_132
  %temp_134 = bitcast i8* %temp_133 to i64*
  %temp_135 = load i64, i64* %temp_134
  store i64 %temp_135, i64* @reg.s0
  %temp_136 = load i64, i64* @reg.sp
  %temp_137 = add i64 %temp_136, 32
  store i64 %temp_137, i64* @reg.sp
  ret void
}

define i64 @main() {
Entry:
  br label %L0
L0:
  %temp_0 = load i64, i64* @reg.sp
  %temp_1 = add i64 %temp_0, -16
  store i64 %temp_1, i64* @reg.sp
  %temp_2 = load i64, i64* @reg.sp
  %temp_3 = add i64 %temp_2, 8
  %temp_4 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_3
  %temp_5 = bitcast i8* %temp_4 to i64*
  %temp_6 = load i64, i64* @reg.ra
  store i64 %temp_6, i64* %temp_5
  %temp_7 = load i64, i64* @reg.sp
  %temp_8 = add i64 %temp_7, 0
  %temp_9 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_8
  %temp_10 = bitcast i8* %temp_9 to i64*
  %temp_11 = load i64, i64* @reg.s0
  store i64 %temp_11, i64* %temp_10
  %temp_12 = load i64, i64* @reg.sp
  %temp_13 = add i64 %temp_12, 16
  store i64 %temp_13, i64* @reg.s0
  store i64 1, i64* @reg.a0
  call void @f()
  %temp_14 = load i64, i64* @reg.a0
  store i64 %temp_14, i64* @reg.a5
  %temp_15 = load i64, i64* @reg.a5
  store i64 %temp_15, i64* @reg.a0
  %temp_16 = load i64, i64* @reg.sp
  %temp_17 = add i64 %temp_16, 8
  %temp_18 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_17
  %temp_19 = bitcast i8* %temp_18 to i64*
  %temp_20 = load i64, i64* %temp_19
  store i64 %temp_20, i64* @reg.ra
  %temp_21 = load i64, i64* @reg.sp
  %temp_22 = add i64 %temp_21, 0
  %temp_23 = getelementptr [10240 x i8], [10240 x i8]* @reg.stack, i8 0, i64 %temp_22
  %temp_24 = bitcast i8* %temp_23 to i64*
  %temp_25 = load i64, i64* %temp_24
  store i64 %temp_25, i64* @reg.s0
  %temp_26 = load i64, i64* @reg.sp
  %temp_27 = add i64 %temp_26, 16
  store i64 %temp_27, i64* @reg.sp
  %ret = load i64, i64* @reg.a0  ret i64 %ret}

