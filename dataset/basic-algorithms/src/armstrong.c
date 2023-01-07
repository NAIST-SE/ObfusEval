#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int n, n1, rem, num = 0;
  n = (int) strtol(argv[1], NULL, 10);

  n1 = n;
  while (n1 != 0) {
    rem = n1 % 10;
    num += rem * rem * rem;
    n1 /= 10;
  }
  if (num == n)
    printf("%d is an Armstrong number.", n);
  else
    printf("%d is not an Armstrong number.", n);
}
