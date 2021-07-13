void f(char n) {
    switch (n) {
        case 'a':
            n += 1;
            break;
        case 'b':
            n += 2;
            break;
        case 'c':
            n += 2;
            break;
        case 'd':
            n += 2;
            break;
        case 'e':
            n += 2;
            break;
        case 'f':
            n += 2;
            break;
    }
    return;
}

int main(void) {
    // asm volatile(
    //     "jalr t0, a0, 1 \n"
    // );
    // f();
    // m();
    f('c');
    return 0;
}
