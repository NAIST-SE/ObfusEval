#include <stdio.h>
#include <stdlib.h>

int fact(int);
int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int num, f;

  num = (int) strtol(argv[1], NULL, 10);
  f = fact(num);
  printf("\nFactorial of %d is: %d\n", num, f);
  return 0;
}

int fact(int n) {
  if (n == 1)
    return 1;
  else
    return (n * fact(n - 1));
}
