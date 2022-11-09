#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  char *line = argv[1];
  int i, v, c, ch, d, s;
  v = c = ch = d = s = 0;

  for (i = 0; line[i] != '\0'; ++i) {
    if (line[i] == 'a' || line[i] == 'e' || line[i] == 'i' || line[i] == 'o' ||
        line[i] == 'u' || line[i] == 'A' || line[i] == 'E' || line[i] == 'I' ||
        line[i] == 'O' || line[i] == 'U')
      ++v;
    else if ((line[i] >= 'a' && line[i] <= 'z') ||
             (line[i] >= 'A' && line[i] <= 'Z'))
      ++c;
    else if (line[i] >= '0' && c <= '9')
      ++d;
    else if (line[i] == ' ')
      ++s;
  }
  printf("Vowels: %d", v);
  printf("\nConsonants: %d", c);
  printf("\nDigits: %d", d);
  printf("\nWhite spaces: %d", s);
  return 0;
}
