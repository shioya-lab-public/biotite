; ABI: ilp32d

declare i32 @syscall(i32, ...)

declare double @llvm.sqrt.f64(double %op1)
declare double @llvm.fma.f64(double %op1, double %op2, double %op3)
declare double @llvm.fabs.f64(double %op1)
declare double @llvm.minimum.f64(double %op1, double %op2)
declare double @llvm.maximum.f64(double %op1, double %op2)
declare double @llvm.copysign.f64(double %mag, double %sgn)

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

  %ft0 = alloca double
  %ft1 = alloca double
  %ft2 = alloca double
  %ft3 = alloca double
  %ft4 = alloca double
  %ft5 = alloca double
  %ft6 = alloca double
  %ft7 = alloca double
  %fs0 = alloca double
  %fs1 = alloca double
  %fa0 = alloca double
  %fa1 = alloca double
  %fa2 = alloca double
  %fa3 = alloca double
  %fa4 = alloca double
  %fa5 = alloca double
  %fa6 = alloca double
  %fa7 = alloca double
  %fs2 = alloca double
  %fs3 = alloca double
  %fs4 = alloca double
  %fs5 = alloca double
  %fs6 = alloca double
  %fs7 = alloca double
  %fs8 = alloca double
  %fs9 = alloca double
  %fs10 = alloca double
  %fs11 = alloca double
  %ft8 = alloca double
  %ft9 = alloca double
  %ft10 = alloca double
  %ft11 = alloca double

  store double zeroinitializer, double* %ft0
  store double zeroinitializer, double* %ft1
  store double zeroinitializer, double* %ft2
  store double zeroinitializer, double* %ft3
  store double zeroinitializer, double* %ft4
  store double zeroinitializer, double* %ft5
  store double zeroinitializer, double* %ft6
  store double zeroinitializer, double* %ft7
  store double zeroinitializer, double* %fs0
  store double zeroinitializer, double* %fs1
  store double zeroinitializer, double* %fa0
  store double zeroinitializer, double* %fa1
  store double zeroinitializer, double* %fa2
  store double zeroinitializer, double* %fa3
  store double zeroinitializer, double* %fa4
  store double zeroinitializer, double* %fa5
  store double zeroinitializer, double* %fa6
  store double zeroinitializer, double* %fa7
  store double zeroinitializer, double* %fs2
  store double zeroinitializer, double* %fs3
  store double zeroinitializer, double* %fs4
  store double zeroinitializer, double* %fs5
  store double zeroinitializer, double* %fs6
  store double zeroinitializer, double* %fs7
  store double zeroinitializer, double* %fs8
  store double zeroinitializer, double* %fs9
  store double zeroinitializer, double* %fs10
  store double zeroinitializer, double* %fs11
  store double zeroinitializer, double* %ft8
  store double zeroinitializer, double* %ft9
  store double zeroinitializer, double* %ft10
  store double zeroinitializer, double* %ft11

  br label %label_0

; 0: .text <_start>
label_0:
  ; J { address: Address(0), addr: Address(0) }
  br label %label_0
}
