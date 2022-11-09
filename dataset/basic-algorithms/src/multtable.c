/* C program to find multiplication table up to 10. */
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int n, i;
  n = (int) strtol(argv[1], NULL, 10);

  for (i = 1; i <= 10; ++i) {
    printf("%d * %d = %d\n", n, i, n * i);
  }
  return 0;
}
