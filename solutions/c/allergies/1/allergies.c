#include "allergies.h"

bool is_allergic_to(allergen_t allergen, int num) {
  return (num & (1 << allergen)) != 0;
 }

allergen_list_t get_allergens(int num) {
  allergen_list_t allergen_list;
  allergen_list.count = 0;
  for (int i = 0; i < ALLERGEN_COUNT; i++) {
    if (is_allergic_to(i, num)) {
      allergen_list.count++;
      allergen_list.allergens[i] = true;
    }
    else {
      allergen_list.allergens[i] = false;
    }
  }

  return allergen_list;
}
