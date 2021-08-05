# riscv2llvm

A binary translator that translates RISC-V to LLVM IR.

https://github.com/riscv/riscv-gnu-toolchain/releases/tag/2021.06.26
https://github.com/llvm/llvm-project/releases/tag/llvmorg-12.0.0

``` bash
clang -emit-llvm examples/test.c -S -o examples/reference.ll

riscv64-unknown-linux-gnu-gcc examples/test.c -o examples/test
riscv64-unknown-linux-gnu-objdump -d -j.text -j.rodata examples/test > examples/test.dump

cargo run -- examples/test.dump -o examples/test.ll
lli examples/test.ll
```

https://releases.llvm.org/12.0.0/docs/LangRef.html
https://mukulrathi.co.uk/create-your-own-programming-language/llvm-ir-cpp-api-tutorial/
