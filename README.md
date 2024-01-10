# riscv2llvm

`riscv2llvm` is a binary translator that lifts statically-linked Linux executable files compiled in RV64GC to LLVM IR.

## Quick Start

``` sh
# Disassemble the target binary.
llvm-objdump -fhtDz --mattr=a,c,d,f,m example > example.dump

# For binaries compiled with glibc, we also need the contents of the `.tdata` section.
llvm-objdump -sj.tdata example > example.tdata

# Perform the translation.
riscv2llvm --sys-call=x86_64 --mem=x86_64 example.dump example.tdata

# Compile the translated LLVM IR to a native binary.
# Static linking is necessary if `.tdata` is provided as input.
clang --static example.ll example.s -T example.ld -lm
```
## cmd option ref
## Source Code Substitution

Suppose you are translating the `mcf_s` benchmark in SPEC CPU 2017, then the following commands will substitute functions in `spec_qsort.c`.

``` sh
export CLANG=clang

riscv2llvm --sys-call=x86_64 --mem=x86_64 mcf_s.dump mcf_s.tdata --srcs spec_qsort.c

clang --static mcf_s.ll mcf_s.s mcf_s.ir/spec_qsort.ll -T mcf_s.ld -lm
```

## Notes

1. `riscv2llvm` is implemented based on RISC-V unprivileged ISA specification (version 20191213) and tested with Rust 1.72.0 and LLVM 15.0.7.
2. System call support for x86_64 is maturer than riscv64gc, and binaries compiled for riscv64gc can work on aarch64 directly.
3. Convert LLVM IR to LLVM bitcode by using `llvm-as` can significantly reduce the file size if clang refuses to compile huge IR files.
4. Try to turn off optimization if the translated binary does not function properly.
5. Currently unsupported features are listed below:
    - All CSR are ignored. Instructions reading them always return 0, and instructions writting to them are ignored.
    - All rounding modes are ignored, except RDN and RUP for FP-to-Int conversion instructions.
    - `fclass` always returns 0.
6. [These entries](https://github.com/torvalds/linux/blob/7cd60e43a6def40ecb75deb8decc677995970d0b/include/uapi/linux/auxvec.h) in the auxiliary vector are properly initialized.
7. System calls are implemented based on [this](https://github.com/riscv-software-src/riscv-pk/blob/7e9b671c0415dfd7b562ac934feb9380075d4aa2/pk/syscall.h) and [this](https://chromium.googlesource.com/chromiumos/docs/+/a2622281357e45f2b2c74cdc4b428b0d1294488d/constants/syscalls.md). Also pay attention to a few quirks listed below:
    - The address hint in the first argument of `mmap` is always set to 0.
    - `mprotect` is ignored and always return 0, because it fails even for legal input on riscv64gc.
    - The layout of the `stat` structure is automatically converted between [riscv64gc](https://github.com/riscv-collab/riscv-gnu-toolchain/blob/baefbdd8bcedfabf0cf89dce679a8bd1a9f27b39/linux-headers/include/asm-generic/stat.h) and [x86_64](https://github.com/torvalds/linux/blob/6f52b16c5b29b89d92c0e7236f4655dc8491ad70/arch/x86/include/uapi/asm/stat.h).
