/* C program to check whether a number is prime or not. */
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int n, i, flag = 0;
  n = (int) strtol(argv[1], NULL, 10);

  for (i = 2; i <= n / 2; ++i) {
    if (n % i == 0) {
      flag = 1;
      break;
    }
  }
  if (flag == 0 && n != 1)
    printf("%d is a prime number.\n", n);
  else
    printf("%d is not a prime number.\n", n);
  return 0;
}
