#include "sieve.h"
#include <stdbool.h>
#include <stdint.h>
#include <string.h>

uint32_t sieve(uint32_t limit, uint32_t *primes, size_t max_primes)
{
  bool is_prime[limit + 1];
  memset(is_prime, true, limit + 1);
  uint32_t count = 0;
  for (uint32_t i = 2; i < limit + 1 && count < max_primes; i++) {
    if (is_prime[i]) {
      for (uint32_t j = i * i; j < limit + 1; j += i) {
        is_prime[j] = false;
      }
      *(primes + count) = i;
      count++;
    }
  }

  return count;
}
