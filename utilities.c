#include <stdint.h>

int8_t* copy_envp(int8_t* host_envp[], int8_t* guest_envp[]) {
    while (*host_envp) {
        *guest_envp++ = *host_envp++;
    }
    return (int8_t*)++guest_envp;
}

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

#include <stdint.h>

void mem_copy(int8_t* dest, int8_t* src, int64_t count) {
    for (int64_t i = 0; i < count; ++i) {
        *dest++ = *src++;
    }
}
