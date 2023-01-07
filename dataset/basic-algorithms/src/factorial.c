/* C program to display factorial of an integer if user enters non-negative
 * integer. */

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int n, count;
  unsigned long long int factorial = 1;

  n = (int) strtol(argv[1], NULL, 10);
  if (n < 0) {
    printf("Error!!! Factorial of negative number doesn't exist.\n");
    return 1;
  }
  
  for (count = 1; count <= n; ++count) /* for loop terminates if count>n */
    factorial *= count; /* factorial=factorial*count */

  printf("Factorial = %llu\n", factorial);
  return 0;
}
