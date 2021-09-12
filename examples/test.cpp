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

int g;

double s(double n) {
    double f = n;
    f = abs(f);
    f = -f;
    switch ((int)n) {
        case 1:
            n += 1;
            g += 9;
            break;
        case 2:
            n += 2;
            g += 9;
            break;
        case 3:
            n += 3;
            g += 9;
            break;
        case 4:
            n += 4;
            g += 9;
            break;
        case 5:
            n += 5;
            g += 9;
            break;
    }
    if (n > 0) {
        return 3;
    }
    return n+f;
}

int main(int argc, char** argv) {
    double n = 0;
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
        s(n);
    }
    if (argc > (long)argv) {
        return 9;
    }
    asm volatile(
        "nop\n"
        "nop\n"
        "fabs.s	fa0,fa5\n"
        
        
        "call main\n"
        "tail main\n"
        "nop\n"
        "nop\n"
        
        // "sgtz	a4,a5\n"
        // "fmv.x.d	t0,ft0\n"
        // "fcvt.d.l	fa5,a5\n"
        // "fcvt.d.l	fa5,a5,rtz\n"
        // "fcvt.d.lu	fa5,a5\n"
        // "fcvt.d.lu	fa5,a5,rtz\n"
        // "fmv.d.x	ft0,t0\n"
        // "fsub.d	fa3,fa4,fa5\n"
        // "fsub.d	fa3,fa4,fa5,rtz\n"
        // "fmul.d	fa3,fa4,fa5\n"
        // "fmul.d	fa3,fa4,fa5,rtz\n"
        // "fdiv.d	fa3,fa4,fa5\n"
        // "fdiv.d	fa3,fa4,fa5,rtz\n"
        // "fsqrt.d	fa0,fa1\n"
        // "fsqrt.d	fa0,fa1,rtz\n"
        // "fsgnj.d	ft0,ft1,ft2\n"
        // "fsgnjn.d	ft0,ft1,ft2\n"
        // "fsgnjx.d	ft0,ft1,ft2\n"
        // "fmin.d	ft0,ft1,ft2\n"
        // "fmax.d	ft0,ft1,ft2\n"
        // "fcvt.s.d	fa5,fa5,rtz\n"
        // "fcvt.d.s	fa5,fa5,rtz\n"
        // "amoand.d.aqrl	t1,t0,(a0)\n"
        // "amoor.d	t1,t0,(a0)\n"
        // "amoor.d.aq	t1,t0,(a0)\n"
        // "amoor.d.rl	t1,t0,(a0)\n"
        // "amoor.d.aqrl	t1,t0,(a0)\n"
        // "amomin.d	t1,t0,(a0)\n"
        // "amomin.d.aq	t1,t0,(a0)\n"
        // "amomin.d.rl	t1,t0,(a0)\n"
        // "amomin.d.aqrl	t1,t0,(a0)\n"
        // "amomax.d	t1,t0,(a0)\n"
        // "amomax.d.aq	t1,t0,(a0)\n"
        // "amomax.d.rl	t1,t0,(a0)\n"
        // "amomax.d.aqrl	t1,t0,(a0)\n"
        // "amominu.d	t1,t0,(a0)\n"
        // "amominu.d.aq	t1,t0,(a0)\n"
        // "amominu.d.rl	t1,t0,(a0)\n"
        // "amominu.d.aqrl	t1,t0,(a0)\n"
        // "amomaxu.d	t1,t0,(a0)\n"
        // "amomaxu.d.aq	t1,t0,(a0)\n"
        // "amomaxu.d.rl	t1,t0,(a0)\n"
        // "amomaxu.d.aqrl	t1,t0,(a0)\n"
    );
    
    return 3+g;  // `echo $?` => 6
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