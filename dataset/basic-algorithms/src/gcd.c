/* C Program to Find Highest Common Factor. */
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 3)
    return 1;
  int num1, num2, i, hcf;
  num1 = (int) strtol(argv[1], NULL, 10);
  num2 = (int) strtol(argv[2], NULL, 10);

  for (i = 1; i <= num1 || i <= num2; ++i) {
    if (num1 % i == 0 &&
        num2 % i == 0) /* Checking whether i is a factor of both number */
      hcf = i;
  }
  printf("H.C.F of %d and %d is %d\n", num1, num2, hcf);
  return 0;
}
