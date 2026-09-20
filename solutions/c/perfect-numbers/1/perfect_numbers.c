#include "perfect_numbers.h"


kind classify_number(int num) {
  if (num <= 0) return -1;
  int aliquot_sum = 0;
  for (int i = 1; i < num; i++) {
    if (num % i == 0) aliquot_sum += i;
  }

  if (aliquot_sum == num) return 1;
  if (aliquot_sum  > num) return 2;
  if (aliquot_sum  < num) return 3;
  return -1;

}
