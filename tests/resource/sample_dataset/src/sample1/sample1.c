void bubblesort(int n, int a[])
{
    int i, j, k;
    int x;

    k = n - 1;
    while (k >= 0)
    {
        j = -1;
        for (i = 1; i <= k; i++)
            if (a[i - 1] > a[i])
            {
                j = i - 1;
                x = a[j];
                a[j] = a[i];
                a[i] = x;
            }
        k = j;
    }
}
