#include "reverse_string.h"
#include <stdlib.h>
#include <string.h>

char *reverse(const char *value) {
  size_t l = strlen(value);
  char *rs;
  rs = malloc(l + 1);
  for (size_t i = 0; i < l; i++) {
    rs[i] = value[l - 1 - i];
  }
  rs[l] = '\0';

  return rs;
}
