#include <stdio.h>

int f(int n) {
    if (n) {
        return n;
    } else {
        return n;
    }
}

int main(void) {
    printf("%d", f(0));
    return 0;
}
