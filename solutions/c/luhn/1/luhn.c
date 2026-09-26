#include "luhn.h"

bool luhn(const char *num) {
  if (!num) return false;
  int digits = 0, sum_even = 0, sum_odd = 0;
  static const int LUHN_TABLE[10] = { 0, 2, 4, 6, 8, 1, 3, 5, 7, 9};

  for (int i = 0; num[i] != '\0'; i++) {
    char c = num[i];
    if (c == ' ') continue;
    if (c < '0' || c > '9') return false;

    int digit = c - '0', doubled = LUHN_TABLE[digit];

    if ((digits & 1) != 0) {
      sum_even += digit;
      sum_odd += doubled;
    } else {
      sum_even += doubled;
      sum_odd += digit;
    }

    digits++;
  }

  return (digits > 1) && ((digits & 1) == 0 ? sum_even : sum_odd) % 10 == 0;
}
