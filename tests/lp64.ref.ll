; ABI: lp64

declare i64 @syscall(i64, ...)

; 0: .data <.data>
@data_0 = global [4 x i8] [i8 3, i8 2, i8 1, i8 0]

define void @main(i32 %argc, i8** %argv) {
entry:
  %zero = alloca i64
  %ra = alloca i64
  %sp = alloca i64
  %gp = alloca i64
  %tp = alloca i64
  %t0 = alloca i64
  %t1 = alloca i64
  %t2 = alloca i64
  %s0 = alloca i64
  %s1 = alloca i64
  %a0 = alloca i64
  %a1 = alloca i64
  %a2 = alloca i64
  %a3 = alloca i64
  %a4 = alloca i64
  %a5 = alloca i64
  %a6 = alloca i64
  %a7 = alloca i64
  %s2 = alloca i64
  %s3 = alloca i64
  %s4 = alloca i64
  %s5 = alloca i64
  %s6 = alloca i64
  %s7 = alloca i64
  %s8 = alloca i64
  %s9 = alloca i64
  %s10 = alloca i64
  %s11 = alloca i64
  %t3 = alloca i64
  %t4 = alloca i64
  %t5 = alloca i64
  %t6 = alloca i64

  store i64 zeroinitializer, i64* %zero
  store i64 zeroinitializer, i64* %ra
  store i64 zeroinitializer, i64* %sp
  store i64 zeroinitializer, i64* %gp
  store i64 zeroinitializer, i64* %tp
  store i64 zeroinitializer, i64* %t0
  store i64 zeroinitializer, i64* %t1
  store i64 zeroinitializer, i64* %t2
  store i64 zeroinitializer, i64* %s0
  store i64 zeroinitializer, i64* %s1
  store i64 zeroinitializer, i64* %a0
  store i64 zeroinitializer, i64* %a1
  store i64 zeroinitializer, i64* %a2
  store i64 zeroinitializer, i64* %a3
  store i64 zeroinitializer, i64* %a4
  store i64 zeroinitializer, i64* %a5
  store i64 zeroinitializer, i64* %a6
  store i64 zeroinitializer, i64* %a7
  store i64 zeroinitializer, i64* %s2
  store i64 zeroinitializer, i64* %s3
  store i64 zeroinitializer, i64* %s4
  store i64 zeroinitializer, i64* %s5
  store i64 zeroinitializer, i64* %s6
  store i64 zeroinitializer, i64* %s7
  store i64 zeroinitializer, i64* %s8
  store i64 zeroinitializer, i64* %s9
  store i64 zeroinitializer, i64* %s10
  store i64 zeroinitializer, i64* %s11
  store i64 zeroinitializer, i64* %t3
  store i64 zeroinitializer, i64* %t4
  store i64 zeroinitializer, i64* %t5
  store i64 zeroinitializer, i64* %t6

  %argc_i64 = sext i32 %argc to i64
  store i64 %argc_i64, i64* %a0

  br label %label_0

; 0: .text <_start>
label_0:
  ; J { address: Address(0), raw: Raw(""), addr: Address(0) }
  br label %label_0
}
