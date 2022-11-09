#include <stdio.h>

int stringCompare(char[], char[]);
int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  int compare;

  char *str1 = argv[1];
  char *str2 = argv[2];

  compare = stringCompare(str1, str2);

  if (compare == 1)
    printf("Both strings are equal.");
  else
    printf("Both strings are not equal");

  return 0;
}

int stringCompare(char str1[], char str2[]) {
  int i = 0, flag = 0;

  while (str1[i] != '\0' && str2[i] != '\0') {
    if (str1[i] != str2[i]) {
      flag = 1;
      break;
    }
    i++;
  }

  if (flag == 0 && str1[i] == '\0' && str2[i] == '\0')
    return 1;
  else
    return 0;
}
