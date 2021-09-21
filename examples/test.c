// #include <fenv.h>
// #pragma STDC FENV_ACCESS ON

// // store the original rounding mode
// int originalRounding = fegetround( );
// // establish the desired rounding mode
// fesetround((int)3072); //FE_TOWARDZERO
// // do whatever you need to do ...

// // ... and restore the original mode afterwards
// fesetround(originalRounding);

// int main(void) {
//     fesetround(FE_DOWNWARD);
//     // asm volatile("frcsr t0 \n");
    
//     return 0;
// }

// #include <stdio.h>
// #include <math.h>
// extern int aT;

double g1 = 1;
long g2;
long g3;

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
// #include <stdio.h>
int main() {
    // double n = 0;
    // for (int i = 0; i < 1; ++i) {
    //     ++n;
    // }
    // while (n < 2) {
    //     ++n;
    // }
    // do {
    //     ++n;
    // } while (0);
    // if (1) {
    //     s(n);
    // }
    // if (argc > (long)argv) {
    //     return 9;
    // }
    // asm volatile(
    //     "beq t0,t0,main\n"
    //     "bne t0,t0,main\n"
    //     "blt t0,t0,main\n"
    //     "bge t0,t0,main\n"
    //     "bltu t0,t0,main\n"
    //     "bgeu t0,t0,main\n"
    //     "beqz t0,main\n"
    //     "bnez t0,main\n"
    //     "blez t0,main\n"
    //     "bgez t0,main\n"
    //     "bltz t0,main\n"
    //     "bgtz t0,main\n"
    // );
    g2 = 999999999999;
    g3 = 999999999999;
    return g1+g2+g3;  // `echo $?` => 6
}

// int main(void) {
//     asm volatile("ecall\n");
//     return 0;
// }

// #include <thread>
// #include <vector>
// #include <iostream>
// #include <atomic>
 
// std::atomic_flag lock = ATOMIC_FLAG_INIT;
 
// void f(int n)
// {
//     for (int cnt = 0; cnt < 40; ++cnt) {
//         while (lock.test_and_set(std::memory_order_acquire)) {  // acquire lock
//         // Since C++20, it is possible to update atomic_flag's
//         // value only when there is a chance to acquire the lock.
//         // See also: https://stackoverflow.com/questions/62318642
//         #if defined(__cpp_lib_atomic_flag_test)
//             while (lock.test(std::memory_order_relaxed))        // test lock
//         #endif
//                 ; // spin
//         }
//         static int out{};
//         std::cout << n << ((++out % 40) == 0 ? '\n' : ' ');
//         lock.clear(std::memory_order_release);                  // release lock
//     }
// }
 
// int main()
// {
//     std::vector<std::thread> v;
//     for (int n = 0; n < 10; ++n) {
//         v.emplace_back(f, n);
//     }
//     for (auto& t : v) {
//         t.join();
//     }
// }