/* C program to find LCM of two positive integers entered by user */
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 3)
    return 1;

  int num1, num2, max;
  num1 = (int) strtol(argv[1], NULL, 10);
  num2 = (int) strtol(argv[2], NULL, 10);

  /* maximum value is stored in variable max */
  max = (num1 > num2) ? num1 : num2;
  while (1) {
    if (max % num1 == 0 && max % num2 == 0) {
      printf("LCM of %d and %d is %d\n", num1, num2, max);
      break; /* while loop terminates. */
    }
    ++max;
  }
  return 0;
}
