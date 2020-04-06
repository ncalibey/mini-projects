#include <stdio.h>

int main()
{
    int c;

    c = getchar() != EOF;
    if (c == 0 || c == 1)
    {
        printf("c is %d\n", c);
    }
}
