# riscv2llvm

`riscv2llvm` is a binary translator that lifts RISC-V to LLVM IR. Currently it can translate little-endian statically-linked Linux executable files compiled in RV64GC. The system call module supports x86_64, RV64GC and arm64 as target ISAs.

## Quick Start

``` shell
# Disassemble the target binary.
llvm-objdump -fhtDz --mattr=a,c,d,f,m example > example.dump

# For binaries compiled with glibc, we also need the contents of the `.tdata` section.
llvm-objdump -sj.tdata example > example.tdata

# Perform the translation.
riscv2llvm --arch=x86_64 example.dump example.tdata

# Compile the translated LLVM IR to a native binary.
# Static linking is necessary if `.tdata` is provided as input.
clang --static example.ll -lm
```

## Source Code Substitution

no literal struct as args/rets
no var args

Suppose you are translating the `mcf_s` benchmark in SPEC CPU 2017, and you have had `mcf_s.dump`, `mcf_s.tdata`, and `spec_qsort.c` in the current directory, then the following commands will substitute the `spec_qsort` function.

``` shell
clang -Xclang -no-opaque-pointers -O1 -S -emit-llvm spec_qsort.c

riscv2llvm --arch=x86_64 mcf_s.dump mcf_s.tdata --ir-funcs spec_qsort --ir-files spec_qsort.ll

clang --static mcf_s.ll spec_qsort.transed.ll -lm
```

## Notes

1. `riscv2llvm` is implemented based on RISC-V unprivileged ISA specification (version 20191213) and tested with Rust 1.72.0 and LLVM 15.0.7.
2. System call support for x86_64 is maturer than RV64GC, and binaries compiled for RV64GC can work on arm64 directly.
3. Convert LLVM IR to LLVM bitcode by using `llvm-as` can significantly reduce the file size.
4. Try to turn off optimization if the translated binary does not function properly.
5. Currently unsupported features are listed below:
    - All CSR are ignored. Instructions reading them always return 0, and instructions writting to them are ignored.
    - All rounding modes are ignored, except RDN and RUP for FP-to-Int conversion instructions.
    - `fclass` always returns 0.
6. [These entries](https://github.com/torvalds/linux/blob/7cd60e43a6def40ecb75deb8decc677995970d0b/include/uapi/linux/auxvec.h) in the auxiliary vector are properly initialized.
7. System calls are implemented based on [this](https://github.com/riscv-software-src/riscv-pk/blob/7e9b671c0415dfd7b562ac934feb9380075d4aa2/pk/syscall.h) and [this](https://chromium.googlesource.com/chromiumos/docs/+/a2622281357e45f2b2c74cdc4b428b0d1294488d/constants/syscalls.md). Also pay attention to a few quirks listed below:
    - The address hint in the first argument of `mmap` is always set to 0.
    - `mprotect` is ignored and always return 0, because it fails even for legal input in RV64GC.
    - The layout of the `stat` structure is automatically converted between [RV64GC](https://github.com/riscv-collab/riscv-gnu-toolchain/blob/baefbdd8bcedfabf0cf89dce679a8bd1a9f27b39/linux-headers/include/asm-generic/stat.h) and [x86_64](https://github.com/torvalds/linux/blob/6f52b16c5b29b89d92c0e7236f4655dc8491ad70/arch/x86/include/uapi/asm/stat.h).

## C Source Code for Supporting Functions

``` c
#include <stdint.h>

int8_t* copy_envp(int8_t* host_envp[], int8_t* guest_envp[]) {
    while (*host_envp) {
        *guest_envp++ = *host_envp++;
    }
    return (int8_t*)++guest_envp;
}
```

``` c
#include <elf.h>
#include <stdint.h>
#include <sys/auxv.h>

void init_auxv(int64_t* auxv, int8_t* phdr, int64_t phdr_addr, int64_t tdata, int64_t tdata_len) {
    // Initialize `AT_PHDR`.
    Elf64_Phdr* host_phdr = (Elf64_Phdr*) getauxval(AT_PHDR);
    int64_t host_phnum = getauxval(AT_PHNUM);
    if (host_phdr && host_phnum) {
        Elf64_Phdr* guest_phdr = (Elf64_Phdr*) phdr;
        for (int64_t i = 0; i < host_phnum; ++i) {
            if (host_phdr->p_type == PT_TLS || host_phdr->p_type == PT_GNU_RELRO) {
                *guest_phdr = *host_phdr++;
                guest_phdr->p_vaddr = tdata;
                guest_phdr->p_filesz = tdata_len;
                guest_phdr->p_memsz = tdata_len;
                ++guest_phdr;
            } else {
                *guest_phdr++ = *host_phdr++;
            }
        }
        *auxv++ = AT_PHDR;
        *auxv++ = phdr_addr;
    }

    // Initialize other entries.
    #define CNT 23
    int64_t entries[CNT] = {
        0, 1, 2,
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
        23, 24, 25, 26,
        31,
        51,
    };
    for (int64_t i = 0; i < CNT; ++i) {
        int64_t entry = entries[i];
        int64_t value = getauxval(entry);
        if (value) {
            *auxv++ = entry;
            *auxv++ = value;
        }
    }
}
```

``` c
#include <stdbool.h>
#include <stdint.h>

int64_t rounding(double f, bool is_rdn) {
    int64_t i = f;
    if (i != f && f > 0 && !is_rdn) {
        return i + 1;
    } else if (i != f && f < 0 && is_rdn) {
        return i - 1;
    } else {
        return i;
    }
}
```

``` c
#include <stdint.h>

void mem_copy(int8_t* dest, int8_t* src, int64_t count) {
    for (int64_t i = 0; i < count; ++i) {
        *dest++ = *src++;
    }
}
```
