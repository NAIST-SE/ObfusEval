#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {

  if (argc < 12)
    return 1;
  int a[10], i, m, c = 0, l, u, mid;

  for (i = 1; i < argc - 1; i++) {
    a[i - 1] = (int) strtol(argv[i], NULL, 10);
  }

  m = (int) strtol(argv[argc - 1], NULL, 0);

  l = 0, u = argc - 2;
  while (l <= u) {
    mid = l + (u - l) / 2;

    if (m == a[mid]) {
      c = 1;
      break;
    } else if (m < a[mid]) {
      u = mid - 1;
    } else
      l = mid + 1;
  }
  if (c == 0)
    printf("The number is not found.\n");
  else
    printf("The number is found.\n");

  return 0;
}
