# riscv2llvm

sudo docker start -i straight-env
cd /work/straight-util/STRAIGHT_Tester/HelloMusl
make test -j$(nproc)


riscv64-unknown-linux-gnu-gcc -static -g <file>
riscv64-unknown-linux-gnu-objdump -d <file>

clang -emit-llvm test.c -S -o test.ll

-march=rv64imafdc -mabi=lp64d

https://releases.llvm.org/12.0.0/docs/LangRef.html
https://mukulrathi.co.uk/create-your-own-programming-language/llvm-ir-cpp-api-tutorial/


entry point
    all gcc supported optimization level will preserve the <main> tag
switch?
    AMT, user code only
stack model
    convert to standard llvm convention so it won't interfere with STRAIGHT calling convention
        Simulate sp, fp, and the stack in the compilation, then emit LLVM alloca properly.
heap model
    heap is a library feature
sys call
    identify and skip lib functions? -> ✔
        pro: directly translate to LLVM IR
        con: many engineering effort
        con: cannot handle inline sys call
    translate bare ecall? -> straight backend does not handle RISC-V assembly, MCTOLL also uses libc directly
        con: less intuitive LLVM IR
        pro: less engieering effort
        pro: handle inline sys call
    