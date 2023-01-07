#include <stdio.h>
#include <stdlib.h>

int findgcd(int x, int y);
int main(int argc, char *argv[]) {
  if (argc < 3)
    return 1;
  int n1, n2, gcd;

  n1 = (int) strtol(argv[1], NULL, 10);
  n2 = (int) strtol(argv[2], NULL, 10);
  
  if (n1 <= 0 || n2 <= 0)
    return 1;

  gcd = findgcd(n1, n2);
  printf("GCD of %d and %d is: %d\n", n1, n2, gcd);
  return 0;
}

int findgcd(int x, int y) {
  while (x != y) {
    if (x > y)
      return findgcd(x - y, y);
    else
      return findgcd(x, y - x);
  }
  return x;
}
