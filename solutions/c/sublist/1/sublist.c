#include "sublist.h"

#include <stdbool.h>

static bool is_contained(const int *inner, size_t inner_len, const int *outer,
                         size_t outer_len) {

  if (inner_len == 0) {
    return true;
  }

  if (inner_len > outer_len) {
    return false;
  }

  if (!inner || !outer) {
    return false;
  }

  for (size_t i = 0; i <= outer_len - inner_len; i++) {
    bool match = true;
    for (size_t j = 0; j < inner_len; j++) {
      if (outer[i + j] != inner[j]) {
        match = false;
        break;
      }
    }
    if (match) {
      return true;
    }
  }
  return false;
}

comparison_result_t check_lists(const int *list_to_compare,
                                const int *base_list,
                                size_t list_to_compare_element_count,
                                size_t base_list_element_count) {

  bool is_sub = is_contained(list_to_compare, list_to_compare_element_count,
                             base_list, base_list_element_count);

  bool is_super = is_contained(base_list, base_list_element_count,
                               list_to_compare, list_to_compare_element_count);

  if (is_sub && is_super) {
    return EQUAL;
  } else if (is_sub) {
    return SUBLIST;
  } else if (is_super) {
    return SUPERLIST;
  }
  return UNEQUAL;
}
