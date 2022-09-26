# riscv2llvm

A binary translator that translates RISC-V to LLVM IR.

## Requirements

- RISC-V Linux ABI and system calls
- Little-endian statically-linked executable files
- Arch: RV64GC, ABI: LP64D

all CSR are ignored
all rm are ignored
fmv only sect result int val
fclass is not supported

## Testing Commands

``` Bash
llvm-objdump -fhtDz test
```

## RISC-V Notes

- The base integer instruction sets use a two’s-complement representation for signed integer values.
- The program counter `pc` holds the address of the current instruction
- Except for the 5-bit immediates used in CSR instructions (Chapter 9), immediates are always sign-extended
- The shift amount held in the lower 5 bits
- The target of `JALR` address is obtained by adding the sign-extended 12-bit I-immediate to the register rs1, then setting the least-significant bit of the result to zero.
- In RV64I, only the low 6 bits of rs2 are considered for the shift amount
- Unlike RISC-V, taking the remainder of a division by zero in LLVM is undefined behavior.
- All CSR instructions atomically read-modify-write a single CSR

CSR
- If rd=x0, then CSRRW shall not read the CSR and shall not cause any of the side effects that might occur on a CSR read.
- For both CSRRS and CSRRC, if rs1=x0, then the instruction will not write to the CSR at all, and so shall not cause any of the side effects that might otherwise occur on a CSR write.
- Both CSRRS and CSRRC always read the addressed CSR and cause any read side effects regardless of rs1 and rd fields.
-  For CSRRSI and CSRRCI, if the uimm[4:0] field is zero, then these instructions will not write to the CSR, and shall not cause any of the side effects that might otherwise occur on a CSR write. For CSRRWI, if rd=x0, then the instruction shall not read the CSR and shall not cause any of the side effects that might occur on a CSR read. Both CSRRSI and CSRRCI will always read the CSR and cause any read side effects regardless of rd and rs1 fields.

Counter
- RDCYCLE: The execution environment should provide a means to determine the current rate (cycles/second) at which the cycle counter is incrementing.
- RDTIME: The execution environment should provide a means of determining the period of the real-time counter (seconds/tick). The environment should provide a means to determine the accuracy of the clock.

## Reference

``` llvm
declare dso_local void @exit(i32)
declare dso_local i32 @printf(i8*, ...)
@.str.d = private unnamed_addr constant [14 x i8] c"#value: %lx#\0A\00", align 1

%val = load i64, i64* @.a0
call i32 (i8*, ...) @printf(i8* getelementptr ([14 x i8], [14 x i8]* @.str.d, i64 0, i64 0), i64 %val)
call void @exit(i32 0)

clang -static t.c --target=riscv64 -march=rv64gc --gcc-toolchain=/opt/riscv64-elf-ubuntu-20.04-nightly-2022.06.10-nightly --sysroot=/opt/riscv64-elf-ubuntu-20.04-nightly-2022.06.10-nightly/riscv64-unknown-elf
```

## syscall

- <https://github.com/riscv-software-src/riscv-pk/blob/7e9b671c0415dfd7b562ac934feb9380075d4aa2/pk/syscall.h>
- <https://chromium.googlesource.com/chromiumos/docs/+/a2622281357e45f2b2c74cdc4b428b0d1294488d/constants/syscalls.md>

## auxvec

- <https://github.com/torvalds/linux/blob/master/include/uapi/linux/auxvec.h>
- <http://articles.manugarg.com/aboutelfauxiliaryvectors.html>
- <https://man7.org/linux/man-pages/man3/getauxval.3.html>
