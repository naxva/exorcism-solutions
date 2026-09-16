#include "binary.h"
#include <math.h>
#include <string.h>

int convert_digit(char);

int convert_digit(char digit) {
  switch(digit) {
    case '1':
      return 1;
    case '0':
      return 0;
    default:
      return INVALID;
  }
}

int convert(const char *input) {
  int result = 0;
  int max_power = strlen(input) - 1;
  while (*input != 0) {
    int digit = convert_digit(*input++);
    if (digit == INVALID) return INVALID;
    result += (digit * (pow(2, max_power--)));
  }
  return result;
}
