int main(void) {
    asm volatile(
        "jalr t0, a0, 1 \n"
    );
    return 0;
}
