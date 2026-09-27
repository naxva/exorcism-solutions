#include "square_root.h"

/// Approximates the square root of x using binary search
uint16_t square_root(uint16_t x) {
  uint16_t lower = 1, upper = 256;
  while (1) {
    uint16_t mid = (upper + lower) / 2;
    uint16_t square = mid * mid;

    if (square < x)
      lower = mid;
    else if (square > x)
      upper = mid;
    else
      return mid;
  }
}
