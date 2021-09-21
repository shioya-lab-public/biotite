
; @.str = private unnamed_addr constant [8 x i8] c"### %d\0A\00", align 1
; declare dso_local i32 @printf(i8*, ...)
; %val = load i64, i64* @zero
; call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.str, i64 0, i64 0), i64 %val)

@zero = global i64 0
@ra = global i64 0
@sp = global i64 1023
@gp = global i64 0
@tp = global i64 0
@t0 = global i64 0
@t1 = global i64 0
@t2 = global i64 0
@s0 = global i64 0
@s1 = global i64 0
@a0 = global i64 0
@a1 = global i64 0
@a2 = global i64 0
@a3 = global i64 0
@a4 = global i64 0
@a5 = global i64 0
@a6 = global i64 0
@a7 = global i64 0
@s2 = global i64 0
@s3 = global i64 0
@s4 = global i64 0
@s5 = global i64 0
@s6 = global i64 0
@s7 = global i64 0
@s8 = global i64 0
@s9 = global i64 0
@s10 = global i64 0
@s11 = global i64 0
@t3 = global i64 0
@t4 = global i64 0
@t5 = global i64 0
@t6 = global i64 0

@stack = global [1024 x i8] zeroinitializer

define i64 @s() {
    ; Label("Entry")
Entry:
    ; DirectBr("L0")
    br label %L0

    ; Label("L0")
L0:
    ; Addi { result: Sp, op1: Sp, op2: -32 }
    %temp_0 = load i64, i64* @sp
    %temp_1 = add i64 %temp_0, -32
    store i64 %temp_1, i64* @sp

    ; Save { ty: I64, op1: Sp, op2: 24, source: S0 }
    %temp_2 = load i64, i64* @sp
    %temp_3 = add i64 %temp_2, 24
    %temp_4 = sub i64 1023, %temp_3
    %temp_5 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_4
    %temp_6 = bitcast i8* %temp_5 to i64*
    %temp_7 = load i64, i64* @s0
    store i64 %temp_7, i64* %temp_6

    ; Addi { result: S0, op1: Sp, op2: 32 }
    %temp_8 = load i64, i64* @sp
    %temp_9 = add i64 %temp_8, 32
    store i64 %temp_9, i64* @s0

    ; Add { result: A5, op1: Zero, op2: A0 }
    %temp_10 = load i64, i64* @zero
    %temp_11 = load i64, i64* @a0
    %temp_12 = add i64 %temp_10, %temp_11
    store i64 %temp_12, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_13 = load i64, i64* @s0
    %temp_14 = add i64 %temp_13, -20
    %temp_15 = sub i64 1023, %temp_14
    %temp_16 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_15
    %temp_17 = bitcast i8* %temp_16 to i32*
    %temp_18 = load i64, i64* @a5
    %temp_19 = trunc i64 %temp_18 to i32
    store i32 %temp_19, i32* %temp_17

    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_20 = load i64, i64* @s0
    %temp_21 = add i64 %temp_20, -20
    %temp_22 = sub i64 1023, %temp_21
    %temp_23 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_22
    %temp_24 = bitcast i8* %temp_23 to i32*
    %temp_25 = load i32, i32* %temp_24
    %temp_26 = sext i32 %temp_25 to i64
    store i64 %temp_26, i64* @a5

    ; Addi { result: A4, op1: A5, op2: 0 }
    %temp_27 = load i64, i64* @a5
    %temp_28 = add i64 %temp_27, 0
    store i64 %temp_28, i64* @a4

    ; Addi { result: A5, op1: Zero, op2: 5 }
    %temp_29 = load i64, i64* @zero
    %temp_30 = add i64 %temp_29, 5
    store i64 %temp_30, i64* @a5

    ; Icmp { condition: Ult, op1: A5, op2: A4 }
    %temp_31 = load i64, i64* @a5
    %temp_32 = load i64, i64* @a4
    %temp_33 = icmp ult i64 %temp_31, %temp_32

    ; Br { iftrue: "L7", iffalse: "L1" }
    br i1 %temp_33, label %L7, label %L1

    ; Label("L1")
L1:
    ; Load { ty: U32, result: A5, op1: S0, op2: -20 }
    %temp_34 = load i64, i64* @s0
    %temp_35 = add i64 %temp_34, -20
    %temp_36 = sub i64 1023, %temp_35
    %temp_37 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_36
    %temp_38 = bitcast i8* %temp_37 to i32*
    %temp_39 = load i32, i32* %temp_38
    %temp_40 = sext i32 %temp_39 to i64
    store i64 %temp_40, i64* @a5

    ; Shli { result: A4, op1: A5, op2: 2 }
    %temp_41 = load i64, i64* @a5
    %temp_42 = shl i64 %temp_41, 2
    store i64 %temp_42, i64* @a4

    ; Shli12 { result: A5, op1: 16 }
    %temp_43 = shl i64 16, 12
    store i64 %temp_43, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 1468 }
    %temp_44 = load i64, i64* @a5
    %temp_45 = add i64 %temp_44, 1468
    store i64 %temp_45, i64* @a5

    ; Add { result: A5, op1: A5, op2: A4 }
    %temp_46 = load i64, i64* @a5
    %temp_47 = load i64, i64* @a4
    %temp_48 = add i64 %temp_46, %temp_47
    store i64 %temp_48, i64* @a5

    ; Load { ty: I32, result: A5, op1: A5, op2: 0 }
    ; Switch { register: A5, targets: {67016: 4, 67024: 6, 67004: 7, 67020: 5, 67012: 3, 67008: 2} }
    %temp_56 = load i64, i64* @a5
    switch i64 %temp_56, label %L57 [ i64 67016, label %L4 i64 67024, label %L6 i64 67004, label %L7 i64 67020, label %L5 i64 67012, label %L3 i64 67008, label %L2 ]
L57:
    unreachable

    ; Label("L2")
L2:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_58 = load i64, i64* @s0
    %temp_59 = add i64 %temp_58, -20
    %temp_60 = sub i64 1023, %temp_59
    %temp_61 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_60
    %temp_62 = bitcast i8* %temp_61 to i32*
    %temp_63 = load i32, i32* %temp_62
    %temp_64 = sext i32 %temp_63 to i64
    store i64 %temp_64, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 1 }
    %temp_65 = load i64, i64* @a5
    %temp_66 = add i64 %temp_65, 1
    store i64 %temp_66, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_67 = load i64, i64* @s0
    %temp_68 = add i64 %temp_67, -20
    %temp_69 = sub i64 1023, %temp_68
    %temp_70 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_69
    %temp_71 = bitcast i8* %temp_70 to i32*
    %temp_72 = load i64, i64* @a5
    %temp_73 = trunc i64 %temp_72 to i32
    store i32 %temp_73, i32* %temp_71

    ; DirectBr("L7")
    br label %L7

    ; Label("L3")
L3:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_74 = load i64, i64* @s0
    %temp_75 = add i64 %temp_74, -20
    %temp_76 = sub i64 1023, %temp_75
    %temp_77 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_76
    %temp_78 = bitcast i8* %temp_77 to i32*
    %temp_79 = load i32, i32* %temp_78
    %temp_80 = sext i32 %temp_79 to i64
    store i64 %temp_80, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 2 }
    %temp_81 = load i64, i64* @a5
    %temp_82 = add i64 %temp_81, 2
    store i64 %temp_82, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_83 = load i64, i64* @s0
    %temp_84 = add i64 %temp_83, -20
    %temp_85 = sub i64 1023, %temp_84
    %temp_86 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_85
    %temp_87 = bitcast i8* %temp_86 to i32*
    %temp_88 = load i64, i64* @a5
    %temp_89 = trunc i64 %temp_88 to i32
    store i32 %temp_89, i32* %temp_87

    ; DirectBr("L7")
    br label %L7

    ; Label("L4")
L4:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_90 = load i64, i64* @s0
    %temp_91 = add i64 %temp_90, -20
    %temp_92 = sub i64 1023, %temp_91
    %temp_93 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_92
    %temp_94 = bitcast i8* %temp_93 to i32*
    %temp_95 = load i32, i32* %temp_94
    %temp_96 = sext i32 %temp_95 to i64
    store i64 %temp_96, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 3 }
    %temp_97 = load i64, i64* @a5
    %temp_98 = add i64 %temp_97, 3
    store i64 %temp_98, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_99 = load i64, i64* @s0
    %temp_100 = add i64 %temp_99, -20
    %temp_101 = sub i64 1023, %temp_100
    %temp_102 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_101
    %temp_103 = bitcast i8* %temp_102 to i32*
    %temp_104 = load i64, i64* @a5
    %temp_105 = trunc i64 %temp_104 to i32
    store i32 %temp_105, i32* %temp_103

    ; DirectBr("L7")
    br label %L7

    ; Label("L5")
L5:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_106 = load i64, i64* @s0
    %temp_107 = add i64 %temp_106, -20
    %temp_108 = sub i64 1023, %temp_107
    %temp_109 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_108
    %temp_110 = bitcast i8* %temp_109 to i32*
    %temp_111 = load i32, i32* %temp_110
    %temp_112 = sext i32 %temp_111 to i64
    store i64 %temp_112, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 4 }
    %temp_113 = load i64, i64* @a5
    %temp_114 = add i64 %temp_113, 4
    store i64 %temp_114, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_115 = load i64, i64* @s0
    %temp_116 = add i64 %temp_115, -20
    %temp_117 = sub i64 1023, %temp_116
    %temp_118 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_117
    %temp_119 = bitcast i8* %temp_118 to i32*
    %temp_120 = load i64, i64* @a5
    %temp_121 = trunc i64 %temp_120 to i32
    store i32 %temp_121, i32* %temp_119

    ; DirectBr("L7")
    br label %L7

    ; Label("L6")
L6:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_122 = load i64, i64* @s0
    %temp_123 = add i64 %temp_122, -20
    %temp_124 = sub i64 1023, %temp_123
    %temp_125 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_124
    %temp_126 = bitcast i8* %temp_125 to i32*
    %temp_127 = load i32, i32* %temp_126
    %temp_128 = sext i32 %temp_127 to i64
    store i64 %temp_128, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 5 }
    %temp_129 = load i64, i64* @a5
    %temp_130 = add i64 %temp_129, 5
    store i64 %temp_130, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_131 = load i64, i64* @s0
    %temp_132 = add i64 %temp_131, -20
    %temp_133 = sub i64 1023, %temp_132
    %temp_134 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_133
    %temp_135 = bitcast i8* %temp_134 to i32*
    %temp_136 = load i64, i64* @a5
    %temp_137 = trunc i64 %temp_136 to i32
    store i32 %temp_137, i32* %temp_135

    ; DirectBr("L7")
    br label %L7

    ; Label("L7")
L7:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_138 = load i64, i64* @s0
    %temp_139 = add i64 %temp_138, -20
    %temp_140 = sub i64 1023, %temp_139
    %temp_141 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_140
    %temp_142 = bitcast i8* %temp_141 to i32*
    %temp_143 = load i32, i32* %temp_142
    %temp_144 = sext i32 %temp_143 to i64
    store i64 %temp_144, i64* @a5

    ; Add { result: A0, op1: Zero, op2: A5 }
    %temp_145 = load i64, i64* @zero
    %temp_146 = load i64, i64* @a5
    %temp_147 = add i64 %temp_145, %temp_146
    store i64 %temp_147, i64* @a0

    ; Load { ty: I64, result: S0, op1: Sp, op2: 24 }
    %temp_148 = load i64, i64* @sp
    %temp_149 = add i64 %temp_148, 24
    %temp_150 = sub i64 1023, %temp_149
    %temp_151 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_150
    %temp_152 = bitcast i8* %temp_151 to i64*
    %temp_153 = load i64, i64* %temp_152
    store i64 %temp_153, i64* @s0

    ; Addi { result: Sp, op1: Sp, op2: 32 }
    %temp_154 = load i64, i64* @sp
    %temp_155 = add i64 %temp_154, 32
    store i64 %temp_155, i64* @sp

    ; Ret
    %temp_156 = load i64, i64* @a0
    ret i64 %temp_156
}

define i64 @main() {
    ; Label("Entry")
Entry:
    ; DirectBr("L0")
    br label %L0

    ; Label("L0")
L0:
    ; Addi { result: Sp, op1: Sp, op2: -32 }
    %temp_0 = load i64, i64* @sp
    %temp_1 = add i64 %temp_0, -32
    store i64 %temp_1, i64* @sp

    ; Save { ty: I64, op1: Sp, op2: 24, source: Ra }
    %temp_2 = load i64, i64* @sp
    %temp_3 = add i64 %temp_2, 24
    %temp_4 = sub i64 1023, %temp_3
    %temp_5 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_4
    %temp_6 = bitcast i8* %temp_5 to i64*
    %temp_7 = load i64, i64* @ra
    store i64 %temp_7, i64* %temp_6

    ; Save { ty: I64, op1: Sp, op2: 16, source: S0 }
    %temp_8 = load i64, i64* @sp
    %temp_9 = add i64 %temp_8, 16
    %temp_10 = sub i64 1023, %temp_9
    %temp_11 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_10
    %temp_12 = bitcast i8* %temp_11 to i64*
    %temp_13 = load i64, i64* @s0
    store i64 %temp_13, i64* %temp_12

    ; Addi { result: S0, op1: Sp, op2: 32 }
    %temp_14 = load i64, i64* @sp
    %temp_15 = add i64 %temp_14, 32
    store i64 %temp_15, i64* @s0

    ; Save { ty: I32, op1: S0, op2: -20, source: Zero }
    %temp_16 = load i64, i64* @s0
    %temp_17 = add i64 %temp_16, -20
    %temp_18 = sub i64 1023, %temp_17
    %temp_19 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_18
    %temp_20 = bitcast i8* %temp_19 to i32*
    %temp_21 = load i64, i64* @zero
    %temp_22 = trunc i64 %temp_21 to i32
    store i32 %temp_22, i32* %temp_20

    ; Save { ty: I32, op1: S0, op2: -24, source: Zero }
    %temp_23 = load i64, i64* @s0
    %temp_24 = add i64 %temp_23, -24
    %temp_25 = sub i64 1023, %temp_24
    %temp_26 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_25
    %temp_27 = bitcast i8* %temp_26 to i32*
    %temp_28 = load i64, i64* @zero
    %temp_29 = trunc i64 %temp_28 to i32
    store i32 %temp_29, i32* %temp_27

    ; DirectBr("L8")
    br label %L8

    ; Label("L1")
L1:
    ; Save { ty: I32, op1: S0, op2: -20, source: Zero }
    %temp_30 = load i64, i64* @s0
    %temp_31 = add i64 %temp_30, -20
    %temp_32 = sub i64 1023, %temp_31
    %temp_33 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_32
    %temp_34 = bitcast i8* %temp_33 to i32*
    %temp_35 = load i64, i64* @zero
    %temp_36 = trunc i64 %temp_35 to i32
    store i32 %temp_36, i32* %temp_34

    ; Save { ty: I32, op1: S0, op2: -28, source: Zero }
    %temp_37 = load i64, i64* @s0
    %temp_38 = add i64 %temp_37, -28
    %temp_39 = sub i64 1023, %temp_38
    %temp_40 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_39
    %temp_41 = bitcast i8* %temp_40 to i32*
    %temp_42 = load i64, i64* @zero
    %temp_43 = trunc i64 %temp_42 to i32
    store i32 %temp_43, i32* %temp_41

    ; DirectBr("L3")
    br label %L3

    ; Label("L2")
L2:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_44 = load i64, i64* @s0
    %temp_45 = add i64 %temp_44, -20
    %temp_46 = sub i64 1023, %temp_45
    %temp_47 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_46
    %temp_48 = bitcast i8* %temp_47 to i32*
    %temp_49 = load i32, i32* %temp_48
    %temp_50 = sext i32 %temp_49 to i64
    store i64 %temp_50, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 1 }
    %temp_51 = load i64, i64* @a5
    %temp_52 = add i64 %temp_51, 1
    store i64 %temp_52, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_53 = load i64, i64* @s0
    %temp_54 = add i64 %temp_53, -20
    %temp_55 = sub i64 1023, %temp_54
    %temp_56 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_55
    %temp_57 = bitcast i8* %temp_56 to i32*
    %temp_58 = load i64, i64* @a5
    %temp_59 = trunc i64 %temp_58 to i32
    store i32 %temp_59, i32* %temp_57

    ; Load { ty: I32, result: A5, op1: S0, op2: -28 }
    %temp_60 = load i64, i64* @s0
    %temp_61 = add i64 %temp_60, -28
    %temp_62 = sub i64 1023, %temp_61
    %temp_63 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_62
    %temp_64 = bitcast i8* %temp_63 to i32*
    %temp_65 = load i32, i32* %temp_64
    %temp_66 = sext i32 %temp_65 to i64
    store i64 %temp_66, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 1 }
    %temp_67 = load i64, i64* @a5
    %temp_68 = add i64 %temp_67, 1
    store i64 %temp_68, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -28, source: A5 }
    %temp_69 = load i64, i64* @s0
    %temp_70 = add i64 %temp_69, -28
    %temp_71 = sub i64 1023, %temp_70
    %temp_72 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_71
    %temp_73 = bitcast i8* %temp_72 to i32*
    %temp_74 = load i64, i64* @a5
    %temp_75 = trunc i64 %temp_74 to i32
    store i32 %temp_75, i32* %temp_73

    ; DirectBr("L3")
    br label %L3

    ; Label("L3")
L3:
    ; Load { ty: I32, result: A5, op1: S0, op2: -28 }
    %temp_76 = load i64, i64* @s0
    %temp_77 = add i64 %temp_76, -28
    %temp_78 = sub i64 1023, %temp_77
    %temp_79 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_78
    %temp_80 = bitcast i8* %temp_79 to i32*
    %temp_81 = load i32, i32* %temp_80
    %temp_82 = sext i32 %temp_81 to i64
    store i64 %temp_82, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 0 }
    %temp_83 = load i64, i64* @a5
    %temp_84 = add i64 %temp_83, 0
    store i64 %temp_84, i64* @a5

    ; Icmp { condition: Sle, op1: A5, op2: Zero }
    %temp_85 = load i64, i64* @a5
    %temp_86 = load i64, i64* @zero
    %temp_87 = icmp sle i64 %temp_85, %temp_86

    ; Br { iftrue: "L2", iffalse: "L4" }
    br i1 %temp_87, label %L2, label %L4

    ; Label("L4")
L4:
    ; DirectBr("L6")
    br label %L6

    ; Label("L5")
L5:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_88 = load i64, i64* @s0
    %temp_89 = add i64 %temp_88, -20
    %temp_90 = sub i64 1023, %temp_89
    %temp_91 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_90
    %temp_92 = bitcast i8* %temp_91 to i32*
    %temp_93 = load i32, i32* %temp_92
    %temp_94 = sext i32 %temp_93 to i64
    store i64 %temp_94, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 1 }
    %temp_95 = load i64, i64* @a5
    %temp_96 = add i64 %temp_95, 1
    store i64 %temp_96, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_97 = load i64, i64* @s0
    %temp_98 = add i64 %temp_97, -20
    %temp_99 = sub i64 1023, %temp_98
    %temp_100 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_99
    %temp_101 = bitcast i8* %temp_100 to i32*
    %temp_102 = load i64, i64* @a5
    %temp_103 = trunc i64 %temp_102 to i32
    store i32 %temp_103, i32* %temp_101

    ; DirectBr("L6")
    br label %L6

    ; Label("L6")
L6:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_104 = load i64, i64* @s0
    %temp_105 = add i64 %temp_104, -20
    %temp_106 = sub i64 1023, %temp_105
    %temp_107 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_106
    %temp_108 = bitcast i8* %temp_107 to i32*
    %temp_109 = load i32, i32* %temp_108
    %temp_110 = sext i32 %temp_109 to i64
    store i64 %temp_110, i64* @a5

    ; Addi { result: A4, op1: A5, op2: 0 }
    %temp_111 = load i64, i64* @a5
    %temp_112 = add i64 %temp_111, 0
    store i64 %temp_112, i64* @a4

    ; Addi { result: A5, op1: Zero, op2: 1 }
    %temp_113 = load i64, i64* @zero
    %temp_114 = add i64 %temp_113, 1
    store i64 %temp_114, i64* @a5

    ; Icmp { condition: Sge, op1: A5, op2: A4 }
    %temp_115 = load i64, i64* @a5
    %temp_116 = load i64, i64* @a4
    %temp_117 = icmp sge i64 %temp_115, %temp_116

    ; Br { iftrue: "L5", iffalse: "L7" }
    br i1 %temp_117, label %L5, label %L7

    ; Label("L7")
L7:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_118 = load i64, i64* @s0
    %temp_119 = add i64 %temp_118, -20
    %temp_120 = sub i64 1023, %temp_119
    %temp_121 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_120
    %temp_122 = bitcast i8* %temp_121 to i32*
    %temp_123 = load i32, i32* %temp_122
    %temp_124 = sext i32 %temp_123 to i64
    store i64 %temp_124, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 1 }
    %temp_125 = load i64, i64* @a5
    %temp_126 = add i64 %temp_125, 1
    store i64 %temp_126, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_127 = load i64, i64* @s0
    %temp_128 = add i64 %temp_127, -20
    %temp_129 = sub i64 1023, %temp_128
    %temp_130 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_129
    %temp_131 = bitcast i8* %temp_130 to i32*
    %temp_132 = load i64, i64* @a5
    %temp_133 = trunc i64 %temp_132 to i32
    store i32 %temp_133, i32* %temp_131

    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_134 = load i64, i64* @s0
    %temp_135 = add i64 %temp_134, -20
    %temp_136 = sub i64 1023, %temp_135
    %temp_137 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_136
    %temp_138 = bitcast i8* %temp_137 to i32*
    %temp_139 = load i32, i32* %temp_138
    %temp_140 = sext i32 %temp_139 to i64
    store i64 %temp_140, i64* @a5

    ; Add { result: A0, op1: Zero, op2: A5 }
    %temp_141 = load i64, i64* @zero
    %temp_142 = load i64, i64* @a5
    %temp_143 = add i64 %temp_141, %temp_142
    store i64 %temp_143, i64* @a0

    ; Call("s")
    call i64 @s()

    ; Add { result: A5, op1: Zero, op2: A0 }
    %temp_144 = load i64, i64* @zero
    %temp_145 = load i64, i64* @a0
    %temp_146 = add i64 %temp_144, %temp_145
    store i64 %temp_146, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -20, source: A5 }
    %temp_147 = load i64, i64* @s0
    %temp_148 = add i64 %temp_147, -20
    %temp_149 = sub i64 1023, %temp_148
    %temp_150 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_149
    %temp_151 = bitcast i8* %temp_150 to i32*
    %temp_152 = load i64, i64* @a5
    %temp_153 = trunc i64 %temp_152 to i32
    store i32 %temp_153, i32* %temp_151

    ; Load { ty: I32, result: A5, op1: S0, op2: -24 }
    %temp_154 = load i64, i64* @s0
    %temp_155 = add i64 %temp_154, -24
    %temp_156 = sub i64 1023, %temp_155
    %temp_157 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_156
    %temp_158 = bitcast i8* %temp_157 to i32*
    %temp_159 = load i32, i32* %temp_158
    %temp_160 = sext i32 %temp_159 to i64
    store i64 %temp_160, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 1 }
    %temp_161 = load i64, i64* @a5
    %temp_162 = add i64 %temp_161, 1
    store i64 %temp_162, i64* @a5

    ; Save { ty: I32, op1: S0, op2: -24, source: A5 }
    %temp_163 = load i64, i64* @s0
    %temp_164 = add i64 %temp_163, -24
    %temp_165 = sub i64 1023, %temp_164
    %temp_166 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_165
    %temp_167 = bitcast i8* %temp_166 to i32*
    %temp_168 = load i64, i64* @a5
    %temp_169 = trunc i64 %temp_168 to i32
    store i32 %temp_169, i32* %temp_167

    ; DirectBr("L8")
    br label %L8

    ; Label("L8")
L8:
    ; Load { ty: I32, result: A5, op1: S0, op2: -24 }
    %temp_170 = load i64, i64* @s0
    %temp_171 = add i64 %temp_170, -24
    %temp_172 = sub i64 1023, %temp_171
    %temp_173 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_172
    %temp_174 = bitcast i8* %temp_173 to i32*
    %temp_175 = load i32, i32* %temp_174
    %temp_176 = sext i32 %temp_175 to i64
    store i64 %temp_176, i64* @a5

    ; Addi { result: A4, op1: A5, op2: 0 }
    %temp_177 = load i64, i64* @a5
    %temp_178 = add i64 %temp_177, 0
    store i64 %temp_178, i64* @a4

    ; Shli12 { result: A5, op1: 24414 }
    %temp_179 = shl i64 24414, 12
    store i64 %temp_179, i64* @a5

    ; Addi { result: A5, op1: A5, op2: 255 }
    %temp_180 = load i64, i64* @a5
    %temp_181 = add i64 %temp_180, 255
    store i64 %temp_181, i64* @a5

    ; Icmp { condition: Sge, op1: A5, op2: A4 }
    %temp_182 = load i64, i64* @a5
    %temp_183 = load i64, i64* @a4
    %temp_184 = icmp sge i64 %temp_182, %temp_183

    ; Br { iftrue: "L1", iffalse: "L9" }
    br i1 %temp_184, label %L1, label %L9

    ; Label("L9")
L9:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    %temp_185 = load i64, i64* @s0
    %temp_186 = add i64 %temp_185, -20
    %temp_187 = sub i64 1023, %temp_186
    %temp_188 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_187
    %temp_189 = bitcast i8* %temp_188 to i32*
    %temp_190 = load i32, i32* %temp_189
    %temp_191 = sext i32 %temp_190 to i64
    store i64 %temp_191, i64* @a5

    ; Add { result: A0, op1: Zero, op2: A5 }
    %temp_192 = load i64, i64* @zero
    %temp_193 = load i64, i64* @a5
    %temp_194 = add i64 %temp_192, %temp_193
    store i64 %temp_194, i64* @a0

    ; Load { ty: I64, result: Ra, op1: Sp, op2: 24 }
    %temp_195 = load i64, i64* @sp
    %temp_196 = add i64 %temp_195, 24
    %temp_197 = sub i64 1023, %temp_196
    %temp_198 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_197
    %temp_199 = bitcast i8* %temp_198 to i64*
    %temp_200 = load i64, i64* %temp_199
    store i64 %temp_200, i64* @ra

    ; Load { ty: I64, result: S0, op1: Sp, op2: 16 }
    %temp_201 = load i64, i64* @sp
    %temp_202 = add i64 %temp_201, 16
    %temp_203 = sub i64 1023, %temp_202
    %temp_204 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_203
    %temp_205 = bitcast i8* %temp_204 to i64*
    %temp_206 = load i64, i64* %temp_205
    store i64 %temp_206, i64* @s0

    ; Addi { result: Sp, op1: Sp, op2: 32 }
    %temp_207 = load i64, i64* @sp
    %temp_208 = add i64 %temp_207, 32
    store i64 %temp_208, i64* @sp

    ; Ret
    %temp_209 = load i64, i64* @a0
    ret i64 %temp_209
}
