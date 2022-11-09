#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 3)
    return 1;
  int i, count = 0;

  char *c = argv[1];
  char ch = argv[2][0];

  for (i = 0; c[i] != '\0'; ++i) {
    if (ch == c[i])
      ++count;
  }
  printf("Frequency of %c = %d", ch, count);
  return 0;
}