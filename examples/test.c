int g1 = 1;
int g2;

int f(int n) {
    switch (n) {
        case 1:
            n += 1;
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
    if (1) {
        return f(1);
    }
}
