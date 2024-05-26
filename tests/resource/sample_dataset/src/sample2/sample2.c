#include <stdio.h>
#include <string.h>

void factoriz(int x, char *result)
{
    int d, q;
    char temp[80];

    sprintf(result, "%d = ", x);

    while (x >= 4 && x % 2 == 0)
    {
        strcat(result, "2 * ");
        x /= 2;
    }
    d = 3;
    q = x / d;
    while (q >= d)
    {
        if (x % d == 0)
        {
            sprintf(temp, "%d * ", d);
            strcat(result, temp);
            x = q;
        }
        else
            d += 2;
        q = x / d;
    }
    sprintf(temp, "%d", x);
    strcat(result, temp);
}
