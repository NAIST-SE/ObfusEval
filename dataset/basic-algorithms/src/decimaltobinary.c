#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;
  long int decimalNumber, quotient;

  int binaryNumber[100], i = 0, j;

  decimalNumber = strtol(argv[1], NULL, 10);
  if (decimalNumber < 0)
    return 1;

  quotient = decimalNumber;

  while(quotient!=0){
    binaryNumber[i++] = quotient % 2;
    quotient = quotient / 2;
  }

  printf("Equivalent binary value of decimal number %ld: \n", decimalNumber);
  for(j = i - 1; j > 0; j--)
    printf("%d", binaryNumber[j]);

  return 0;
}
