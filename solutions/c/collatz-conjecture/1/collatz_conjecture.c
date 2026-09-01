#include "collatz_conjecture.h"

int steps(int start) {
  if (start < 1)
    return ERROR_VALUE;
  int step_count = 0;
  int value = start;
  while (value > 1) {
    if (value % 2 == 0) {
      value = value / 2;
    } else {
      value = 3 * value + 1;
    }
    step_count++;
  }

  return step_count;
}
