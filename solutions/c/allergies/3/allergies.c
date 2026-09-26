#include "allergies.h"

bool is_allergic_to(allergen_t allergen, int num) {
  return num & (1 << allergen);
}

allergen_list_t get_allergens(int num) {
  allergen_list_t list;
  list.count = 0;
  for (int i = 0; i < ALLERGEN_COUNT; i++) {

    list.allergens[i] = is_allergic_to(i, num);
    if (list.allergens[i]) list.count++;
  }

  return list;
}
