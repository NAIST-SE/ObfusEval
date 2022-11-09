#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int n, i, c, a = 1;

  n = (int) strtol(argv[1], NULL, 10);

  for (i = 1; i <= n; i++) {
    for (c = 1; c <= i; c++) {
      printf("%d ", a);
      a++;
    }
    printf("\n");
  }

  return 0;
}
