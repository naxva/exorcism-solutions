#include "pangram.h"
#include <stdlib.h>

static int get_char_code(char c) {
  if (c >= 'a' && c <= 'z') return c - 'a';
  else return c - 'A';
}

static const int ALPHABET_SIZE = 26;

bool is_pangram(const char *sentence) {
  if (sentence == NULL) return false;

  int mask = 0;

  for (char *c = (char *)sentence; *c; c++) {
    int code = get_char_code(*c);
    if (code >= 0 && code < ALPHABET_SIZE) mask |= 1 << code;
  }

  return (mask + 1) == (1 << ALPHABET_SIZE);
}
