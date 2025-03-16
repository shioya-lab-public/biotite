# Biotite

Biotite is a static binary translator that lifts little-endian statically linked RV64GC Linux binaries to self-contained ISA-independent LLVM IR. Please refer to our [CC 2025 paper](https://dl.acm.org/doi/10.1145/3708493.3712693) and also [the presentation slides](./CC-2025-presentation.pdf) for an overview.

## Quick Start

Biotite is tested on x86_64 Ubuntu 24.04 using RISC-V GCC 13, LLVM 18, Rust 1.85.0.

```shell
# Prepare the input for Biotite.
$ echo "
#include <stdio.h>

int main(void) {
    printf(\"Hello world! \n\");
    return 0;
}
" > hello.c
$ riscv64-linux-gnu-gcc hello.c --static -o hello
$ llvm-objdump -fhtDz --no-print-imm-hex hello > hello.dump
# Run Biotite.
$ git clone https://github.com/shioya-lab-public/biotite
$ cd biotite
$ cargo run --release -- --arch=x86_64 ../hello.dump
# Compile the translated program.
$ cd ../hello.translated
$ export CLANG=clang
$ export OPT=0 # Clang's optimization level (0~3)
$ make
$ ./hello.translated # Hello world!
```

Here is another script translating [CoreMark](https://github.com/eembc/coremark) using free linkage.

```shell
$ git clone https://github.com/eembc/coremark
$ cd coremark
$ git checkout d5fad6bd094899101a4e5fd53af7298160ced6ab
$ export CC=riscv64-linux-gnu-gcc
$ make compile PORT_DIR=linux XCFLAGS=--static
$ llvm-objdump -fhtDz --no-print-imm-hex coremark.exe > coremark.dump
# `clang -S -emit-llvm *.c` compiles source code files to LLVM IR.
# Other options are program-specific and can be obtained when compiling CoreMark in previous steps.
$ clang -S -emit-llvm *.c -O2 -Ilinux -Iposix -I. -DFLAGS_STR=\""-O2 --static  -lrt"\" -DITERATIONS=0
$ git clone https://github.com/shioya-lab-public/biotite
$ cd biotite
# `--srcs` lists input LLVM IR files for free linkage.
$ cargo run --release -- --arch=x86_64 ../coremark.dump --srcs ../*.ll
$ cd ../coremark.translated
$ export CLANG=clang
$ export OPT=2
$ make
$ ./coremark.translated 0 0 102 500000 7 1 2000
```

## Option Reference

```shell
Usage: biotite [OPTIONS] <INPUT>

Arguments:
  <INPUT>

Options:
  -o, --output <OUTPUT>
      # The number of translated functions contained in a single translated module.
      #
      # 0 means grouping all translated functions in one module.
      # Multiple small modules generally can be compilated faster and allow parallel compilation by `make -j`.
      --module-size <MODULE_SIZE>       [default: 0]
      # The target ISA for the system call implementation and the image mapping optimization.
      #
      # Currently only `x86_64` is supported.
      --arch <ARCH>
      # Optimization options.
      #
      # Only one of the four options below should be set.
      # The default option is `--enable-all-opts`.
      # Each file in `src/opt/` defines an optimization pass of the same name.
      --enable-all-opts
      --disable-all-opts
      --enable-opts <ENABLE_OPTS>...
      --disable-opts <DISABLE_OPTS>...
      # Input LLVM IR files for free linkage.
      --srcs <SRCS>...
  -h, --help                            Print help
  -V, --version                         Print version
```

## Some Small Notes

1. Implementation is based on The RISC-V Instruction Set Manual Volume I: Unprivileged Architecture (Version 20240411).
2. Currently unsupported features:
    - All CSRs are unsupported. Reading them always returns 0. Writting to them is ignored.
    - All rounding modes are ignored, except RDN and RUP for FP-to-Int conversion instructions, which are implemented using a simple C function defined in `./utilities.c`.
    - `fclass` always returns 0.
3. [These entries](https://github.com/torvalds/linux/blob/7cd60e43a6def40ecb75deb8decc677995970d0b/include/uapi/linux/auxvec.h) in the auxiliary vector are properly initialized.
4. System calls are implemented based on [this](https://github.com/riscv-software-src/riscv-pk/blob/7e9b671c0415dfd7b562ac934feb9380075d4aa2/pk/syscall.h) and [this](https://chromium.googlesource.com/chromiumos/docs/+/a2622281357e45f2b2c74cdc4b428b0d1294488d/constants/syscalls.md). Also pay attention to a few quirks:
    - `mprotect` is ignored and always return 0, because it fails even for legal input on RV64GC.
    - The layout of the `stat` structure is automatically converted between [RV64GC](https://github.com/riscv-collab/riscv-gnu-toolchain/blob/baefbdd8bcedfabf0cf89dce679a8bd1a9f27b39/linux-headers/include/asm-generic/stat.h) and [x86_64](https://github.com/torvalds/linux/blob/6f52b16c5b29b89d92c0e7236f4655dc8491ad70/arch/x86/include/uapi/asm/stat.h).
5. When running translated Haskell programs, if you encounter random crashes, try to add `+RTS -V0 -RTS` when invoking the translated program.

Finally, if you find programs that Biotite cannot translate or if you want to add support for new RISC-V instructions/LLVM IR/system calls/..., please feel free to open an issue and we would love to provide more information and help.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
