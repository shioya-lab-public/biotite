; ABI: ilp32

declare i32 @syscall(i32, ...)

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

  br label %label_0

; 0: .text <_start>
label_0:
  ; J { address: Address(0), addr: Address(0) }
  br label %label_0
}
