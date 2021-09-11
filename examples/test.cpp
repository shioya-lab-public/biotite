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
#include <math.h>
// extern int aT;

float s(float a, float b, float c) {
    // switch (n) {
    //     case 1:
    //         n += 1;
    //         break;
    //     case 2:
    //         n += 2;
    //         break;
    //     case 3:
    //         n += 3;
    //         break;
    //     case 4:
    //         n += 4;
    //         break;
    //     case 5:
    //         n += 5;
    //         break;
    // }
    return a>b?a:b;
}

int main(int argc, char** argv) {
    float n = 0;
//     // for (int i = 0; i < 1; ++i) {
//     //     ++n;
//     // }
//     // while (n < 2) {
//     //     ++n;
//     // }
//     // do {
//     //     ++n;
//     // } while (0);
//     // if (1) {
    // float f = 0;
    // int i = argc;
    // // float *fp = &f;
    // int *ip = &i;
    // float* fp = reinterpret_cast<float*>(ip);
    // *fp += 2.3;
    // #include <cmath>
    // float a, b;
    // if (isfinite(a)) {
    //     return 3;
    // }
    long i = 0;
    float f = (float)i;
//     // }
    asm volatile(
        "fcvt.l.s	a5,fa5\n"
        "fcvt.lu.s	a5,fa5\n"
        "fcvt.s.l	fa5,a5,rtz\n"
        "fcvt.s.lu	fa5,a5,rtz\n"
        // "fadd.s	fa3,fa4,fa5,rtz\n"
        // "fsub.s	fa3,fa4,fa5,rtz\n"
        // "fmul.s	fa3,fa4,fa5,rtz\n"
        // "fdiv.s	fa3,fa4,fa5,rtz\n"
        // "fsqrt.s	fa0,fa1,rtz\n"
    //     "sc.d.aqrl	t0,a2,(a0)\n"
    //     "amoswap.d	t1,t0,(a0)\n"
    //     "amoswap.d.aq	t1,t0,(a0)\n"
    //     "amoswap.d.rl	t1,t0,(a0)\n"
    //     "amoswap.d.aqrl	t1,t0,(a0)\n"
    //     "amoadd.d	t1,t0,(a0)\n"
    //     "amoadd.d.aq	t1,t0,(a0)\n"
    //     "amoadd.d.rl	t1,t0,(a0)\n"
    //     "amoadd.d.aqrl	t1,t0,(a0)\n"
    //     "amoxor.d	t1,t0,(a0)\n"
    //     "amoxor.d.aq	t1,t0,(a0)\n"
    //     "amoxor.d.rl	t1,t0,(a0)\n"
    //     "amoxor.d.aqrl	t1,t0,(a0)\n"
    //     "amoand.d	t1,t0,(a0)\n"
    //     "amoand.d.aq	t1,t0,(a0)\n"
    //     "amoand.d.rl	t1,t0,(a0)\n"
    //     "amoand.d.aqrl	t1,t0,(a0)\n"
    //     "amoor.d	t1,t0,(a0)\n"
    //     "amoor.d.aq	t1,t0,(a0)\n"
    //     "amoor.d.rl	t1,t0,(a0)\n"
    //     "amoor.d.aqrl	t1,t0,(a0)\n"
    //     "amomin.d	t1,t0,(a0)\n"
    //     "amomin.d.aq	t1,t0,(a0)\n"
    //     "amomin.d.rl	t1,t0,(a0)\n"
    //     "amomin.d.aqrl	t1,t0,(a0)\n"
    //     "amomax.d	t1,t0,(a0)\n"
    //     "amomax.d.aq	t1,t0,(a0)\n"
    //     "amomax.d.rl	t1,t0,(a0)\n"
    //     "amomax.d.aqrl	t1,t0,(a0)\n"
    //     "amominu.d	t1,t0,(a0)\n"
    //     "amominu.d.aq	t1,t0,(a0)\n"
    //     "amominu.d.rl	t1,t0,(a0)\n"
    //     "amominu.d.aqrl	t1,t0,(a0)\n"
    //     "amomaxu.d	t1,t0,(a0)\n"
    //     "amomaxu.d.aq	t1,t0,(a0)\n"
    //     "amomaxu.d.rl	t1,t0,(a0)\n"
    //     "amomaxu.d.aqrl	t1,t0,(a0)\n"
    );
    return 0;  // `echo $?` => 6
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