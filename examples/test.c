// int g1 = 1;
// int g2;

// int f(int n) {
//     switch (n) {
//         case 1:
//             n += 1;
//         case 2:
//             n += 2;
//             break;
//         case 3:
//             n += 3;
//             break;
//         case 4:
//             n += 4;
//             break;
//         case 5:
//             n += 5;
//             break;
//     }
//     return n;
// }

// int main(void) {
//     if (1) {
//         return f(1);
//     }
// }

#include <thread>
#include <vector>
#include <iostream>
#include <atomic>
 
// std::atomic_flag lock1 = 0;//ATOMIC_FLAG_INIT;
std::atomic_bool lock2 = 0;//ATOMIC_FLAG_INIT;
 
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
 
int main()
{
    // std::vector<std::thread> v;
    // for (int n = 0; n < 10; ++n) {
    //     v.emplace_back(f, n);
    // }
    // for (auto& t : v) {
    //     t.join();
    // }
    // while (lock1.test_and_set(std::memory_order_acquire)) {
    //         ; // spin
    // }
    lock2 = 1;
    // lock1.clear(std::memory_order_release); 
}
