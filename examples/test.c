#include <stdio.h>

void f(int i) {
    switch(i)
    {
        case 1:
        {
            printf("Case 1\n");
            break;
        }
        case 2:
        {           
            printf("Case 2\n");
            break;
        }
        case 3:
        {
            printf("Case 3\n");
            break;
        }
        case 4:
        {
            printf("Case 4\n");
            break;
        }
        case 5:
        {
            printf("Case 5\n");
            break;
        }
        // case 6:
        // {
        //     printf("Case 6\n");
        //     break;
        // }
        // case 7:
        // {
        //     printf("Case 7\n");
        //     break;
        // }
        // case 8:
        // {
        //     printf("Case 8\n");
        //     break;
        // }
        // case 9:
        // {
        //     printf("Case 9\n");
        //     break;
        // }
        // case 10:
        // {
        //     printf("Case 10\n");
        //     break;
        // }
        default:
        {
            // printf("Nothing\n");
            break;
        }
    }
}

int main(void) {
    // printf("%d", 1);
    f(1);
    return 0;
}
