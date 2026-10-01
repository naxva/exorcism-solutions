#include "prime_factors.h"


size_t find_factors(uint64_t value, uint64_t factors[static MAXFACTORS])
{
  size_t count = 0, div = 2;
  while (value > 1) {
    if (value % div) div +=1;
    else {
      factors[count++] = div;
      value /= div;
    }
    if (count == MAXFACTORS) break;
  }

  return count;
}
