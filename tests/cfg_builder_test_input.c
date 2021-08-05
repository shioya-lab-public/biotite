void s(char n)
{
    switch (n)
    {
    case '1':
        break;
    case '2':
        break;
    case '3':
        break;
    case '4':
        break;
    case '5':
        break;
    }
}

void f(void)
{
    asm volatile(
        "beq a0, a1, f \n"
        "bge a0, a1, f \n"
        "bgeu a0, a1, f \n"
        "blt a0, a1, f \n"
        "bltu a0, a1, f \n"
        "bne a0, a1, f \n"
        "beqz a0, f \n"
        "bnez a0, f \n"
        "j f \n");
}

int main(void)
{
    s('1');
    f();
    return 0;
}
