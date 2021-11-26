# riscv2llvm

A binary translator that translates RISC-V to LLVM IR.

## Quick Start (outdated)

`riscv2llvm` is precompiled as a standalone executable for Linux (not available until the release of `v0.1.0`) and is tested against [riscv-gnu-toolchain Nightly: September 21, 2021](https://github.com/riscv-collab/riscv-gnu-toolchain/releases/tag/2021.09.21) and [LLVM 13.0.0](https://github.com/llvm/llvm-project/releases/tag/llvmorg-13.0.0) in `Windows WSL 2 (Ubuntu 20.04.2)`.

Currently, `riscv2llvm` supports most RV64I instructions and is able to translate local functions with basic control flow as `examples/test.c` (below) shows.

``` C
// examples/test.c
int s(int n) {
    switch (n) {
        case 1:
            n += 1;
            break;
        case 2:
            n += 2;
            break;
        case 3:
            n += 3;
            break;
        case 4:
            n += 4;
            break;
        case 5:
            n += 5;
            break;
    }
    return n;
}

int main(void) {
    int n = 0;
    for (int i = 0; i < 1; ++i) {
        ++n;
    }
    while (n < 2) {
        ++n;
    }
    do {
        ++n;
    } while (0);
    if (1) {
        n = s(n);
    }
    return n;  // `echo $?` => 6
}
```

To translate `examples/test.c`, follow the commands below.

``` Bash
# Compile a RISC-V executable.
$ riscv64-unknown-linux-gnu-gcc examples/test.c -o examples/test
# Disassemble `.text` and `.rodata` sections of the executable.
$ riscv64-unknown-linux-gnu-objdump -d -j.text -j.rodata examples/test > examples/test.dump
# Translate the dumped assembly to LLVM IR.
$ ./riscv2llvm examples/test.dump -o examples/test.ll
# Test the translated LLVM IR with the LLVM interpreter.
$ lli examples/test.ll
$ echo $? # -> should be 6
```

## Implementation (outdated)

In this section, we will briefly explore the implmentation of `riscv2llvm` by comparing the RISC-V version and the translated LLVM version of the `switch` statement in the function `s` of `examples/test.c`.

The RISC-V version is shown below, GCC implements a `switch` statement which has at least 5 branches as a jump table. This jump table containing the start addresses for each branch resides in the `.rodata` section, and we can load the proper address dynamically based on the value given to the `switch` statement. In this program, the address is computed to be `105a0` (`add a5,a5,a4`), and from which we can load the correct branch address `10496` (`lw a5,0(a5)`). Finally, a indirect jump leads us to the the correct `switch` branch (`jr a5`).

``` RISC-V
0000000000010450 <s>:
   #---------- Omitted ----------#
   10478:	97ba                	add	a5,a5,a4    # 105a0
   1047a:	439c                	lw	a5,0(a5)    # 10496
   1047c:	8782                	jr	a5
   1047e:	fec42783          	lw	a5,-20(s0)
   10482:	2785                	addiw	a5,a5,1
   10484:	fef42623          	sw	a5,-20(s0)
   10488:	a80d                	j	104ba <s+0x6a>
   1048a:	fec42783          	lw	a5,-20(s0)
   1048e:	2789                	addiw	a5,a5,2
   10490:	fef42623          	sw	a5,-20(s0)
   10494:	a01d                	j	104ba <s+0x6a>
   10496:	fec42783          	lw	a5,-20(s0)      # target branch
   1049a:	278d                	addiw	a5,a5,3
   1049c:	fef42623          	sw	a5,-20(s0)
   104a0:	a829                	j	104ba <s+0x6a>
   104a2:	fec42783          	lw	a5,-20(s0)
   104a6:	2791                	addiw	a5,a5,4
   104a8:	fef42623          	sw	a5,-20(s0)
   104ac:	a039                	j	104ba <s+0x6a>
   104ae:	fec42783          	lw	a5,-20(s0)
   104b2:	2795                	addiw	a5,a5,5
   104b4:	fef42623          	sw	a5,-20(s0)
   104b8:	0001                	nop
   104ba:	fec42783          	lw	a5,-20(s0)
   104be:	853e                	mv	a0,a5
   104c0:	6462                	ld	s0,24(sp)
   104c2:	6105                	addi	sp,sp,32
   104c4:	8082                	ret

#---------- Omitted ----------#

0000000000010594 <.rodata>:
   10594:	04ba                	slli	s1,s1,0xe
   10596:	0001                	nop
   10598:	047e                	slli	s0,s0,0x1f
   1059a:	0001                	nop
   1059c:	048a                	slli	s1,s1,0x2
   1059e:	0001                	nop
   105a0:	0496                	slli	s1,s1,0x5   # target branch address
   105a2:	0001                	nop
   105a4:	04a2                	slli	s1,s1,0x8
   105a6:	0001                	nop
   105a8:	04ae                	slli	s1,s1,0xb
   105aa:	0001                	nop
```

Then, let's look at the translated LLVM version shown below. First, we can see all 32 RISC-V registers and the stack is defiend globally. Then, in the function `s`, we can see the `switch` statement in C is translated to a `switch` instruction in LLVM, and each branch is translated to one basic block (from `L2` to `L6`). This `switch` instruction basically says: Check the variable `%temp_56`, and if it equals to `66980` (one address in `.rodata` but expressed in decimal) then go to the label `L5`, ..., and if none of these matches, go to the label `L57`.

Then, let's look at the basic block marked as `L4`, which is the `switch` branch that actually get executed. In RISC-V, it contains 4 instructions: `lw`, `addiw`, `sw`, and `j`. The `j` instruction is translated to a unconditionally branch instruction in LLVM (`br`). The translation of the `addiw` instruction is also straightforward, but we must remember to load and store the global register variable. The `lw` and `sw` instructions are a little bit complex, so please refer to the line-by-line comments for more details.

``` LLVM
@zero = global i64 0
;---------- Omitted ----------;
@t6 = global i64 0

@stack = global [1024 x i8] zeroinitializer

define i64 @s() {
    ;---------- Omitted ----------;
    ; Add { result: A5, op1: A5, op2: A4 }
    %temp_46 = load i64, i64* @a5
    %temp_47 = load i64, i64* @a4
    %temp_48 = add i64 %temp_46, %temp_47
    store i64 %temp_48, i64* @a5

    ; Load { ty: I32, result: A5, op1: A5, op2: 0 }
    ; Switch { register: A5, targets: {66980: 5, 66984: 6, 66968: 2, 66964: 7, 66976: 4, 66972: 3} }
    %temp_56 = load i64, i64* @a5
    switch i64 %temp_56, label %L57 [ i64 66980, label %L5 i64 66984, label %L6 i64 66968, label %L2 i64 66964, label %L7 i64 66976, label %L4 i64 66972, label %L3 ]
L57:
    unreachable

L2:
    ;---------- Omitted ----------;
    br label %L7

L3:
    ;---------- Omitted ----------;
    br label %L7

L4:
    ; Load { ty: I32, result: A5, op1: S0, op2: -20 }
    ; Load the `s0` register.
    %temp_90 = load i64, i64* @s0
    ; Compute `s0 - 20`.
    %temp_91 = add i64 %temp_90, -20
    ; Unlike the real stack, our global stack grows from 0 to 1024.
    %temp_92 = sub i64 1023, %temp_91
    ; Get a pointer to the specific byte in the stack.
    %temp_93 = getelementptr [1024 x i8], [1024 x i8]* @stack, i8 0, i64 %temp_92
    ; Cast the byte pointer to a word pointer.
    %temp_94 = bitcast i8* %temp_93 to i32*
    ; Load a word from the stack.
    %temp_95 = load i32, i32* %temp_94
    ; Sign extend the word to a double word.
    %temp_96 = sext i32 %temp_95 to i64
    ; Store the loaded value to the register `a5`.
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

L5:
    ;---------- Omitted ----------;
    br label %L7

L6:
    ;---------- Omitted ----------;
    br label %L7

L7:
    ;---------- Omitted ----------;
    ; Ret
    %temp_156 = load i64, i64* @a0
    ret i64 %temp_156
}
```

## Misc

Use binary, source code, and debug info to achieve comparable performance with the native binary.

I suddenly realize that it seems that we do not really need to pass registers around. So we have 3 kinds of registers as below.
    (1). a few function arguments/return registers
    (2). 3 control registers: global pointer, stack pointer, and thread pointer
    (3). other temp registers no matter they are caller-saved or callee-saved
Then the following translation method seems to work just fine.
    (a). translate (1) into regular LLVM functions which will hopefully be mapped to corresponding physical registers in the host ISA
    (b). translate (2) in whichever simple method as there are only three of them and the most heavily used stack will be separately translated into real stack allocations using native LLVM instructions after all
    (c). just leave all other temp registers as local variables and do not sync them across functions
How do you think about this? Am I missing something important?

(2) (global context) は，関数の arguments/returns に追加で与えられる，関数の implicit な入出力だと考えることもできると思います

Pass registers as arguments to facilitate multithreading

global variables に fall back する方法

関数の最初で全ての global variables を local variables に読み出し，return 時に中身が書き換えられた local variables の中身を global variables へ書き出す方法があるかもしれません

元のソースコードの情報や Dwarf debug 情報を利用することで元のバイナリの ABI が特定できる場合，それに依存した処理を安全に行うことができるかもしれません

### Assumptions

- RISC-V Linux ABI
- little endian
- Statically linked
- `-march=rv64imafdc -mabi=lp64d`

### RISC-V Notes

- The base integer instruction sets use a two’s-complement representation for signed integer values.
- RV32I: 40 insts including `ecall`, `ebreak`, and `fence`
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

### Todo

- Do not assume executable file.
- Extract `.text`, `.rodata`, `.data`, `.bss`, `.sdata`, and `.sbss`.

- add arbitrary memory access support for ld/sd: Keep all static sections as static byte array in LLVM.
- add support for RV32/64A
- Force x0 always to be 0
- Fix static variable support
- Map RV inst to LLVM comment for easy debugging
- Struct and Arrays: At least for simple structs and arrays, they are compiled away and I do not notice anything special instruction sequence.
- Func pointer: The function addresss is computed in a variable, then `jalr`. Disassembly comment hint may available in previous instructions where the address is determined for the first time.
```
    105ac:	4e078793          	addi	a5,a5,1248 # 104e0 <s>
    105b0:	fef43023          	sd	a5,-32(s0)
    105b4:	fec42783          	lw	a5,-20(s0)
    105b8:	fe043703          	ld	a4,-32(s0)
    105bc:	853e                	mv	a0,a5
    105be:	9702                	jalr	a4
```
- Break static stack into stack space of seperate functions.

### Testing Commands

``` Bash
clang -emit-llvm examples/test.c -S -o examples/reference.ll

riscv64-unknown-linux-gnu-gcc -static test.c -o test
riscv64-unknown-linux-gnu-objdump -D -j.text -j.rodata -j.data -j.bss -j.sdata -j.sbss -wz test > test.dump

cargo run -- examples/test.dump -o examples/test.ll
lli examples/test.ll
echo $?
```

### STRAIGHT Commands

``` Bash
sudo dockerd &
sudo docker cp examples/test.ll straight-env:/work/straight-util/STRAIGHT_Tester/HelloMusl/src/main.ll
sudo docker start -i straight-env
cd /work/straight-util/STRAIGHT_Tester/HelloMusl && make test -j$(nproc)
exit
sudo docker cp straight-env:/work/straight-util/STRAIGHT_Tester/HelloMusl/hello.result ~/riscv2llvm/examples
```

### Reference

https://llvm.org/doxygen/SROA_8cpp.html
https://releases.llvm.org/13.0.0/docs/tutorial/MyFirstLanguageFrontend/LangImpl07.html

- [LLVM Language Reference Manual](https://releases.llvm.org/12.0.0/docs/LangRef.html)
- [A Complete Guide to LLVM for Programming Language Creators](https://mukulrathi.co.uk/create-your-own-programming-language/llvm-ir-cpp-api-tutorial/)
