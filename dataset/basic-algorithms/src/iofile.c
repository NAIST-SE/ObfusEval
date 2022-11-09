#include <stdio.h>
#include <stdlib.h> /* For exit() function */

int main(int argc, char *argv[]) {
  if (argc < 2)
    return 1;

  char *c = argv[1];
  FILE *rfile, *wfile;
   wfile=fopen("program.txt","w");
  if (wfile == NULL) {
    printf("Error!");
    exit(1);
  }

  fprintf(wfile, "%s", c);
  fclose(wfile);
  remove("program.txt");
  
  return 0;
}
