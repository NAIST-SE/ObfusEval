#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 11)
    return 1;
  if (argc > 11)
    return 1;
  int i, j, s = 10, temp, a[10];

  for (i = 1; i < argc; i++)
    a[i - 1] = (int) strtol(argv[i], NULL, 10);

  for (i = 1; i < s; i++) {
    temp = a[i];
    j = i - 1;
    while ((temp < a[j]) && (j >= 0)) {
      a[j + 1] = a[j];
      j = j - 1;
    }
    a[j + 1] = temp;
  }

  printf("After sorting: ");
  for (i = 0; i < s; i++)
    printf(" %d", a[i]);

  return 0;
}
