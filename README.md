# Biotite

Biotite is a binary translator that lifts little-endian statically linked RV64GC Linux binaries to self-contained ISA-independent LLVM IR. Please refer to our [CC25 paper](https://dl.acm.org/doi/10.1145/3708493.3712693) for an overview.

## Quick Start

This script is tested in x86_64 Ubuntu 24.04 using RISC-V GCC 13, LLVM 18, Rust 1.85.0. Code cleanup, a command line argument reference, and instructions for extending Biotite will be completed within March 2025.

``` sh
$ echo "
#include <stdio.h>

int main(void) {
    printf(\"Hello, world! \n\");
    return 0;
}" > example.c
$ riscv64-linux-gnu-gcc example.c -static -o example
$ llvm-objdump -fhtDz --no-print-imm-hex example > example.dump
$ git clone https://github.com/shioya-lab-public/biotite
$ cd biotite
$ cargo run --release -- --arch=x86_64 ../example.dump
$ cd ../example.translated
$ export CLANG=clang
$ export OPT=0
$ make
$ ./example.translated # Hello, world! 
```

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
