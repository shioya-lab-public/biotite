int f(int n) {
    switch (n) {
        case 1:
            n += 1;
            break;
        case 2:
            n += 2;
            break;
        case 3:
            n += 3;
            break;
        case 4:
            n += 4;
            break;
        case 5:
            n += 5;
            break;
    }
    return n;
}

int main(void) {
    for (int i = 0; i < 100000000; ++i) {
        f(1);
    }
    return 0;
}
