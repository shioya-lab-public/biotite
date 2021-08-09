@zero = global i64 0
@ra = global i64 0
@sp = global i64 0
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

define void @s() {
Entry:
    br label %L0

L0:
    %temp0 = load i64, i64* @sp
    %temp1 = add i64 %temp0, -32
    store i64 %temp1, i64* @sp

    %temp2 = load i64, i64* @sp
    %temp3 = add i64 %temp2, 24
    %temp4 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 3
    %temp5 = bitcast i8* %temp4 to i64*
    %temp6 = load i64, i64* @s0
    store i64 6, i64* %temp5

    %temp7 = load i64, i64* @sp
    %temp8 = add i64 %temp7, 32
    store i64 %temp8, i64* @s0

    %temp9 = load i64, i64* @zero
    %temp10 = load i64, i64* @a0
    %temp11 = add i64 %temp9, %temp10
    store i64 %temp11, i64* @a5

    %temp12 = load i64, i64* @s0
    %temp13 = add i64 %temp12, -20
    %temp14 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 13
    %temp15 = bitcast i8* %temp14 to i32*
    %temp16 = load i64, i64* @a5
    store i32 16, i32* %temp15

    %temp17 = load i64, i64* @s0
    %temp18 = add i64 %temp17, -20
    %temp19 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 18
    %temp20 = bitcast i8* %temp19 to i32*
    %temp21 = load i32, i32* %temp20
    store i64 %temp21, i64* @a5

    %temp22 = load i64, i64* @zero
    %temp23 = add i64 %temp22, 5
    store i64 %temp23, i64* @a5

    %temp24 = load i64, i64* @a5
    %temp25 = load i64, i64* @a4
    %temp26 = icmp ult i64 %temp24, %temp25
    br i1 %temp26, label %L7, label %L1

L1:
    %temp27 = load i64, i64* @s0
    %temp28 = add i64 %temp27, -20
    %temp29 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 28
    %temp30 = bitcast i8* %temp29 to u32*
    %temp31 = load u32, u32* %temp30
    store i64 %temp31, i64* @a5

    %temp32 = load i64, i64* @a5
    %temp33 = shl i64 %temp32, 2
    store i64 %temp33, i64* @a4

    %temp34 = shli i64 %16, 12
    store i64 %temp34, i64* @a5

    %temp35 = load i64, i64* @a5
    %temp36 = add i64 %temp35, 1428
    store i64 %temp36, i64* @a5

    %temp37 = load i64, i64* @a5
    %temp38 = load i64, i64* @a4
    %temp39 = add i64 %temp37, %temp38
    store i64 %temp39, i64* @a5

    %temp40 = load i64, i64* @a5
    %temp41 = add i64 %temp40, 0
    %temp42 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 41
    %temp43 = bitcast i8* %temp42 to i32*
    %temp44 = load i32, i32* %temp43
    store i64 %temp44, i64* @a5

    indirectbr i64* @a5, [label %L71, label %L75, label %L79, label %L83, label %L87, label %L91]

L2:
    %temp45 = load i64, i64* @s0
    %temp46 = add i64 %temp45, -20
    %temp47 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 46
    %temp48 = bitcast i8* %temp47 to i32*
    %temp49 = load i32, i32* %temp48
    store i64 %temp49, i64* @a5

    %temp50 = load i64, i64* @a5
    %temp51 = add i64 %temp50, 1
    store i64 %temp51, i64* @a5

    %temp52 = load i64, i64* @s0
    %temp53 = add i64 %temp52, -20
    %temp54 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 53
    %temp55 = bitcast i8* %temp54 to i32*
    %temp56 = load i64, i64* @a5
    store i32 56, i32* %temp55

    br label %L7

L3:
    %temp57 = load i64, i64* @s0
    %temp58 = add i64 %temp57, -20
    %temp59 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 58
    %temp60 = bitcast i8* %temp59 to i32*
    %temp61 = load i32, i32* %temp60
    store i64 %temp61, i64* @a5

    %temp62 = load i64, i64* @a5
    %temp63 = add i64 %temp62, 1
    store i64 %temp63, i64* @a5

    %temp64 = load i64, i64* @s0
    %temp65 = add i64 %temp64, -20
    %temp66 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 65
    %temp67 = bitcast i8* %temp66 to i32*
    %temp68 = load i64, i64* @a5
    store i32 68, i32* %temp67

    br label %L7

L4:
    %temp69 = load i64, i64* @s0
    %temp70 = add i64 %temp69, -20
    %temp71 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 70
    %temp72 = bitcast i8* %temp71 to i32*
    %temp73 = load i32, i32* %temp72
    store i64 %temp73, i64* @a5

    %temp74 = load i64, i64* @a5
    %temp75 = add i64 %temp74, 1
    store i64 %temp75, i64* @a5

    %temp76 = load i64, i64* @s0
    %temp77 = add i64 %temp76, -20
    %temp78 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 77
    %temp79 = bitcast i8* %temp78 to i32*
    %temp80 = load i64, i64* @a5
    store i32 80, i32* %temp79

    br label %L7

L5:
    %temp81 = load i64, i64* @s0
    %temp82 = add i64 %temp81, -20
    %temp83 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 82
    %temp84 = bitcast i8* %temp83 to i32*
    %temp85 = load i32, i32* %temp84
    store i64 %temp85, i64* @a5

    %temp86 = load i64, i64* @a5
    %temp87 = add i64 %temp86, 1
    store i64 %temp87, i64* @a5

    %temp88 = load i64, i64* @s0
    %temp89 = add i64 %temp88, -20
    %temp90 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 89
    %temp91 = bitcast i8* %temp90 to i32*
    %temp92 = load i64, i64* @a5
    store i32 92, i32* %temp91

    br label %L7

L6:
    %temp93 = load i64, i64* @s0
    %temp94 = add i64 %temp93, -20
    %temp95 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 94
    %temp96 = bitcast i8* %temp95 to i32*
    %temp97 = load i32, i32* %temp96
    store i64 %temp97, i64* @a5

    %temp98 = load i64, i64* @a5
    %temp99 = add i64 %temp98, 1
    store i64 %temp99, i64* @a5

    %temp100 = load i64, i64* @s0
    %temp101 = add i64 %temp100, -20
    %temp102 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 101
    %temp103 = bitcast i8* %temp102 to i32*
    %temp104 = load i64, i64* @a5
    store i32 104, i32* %temp103

L7:
    %temp105 = load i64, i64* @s0
    %temp106 = add i64 %temp105, -20
    %temp107 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 106
    %temp108 = bitcast i8* %temp107 to i32*
    %temp109 = load i32, i32* %temp108
    store i64 %temp109, i64* @a5

    %temp110 = load i64, i64* @zero
    %temp111 = load i64, i64* @a5
    %temp112 = add i64 %temp110, %temp111
    store i64 %temp112, i64* @a0

    %temp113 = load i64, i64* @sp
    %temp114 = add i64 %temp113, 24
    %temp115 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 114
    %temp116 = bitcast i8* %temp115 to i64*
    %temp117 = load i64, i64* %temp116
    store i64 %temp117, i64* @s0

    %temp118 = load i64, i64* @sp
    %temp119 = add i64 %temp118, 32
    store i64 %temp119, i64* @sp

    ret
}

define void @main() {
Entry:
    br label %L0

L0:
    %temp0 = load i64, i64* @sp
    %temp1 = add i64 %temp0, -32
    store i64 %temp1, i64* @sp

    %temp2 = load i64, i64* @sp
    %temp3 = add i64 %temp2, 24
    %temp4 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 3
    %temp5 = bitcast i8* %temp4 to i64*
    %temp6 = load i64, i64* @ra
    store i64 6, i64* %temp5

    %temp7 = load i64, i64* @sp
    %temp8 = add i64 %temp7, 16
    %temp9 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 8
    %temp10 = bitcast i8* %temp9 to i64*
    %temp11 = load i64, i64* @s0
    store i64 11, i64* %temp10

    %temp12 = load i64, i64* @sp
    %temp13 = add i64 %temp12, 32
    store i64 %temp13, i64* @s0

    %temp14 = load i64, i64* @s0
    %temp15 = add i64 %temp14, -20
    %temp16 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 15
    %temp17 = bitcast i8* %temp16 to i32*
    %temp18 = load i64, i64* @zero
    store i32 18, i32* %temp17

    %temp19 = load i64, i64* @s0
    %temp20 = add i64 %temp19, -24
    %temp21 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 20
    %temp22 = bitcast i8* %temp21 to i32*
    %temp23 = load i64, i64* @zero
    store i32 23, i32* %temp22

    br label %L2

L1:
    %temp24 = load i64, i64* @s0
    %temp25 = add i64 %temp24, -20
    %temp26 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 25
    %temp27 = bitcast i8* %temp26 to i32*
    %temp28 = load i32, i32* %temp27
    store i64 %temp28, i64* @a5

    %temp29 = load i64, i64* @a5
    %temp30 = add i64 %temp29, 1
    store i64 %temp30, i64* @a5

    %temp31 = load i64, i64* @s0
    %temp32 = add i64 %temp31, -20
    %temp33 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 32
    %temp34 = bitcast i8* %temp33 to i32*
    %temp35 = load i64, i64* @a5
    store i32 35, i32* %temp34

    %temp36 = load i64, i64* @s0
    %temp37 = add i64 %temp36, -24
    %temp38 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 37
    %temp39 = bitcast i8* %temp38 to i32*
    %temp40 = load i32, i32* %temp39
    store i64 %temp40, i64* @a5

    %temp41 = load i64, i64* @a5
    %temp42 = add i64 %temp41, 1
    store i64 %temp42, i64* @a5

    %temp43 = load i64, i64* @s0
    %temp44 = add i64 %temp43, -24
    %temp45 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 44
    %temp46 = bitcast i8* %temp45 to i32*
    %temp47 = load i64, i64* @a5
    store i32 47, i32* %temp46

L2:
    %temp48 = load i64, i64* @s0
    %temp49 = add i64 %temp48, -24
    %temp50 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 49
    %temp51 = bitcast i8* %temp50 to i32*
    %temp52 = load i32, i32* %temp51
    store i64 %temp52, i64* @a5

    %temp53 = load i64, i64* @a5
    %temp54 = load i64, i64* @zero
    %temp55 = icmp sle i64 %temp53, %temp54
    br i1 %temp55, label %L1, label %L3

L3:
    br label %L5

L4:
    %temp56 = load i64, i64* @s0
    %temp57 = add i64 %temp56, -20
    %temp58 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 57
    %temp59 = bitcast i8* %temp58 to i32*
    %temp60 = load i32, i32* %temp59
    store i64 %temp60, i64* @a5

    %temp61 = load i64, i64* @a5
    %temp62 = add i64 %temp61, 1
    store i64 %temp62, i64* @a5

    %temp63 = load i64, i64* @s0
    %temp64 = add i64 %temp63, -20
    %temp65 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 64
    %temp66 = bitcast i8* %temp65 to i32*
    %temp67 = load i64, i64* @a5
    store i32 67, i32* %temp66

L5:
    %temp68 = load i64, i64* @s0
    %temp69 = add i64 %temp68, -20
    %temp70 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 69
    %temp71 = bitcast i8* %temp70 to i32*
    %temp72 = load i32, i32* %temp71
    store i64 %temp72, i64* @a5

    %temp73 = load i64, i64* @zero
    %temp74 = add i64 %temp73, 1
    store i64 %temp74, i64* @a5

    %temp75 = load i64, i64* @a5
    %temp76 = load i64, i64* @a4
    %temp77 = icmp sge i64 %temp75, %temp76
    br i1 %temp77, label %L4, label %L6

L6:
    %temp78 = load i64, i64* @s0
    %temp79 = add i64 %temp78, -20
    %temp80 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 79
    %temp81 = bitcast i8* %temp80 to i32*
    %temp82 = load i32, i32* %temp81
    store i64 %temp82, i64* @a5

    %temp83 = load i64, i64* @a5
    %temp84 = add i64 %temp83, 1
    store i64 %temp84, i64* @a5

    %temp85 = load i64, i64* @s0
    %temp86 = add i64 %temp85, -20
    %temp87 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 86
    %temp88 = bitcast i8* %temp87 to i32*
    %temp89 = load i64, i64* @a5
    store i32 89, i32* %temp88

    %temp90 = load i64, i64* @s0
    %temp91 = add i64 %temp90, -20
    %temp92 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 91
    %temp93 = bitcast i8* %temp92 to i32*
    %temp94 = load i32, i32* %temp93
    store i64 %temp94, i64* @a5

    %temp95 = load i64, i64* @zero
    %temp96 = load i64, i64* @a5
    %temp97 = add i64 %temp95, %temp96
    store i64 %temp97, i64* @a0

    call void @s()

    %temp98 = load i64, i64* @zero
    %temp99 = load i64, i64* @a0
    %temp100 = add i64 %temp98, %temp99
    store i64 %temp100, i64* @a5

    %temp101 = load i64, i64* @s0
    %temp102 = add i64 %temp101, -20
    %temp103 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 102
    %temp104 = bitcast i8* %temp103 to i32*
    %temp105 = load i64, i64* @a5
    store i32 105, i32* %temp104

    %temp106 = load i64, i64* @s0
    %temp107 = add i64 %temp106, -20
    %temp108 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 107
    %temp109 = bitcast i8* %temp108 to i32*
    %temp110 = load i32, i32* %temp109
    store i64 %temp110, i64* @a5

    %temp111 = load i64, i64* @zero
    %temp112 = load i64, i64* @a5
    %temp113 = add i64 %temp111, %temp112
    store i64 %temp113, i64* @a0

    %temp114 = load i64, i64* @sp
    %temp115 = add i64 %temp114, 24
    %temp116 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 115
    %temp117 = bitcast i8* %temp116 to i64*
    %temp118 = load i64, i64* %temp117
    store i64 %temp118, i64* @ra

    %temp119 = load i64, i64* @sp
    %temp120 = add i64 %temp119, 16
    %temp121 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 120
    %temp122 = bitcast i8* %temp121 to i64*
    %temp123 = load i64, i64* %temp122
    store i64 %temp123, i64* @s0

    %temp124 = load i64, i64* @sp
    %temp125 = add i64 %temp124, 32
    store i64 %temp125, i64* @sp

    ret
}
