#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 11)
    return 1;

  int a[10], i, big, small;

  for (i = 1; i < argc; i++)
    a[i - 1] = (int) strtol(argv[i], NULL, 10);

  big = a[0];
  for (i = 1; i < argc - 1; i++) {
    if (big < a[i])
      big = a[i];
  }
  printf("Largest element: %d\n", big);

  small = a[0];
  for (i = 1; i < argc - 1; i++) {
    if (small > a[i])
      small = a[i];
  }
  printf("Smallest element: %d\n", small);

  return 0;
}
