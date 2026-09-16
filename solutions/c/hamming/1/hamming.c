#include "hamming.h"
#include <string.h>

int compute(const char *lhs, const char *rhs) {
  if (strlen(lhs) != strlen(rhs)) { return -1; }
  int count = 0;
  while (*lhs != 0) {
    if ((char) *lhs++ != (char) *rhs++) {
      count++;
    }
  }
  return count;
}
