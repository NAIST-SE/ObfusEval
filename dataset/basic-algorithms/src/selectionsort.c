#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 11)
    return 1;
  int i, j, temp, a[10];

  for (i = 1; i < argc; i++)
    a[i - 1] = (int) strtol(argv[i], NULL, 10);

  for (i = 0; i < argc - 1; i++) {
    for (j = i + 1; j < argc - 1; j++) {
      if (a[i] > a[j]) {
        temp = a[i];
        a[i] = a[j];
        a[j] = temp;
      }
    }
  }

  printf("After sorting is: ");
  for (i = 0; i < argc - 1; i++)
    printf(" %d", a[i]);

  printf("\n");
  return 0;
}
