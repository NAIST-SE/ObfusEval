#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 12)
    return 1;
  int a[10], i, m, c = 0;

  for (i = 1; i < argc - 1; i++)
    a[i - 1] = (int) strtol(argv[i], NULL, 10);
  m = (int) strtol(argv[argc - 1], NULL, 10);

  for (i = 0; i <= argc - 2; i++) {
    if (a[i] == m) {
      c = 1;
      break;
    }
  }
  if (c == 0)
    printf("The number is not in the list\n");
  else
    printf("The number is found\n");

  return 0;
}
