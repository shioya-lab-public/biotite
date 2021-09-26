// #include <stdio.h>
#include <unistd.h>
#include <sys/syscall.h>

int main(void) {
    // printf("Hello, world!\n");
    return syscall(SYS_write, 1, "Hello World\n", 12);
    // return syscall(1, 1, "Hello World\n", 12);
}



