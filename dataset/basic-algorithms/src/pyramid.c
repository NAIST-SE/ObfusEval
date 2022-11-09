#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
    
  int i, j, rows;
  rows = (int) strtol(argv[1], NULL, 10);

  for (i = 1; i <= rows; ++i) {
    for (j = 1; j <= i; ++j) {
      printf("* ");
    }
    printf("\n");
  }
  return 0;
}
