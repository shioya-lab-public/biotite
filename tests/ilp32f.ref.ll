; ABI: ilp32f

declare i32 @syscall(i32, ...)

declare float @llvm.sqrt.f32(float %op1)
declare float @llvm.fma.f32(float %op1, float %op2, float %op3)
declare float @llvm.fabs.f32(float %op1)
declare float @llvm.minimum.f32(float %op1, float %op2)
declare float @llvm.maximum.f32(float %op1, float %op2)
declare float @llvm.copysign.f32(float %mag, float %sgn)

; 0: .data <.data>
@data_0 = global [4 x i8] [i8 3, i8 2, i8 1, i8 0]

define void @main(i32 %argc, i8** %argv) {
entry:
  %zero = alloca i32
  %ra = alloca i32
  %sp = alloca i32
  %gp = alloca i32
  %tp = alloca i32
  %t0 = alloca i32
  %t1 = alloca i32
  %t2 = alloca i32
  %s0 = alloca i32
  %s1 = alloca i32
  %a0 = alloca i32
  %a1 = alloca i32
  %a2 = alloca i32
  %a3 = alloca i32
  %a4 = alloca i32
  %a5 = alloca i32
  %a6 = alloca i32
  %a7 = alloca i32
  %s2 = alloca i32
  %s3 = alloca i32
  %s4 = alloca i32
  %s5 = alloca i32
  %s6 = alloca i32
  %s7 = alloca i32
  %s8 = alloca i32
  %s9 = alloca i32
  %s10 = alloca i32
  %s11 = alloca i32
  %t3 = alloca i32
  %t4 = alloca i32
  %t5 = alloca i32
  %t6 = alloca i32

  store i32 zeroinitializer, i32* %zero
  store i32 zeroinitializer, i32* %ra
  store i32 zeroinitializer, i32* %sp
  store i32 zeroinitializer, i32* %gp
  store i32 zeroinitializer, i32* %tp
  store i32 zeroinitializer, i32* %t0
  store i32 zeroinitializer, i32* %t1
  store i32 zeroinitializer, i32* %t2
  store i32 zeroinitializer, i32* %s0
  store i32 zeroinitializer, i32* %s1
  store i32 zeroinitializer, i32* %a0
  store i32 zeroinitializer, i32* %a1
  store i32 zeroinitializer, i32* %a2
  store i32 zeroinitializer, i32* %a3
  store i32 zeroinitializer, i32* %a4
  store i32 zeroinitializer, i32* %a5
  store i32 zeroinitializer, i32* %a6
  store i32 zeroinitializer, i32* %a7
  store i32 zeroinitializer, i32* %s2
  store i32 zeroinitializer, i32* %s3
  store i32 zeroinitializer, i32* %s4
  store i32 zeroinitializer, i32* %s5
  store i32 zeroinitializer, i32* %s6
  store i32 zeroinitializer, i32* %s7
  store i32 zeroinitializer, i32* %s8
  store i32 zeroinitializer, i32* %s9
  store i32 zeroinitializer, i32* %s10
  store i32 zeroinitializer, i32* %s11
  store i32 zeroinitializer, i32* %t3
  store i32 zeroinitializer, i32* %t4
  store i32 zeroinitializer, i32* %t5
  store i32 zeroinitializer, i32* %t6

  store i32 %argc, i32* %a0

  %ft0 = alloca float
  %ft1 = alloca float
  %ft2 = alloca float
  %ft3 = alloca float
  %ft4 = alloca float
  %ft5 = alloca float
  %ft6 = alloca float
  %ft7 = alloca float
  %fs0 = alloca float
  %fs1 = alloca float
  %fa0 = alloca float
  %fa1 = alloca float
  %fa2 = alloca float
  %fa3 = alloca float
  %fa4 = alloca float
  %fa5 = alloca float
  %fa6 = alloca float
  %fa7 = alloca float
  %fs2 = alloca float
  %fs3 = alloca float
  %fs4 = alloca float
  %fs5 = alloca float
  %fs6 = alloca float
  %fs7 = alloca float
  %fs8 = alloca float
  %fs9 = alloca float
  %fs10 = alloca float
  %fs11 = alloca float
  %ft8 = alloca float
  %ft9 = alloca float
  %ft10 = alloca float
  %ft11 = alloca float

  store float zeroinitializer, float* %ft0
  store float zeroinitializer, float* %ft1
  store float zeroinitializer, float* %ft2
  store float zeroinitializer, float* %ft3
  store float zeroinitializer, float* %ft4
  store float zeroinitializer, float* %ft5
  store float zeroinitializer, float* %ft6
  store float zeroinitializer, float* %ft7
  store float zeroinitializer, float* %fs0
  store float zeroinitializer, float* %fs1
  store float zeroinitializer, float* %fa0
  store float zeroinitializer, float* %fa1
  store float zeroinitializer, float* %fa2
  store float zeroinitializer, float* %fa3
  store float zeroinitializer, float* %fa4
  store float zeroinitializer, float* %fa5
  store float zeroinitializer, float* %fa6
  store float zeroinitializer, float* %fa7
  store float zeroinitializer, float* %fs2
  store float zeroinitializer, float* %fs3
  store float zeroinitializer, float* %fs4
  store float zeroinitializer, float* %fs5
  store float zeroinitializer, float* %fs6
  store float zeroinitializer, float* %fs7
  store float zeroinitializer, float* %fs8
  store float zeroinitializer, float* %fs9
  store float zeroinitializer, float* %fs10
  store float zeroinitializer, float* %fs11
  store float zeroinitializer, float* %ft8
  store float zeroinitializer, float* %ft9
  store float zeroinitializer, float* %ft10
  store float zeroinitializer, float* %ft11

  br label %label_0

; 0: .text <_start>
label_0:
  ; J { address: Address(0), addr: Address(0) }
  br label %label_0
}
