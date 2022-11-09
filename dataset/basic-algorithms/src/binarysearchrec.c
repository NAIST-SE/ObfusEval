#include <stdio.h>
#include <stdlib.h>

int binary(int a[], int n, int m, int l, int u);
int main(int argc, char *argv[]) {
  if (argc < 12)
    return 1;
  int a[10], i, m, c, l, u;

  for (i = 1; i < argc - 1; i++)
    a[i - 1] = (int) strtol(argv[i], NULL, 10);

  m = (int) strtol(argv[argc - 1], NULL, 10);

  l = 0, u = argc - 2;
  c = binary(a, argc - 1, m, l, u);
  if (c == 0)
    printf("Number is not found.\n");
  else
    printf("Number is found.\n");

  return 0;
}

int binary(int a[], int n, int m, int l, int u) {

  int mid, c = 0;

  if (l <= u) {
    mid = l + (u - l) / 2;

    if (m == a[mid]) {
      c = 1;
      return c;
    } else if (m < a[mid]) {
      return binary(a, n, m, l, mid - 1);
    } else
      return binary(a, n, m, mid + 1, u);
  } else
    return c;
}
