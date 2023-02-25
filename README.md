# riscv2llvm

`riscv2llvm` is a binary translator that lifts RISC-V to LLVM IR. Currently it can translate little-endian statically-linked Linux executable files compiled with RV64GC and LP64D ABI. It is tested with Rust 1.67.1 and LLVM 15.0.7.

## Quick Start

``` Bash
# Disassemble the file waiting for translation
llvm-objdump -fhtDz --mattr=a,c,d,f,m test > test.dump

# For binaries compiled with glibc, we also need the contents of the `.tdata` section
llvm-objdump -sj.tdata test > test.tdata

# Translate the target file
riscv2llvm --arch=x86_64 test.dump test.tdata

# Convert LLVM IR to bitcode can significantly reduce the file size, as clang cannot handle very large IR files
llvm-as test.ll

# Compile the translated file to a native binary
# Static linking is necessary if `.tdata` is provided as input
clang --static test.bc -lm
```

## Notes

1. All CSR are ignored.
2. All rm are ignored, except RDN and RUP for all 8 fp-to-int instructions.
3. `fclass` is not supported
4. AUXV initialization is based on [this](https://github.com/torvalds/linux/blob/7cd60e43a6def40ecb75deb8decc677995970d0b/include/uapi/linux/auxvec.h)
5. System calls are implemented based on [riscv](https://github.com/riscv-software-src/riscv-pk/blob/7e9b671c0415dfd7b562ac934feb9380075d4aa2/pk/syscall.h) and [x86_64](https://chromium.googlesource.com/chromiumos/docs/+/a2622281357e45f2b2c74cdc4b428b0d1294488d/constants/syscalls.md)
    - Ignore the address hint in `arg1` in `mmap`
    - mprotect always return 0, because it fails for legal input in RISC-V
    - `getmainvars` is not available in x86_64
    - readlinkat will change the return value to -22 if its -1, as RISC-V seems to require this particular value
    - Adjust the layout of `struct stat` based on [x86_64](https://github.com/torvalds/linux/blob/6f52b16c5b29b89d92c0e7236f4655dc8491ad70/arch/x86/include/uapi/asm/stat.h) and [riscv](https://github.com/riscv-collab/riscv-gnu-toolchain/blob/baefbdd8bcedfabf0cf89dce679a8bd1a9f27b39/linux-headers/include/asm-generic/stat.h)
6. Try to turn off optimization if the translated binary does not function properly
    - For `perlbench_s`
        - `ulimit -s 81920` (refspeed only)
        - disable `native_stack_vars`

## src subs

## C Source Code for Supporting Functions

``` C
#include <elf.h>
#include <stdint.h>
#include <sys/auxv.h>

void init_auxv(int64_t* auxv, int8_t* phdr, int64_t phdr_addr, int64_t tdata) {
    // Initialize `AT_PHDR`
    Elf64_Phdr* host_phdr = (Elf64_Phdr*) getauxval(AT_PHDR);
    int64_t host_phnum = getauxval(AT_PHNUM);
    if (host_phdr && host_phnum) {
        Elf64_Phdr* guest_phdr = (Elf64_Phdr*) phdr;
        for (int64_t i = 0; i < host_phnum; ++i) {
            if (host_phdr->p_type == PT_TLS) {
                *guest_phdr = *host_phdr++;
                guest_phdr->p_vaddr = tdata;
                ++guest_phdr;
            } else if (host_phdr->p_type == PT_GNU_RELRO) {
                *guest_phdr = *host_phdr++;
                guest_phdr->p_vaddr = tdata;
                guest_phdr->p_memsz = 0xac8;
                ++guest_phdr;
            } else {
                *guest_phdr++ = *host_phdr++;
            }
        }
        *auxv++ = AT_PHDR;
        *auxv++ = phdr_addr;
    }

    // Initialize other entries
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

``` C
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

``` C
#include <stdint.h>

void mem_copy(int8_t* dest, int8_t* src, int64_t count) {
    for (int64_t i = 0; i < count; ++i) {
        *dest++ = *src++;
    }
}
```
