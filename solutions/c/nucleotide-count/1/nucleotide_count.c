#include "nucleotide_count.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *count(const char *dna_strand) {
  int a = 0, c = 0, g = 0, t = 0;
  for (; *dna_strand != '\0'; dna_strand++) {
   switch (*dna_strand) {
     case 'A':
       a++;
       break;
     case 'C':
       c++;
       break;
     case 'G':
       g++;
       break;
     case 'T':
       t++;
       break;
     default:
       return calloc(1, sizeof(char));
   }
  }

  size_t length = snprintf(NULL, 0, "A:%d C:%d G:%d T:%d", a, c, g, t);
  char *result = malloc(length + 1);
  sprintf(result, "A:%d C:%d G:%d T:%d", a, c, g, t);
  return result;
}
