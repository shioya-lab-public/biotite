void rv64i(void) {
    asm volatile(
        "add t0, a0, a1 \n"
        "addw t0, a0, a1 \n"
        "addi t0, a0, 1 \n"
        "addiw t0, a0, 1 \n"
        "and t0, a0, a1 \n"
        "andi t0, a0, 1 \n"
        "auipc t0, 1 \n"
        "beq a0, a1, main \n"
        "bge a0, a1, main \n"
        "bgeu a0, a1, main \n"
        "blt a0, a1, main \n"
        "bltu a0, a1, main \n"
        "bne a0, a1, main \n"
        // `csrrc` is not implemented.
        // `csrrci` is not implemented.
        // `csrrs` is not implemented.
        // `csrrsi` is not implemented.
        // `csrrw` is not implemented.
        // `csrrwi` is not implemented.
        "ebreak \n"
        "ecall \n"
        // `fence` is not implemented.
        // `fence.i` is not implemented.
        "jal ra, main \n"
        "jalr ra, a0, 1 \n"
        "lb t0, 1(a0) \n"
        "lbu t0, 1(a0) \n"
        "ld t0, 1(a0) \n"
        "lh t0, 1(a0) \n"
        "lhu t0, 1(a0) \n"
        "lui t0, 1 \n"
        "lw t0, 1(a0) \n"
        "lwu t0, 1(a0) \n"
        "or t0, a0, a1 \n"
        "ori t0, a0, 1 \n"
        "sb a1, 1(a0) \n"
        "sd a1, 1(a0) \n"
        "sh a1, 1(a0) \n"
        "sll t0, a0, a1 \n"
        "sllw t0, a0, a1 \n"
        "slli t0, a0, 1 \n"
        "slliw t0, a0, 1 \n"
        "slt t0, a0, a1 \n"
        "slti t0, a0, 1 \n"
        "sltiu t0, a0, 2 \n" // `sltiu t0, a0, 1` will be compiled to `seqz	t0,a0`.
        "sltu t0, a0, a1 \n"
        "sra t0, a0, a1 \n"
        "sraw t0, a0, a1 \n"
        "srai t0, a0, 1 \n"
        "sraiw t0, a0, 1 \n"
        "srl t0, a0, a1 \n"
        "srlw t0, a0, a1 \n"
        "srli t0, a0, 1 \n"
        "srliw t0, a0, 1 \n"
        "sub t0, a0, a1 \n"
        "subw t0, a0, a1 \n"
        "sw a1, 1(a0) \n"
        "xor t0, a0, a1 \n"
        "xori t0, a0, 1 \n"
    );
}

void pseudo(void) {
    asm volatile(
        "beqz a0, main \n"
        "bnez a0, main \n"
        // `fabs.s` is not implemented.
        // `fabs.d` is not implemented.
        // `fmv.s` is not implemented.
        // `fmv.d` is not implemented.
        // `fneg.s` is not implemented.
        // `fneg.d` is not implemented.
        "j main \n"
        "jr a0 \n"
        "la t0, main \n"
        "li t0, 1 \n"
        "mv t0, a0 \n"
        "neg t0, a0 \n"
        "nop \n"
        "not t0, a0 \n"
        "ret \n"
        "seqz t0, a0 \n"
        "snez t0, a0 \n"
    );
}

void registers(void) {
    asm volatile(
        "add t0, zero, zero \n"
        "add t0, ra, ra \n"
        "add t0, sp, sp \n"
        "add t0, gp, gp \n"
        "add t0, tp, tp \n"
        "add t0, t0, t0 \n"
        "add t0, t1, t1 \n"
        "add t0, t2, t2 \n"
        "add t0, s0, s0 \n"
        "add t0, s1, s1 \n"
        "add t0, a0, a0 \n"
        "add t0, a1, a1 \n"
        "add t0, a2, a2 \n"
        "add t0, a3, a3 \n"
        "add t0, a4, a4 \n"
        "add t0, a5, a5 \n"
        "add t0, a6, a6 \n"
        "add t0, a7, a7 \n"
        "add t0, s2, s2 \n"
        "add t0, s3, s3 \n"
        "add t0, s4, s4 \n"
        "add t0, s5, s5 \n"
        "add t0, s6, s6 \n"
        "add t0, s7, s7 \n"
        "add t0, s8, s8 \n"
        "add t0, s9, s9 \n"
        "add t0, s10, s10 \n"
        "add t0, s11, s11 \n"
        "add t0, t3, t3 \n"
        "add t0, t4, t4 \n"
        "add t0, t5, t5 \n"
        "add t0, t6, t6 \n"
    );
}

int main(void) {
    return 0;
}
