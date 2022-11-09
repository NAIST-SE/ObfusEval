#include <stdio.h>
#include <stdlib.h>

int fib(int n) {
  int a = 1;
  int b = 1;
  int i;
  for (i = 3; i <= n; i++) {
    int c = a + b;
    a = b;
    b = c;
  };
  return b;
}

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;

  int n = (int) strtol(argv[1], NULL, 10);
  int f = fib(n);
  printf("fib(%i)=%i\n", n, f);
}
