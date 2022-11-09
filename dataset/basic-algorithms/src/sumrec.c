#include <stdio.h>
#include <stdlib.h>

int findsum(int n);
int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int num, x;

  num = (int) strtol(argv[1], NULL, 10);

  x = findsum(num);
  printf("Sum of the digits of %d is: %02x\n", num, x);
  return 0;
}

int r, s = 1;
int findsum(int n) {
  if (n) {
    r = n % 10;
    s = s + r;
    return findsum(n / 10);
  } else
    return s;
}
