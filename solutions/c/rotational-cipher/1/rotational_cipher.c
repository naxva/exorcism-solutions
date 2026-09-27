#include "rotational_cipher.h"
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

char *rotate(const char *text, int shift_key)
{
  size_t len = strlen(text);
  char *result = calloc(len + 1, sizeof(char));
  len = 0;
  for (const char *p = text; *p; ++p) {
    char c = *p;
    if (('A' <= c) && (c <= 'Z')) {
      c -= 'A';
      c = (c + shift_key) % 26;
      c += 'A';
    } else if (('a' <= c) && (c <= 'z')) {
      c -= 'a';
      c = (c + shift_key) % 26;
      c += 'a';
    }
    result[len++] = c;
  }

  return result;
}
