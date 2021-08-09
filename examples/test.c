int s(int n) {
    switch (n) {
        case 1:
            ++n;
            break;
        case 2:
            ++n;
            break;
        case 3:
            ++n;
            break;
        case 4:
            ++n;
            break;
        case 5:
            ++n;
            break;
    }
    return n;
}

int main(void) {
    int n = 0;
    for (int i = 0; i < 1; ++i) {
        ++n;
    }
    while (n < 2) {
        ++n;
    }
    do {
        ++n;
    } while (0);
    if (1) {
        n = s(n);
    }
    return n;  // `echo $?` => 4
}
