#include <stdio.h>

int main()
{
    int c, nl, b, t;
    nl = b = t = 0;

    while ((c = getchar()) != EOF)
        if (c == '\n')
        {
            nl++;
        }
        else if (c == '\t')
        {
            t++;
        }
        else if (c == ' ')
        {
            b++;
        }
    printf("newlines: %d\ntabs:%d\nblanks:%d\n", nl, t, b);
}
