#include "eliuds_eggs.h"

unsigned int egg_count(unsigned int bits) {
  unsigned int result = 0;
  while (bits) {
    result += 1;
    bits &= bits - 1;
  }

  return result;
}
