# Biotite

Biotite is a binary translator that lifts little-endian statically linked RV64GC Linux binaries to self-contained ISA-independent LLVM IR with optional support for Linux system calls.

## Quick Start

``` sh
$ llvm-objdump -fhtDz --no-print-imm-hex example > example.dump
$ biotite --arch=x86_64 example.dump
$ cd example.translated
$ export CLANG=clang
$ export OPT=0
$ make
```

## Linking with Source Code

Suppose you are translating [CoreMark](https://github.com/eembc/coremark/archive/refs/tags/v1.01.tar.gz) and want to compile and link functions in `core_util.c` directly into the final output binary using native Clang. The following commands will do what you want, assuming you are currently inside the `coremark-1.01` folder and have followed instructions on Quick Start to get the `dump` file.

``` sh
$ clang -S -emit-llvm -I. -Ilinux64 core_util.c
$ biotite --arch=x86_64 coremark.dump --srcs core_util.ll
$ cd coremark.translated
$ export CLANG=clang
$ export OPT=0
$ make
```

## Notes

1. Implementation is based on RISC-V unprivileged ISA specification (version 20191213) and tested with Rust 1.78.0 and Clang/LLVM 17.0.6.
2. Currently, the system call support is only implemented for x86_64.
3. Currently unsupported features are listed below:
    - All CSR are ignored. Instructions reading them always return 0. Instructions writting to them are ignored.
    - All rounding modes are ignored, except RDN and RUP for FP-to-Int conversion instructions.
    - `fclass` always returns 0.
4. [These entries](https://github.com/torvalds/linux/blob/7cd60e43a6def40ecb75deb8decc677995970d0b/include/uapi/linux/auxvec.h) in the auxiliary vector are properly initialized.
5. System calls are implemented based on [this](https://github.com/riscv-software-src/riscv-pk/blob/7e9b671c0415dfd7b562ac934feb9380075d4aa2/pk/syscall.h) and [this](https://chromium.googlesource.com/chromiumos/docs/+/a2622281357e45f2b2c74cdc4b428b0d1294488d/constants/syscalls.md). Also pay attention to a few quirks listed below:
    - `mprotect` is ignored and always return 0, because it fails even for legal input on RV64GC.
    - The layout of the `stat` structure is automatically converted between [RV64GC](https://github.com/riscv-collab/riscv-gnu-toolchain/blob/baefbdd8bcedfabf0cf89dce679a8bd1a9f27b39/linux-headers/include/asm-generic/stat.h) and [x86_64](https://github.com/torvalds/linux/blob/6f52b16c5b29b89d92c0e7236f4655dc8491ad70/arch/x86/include/uapi/asm/stat.h).
