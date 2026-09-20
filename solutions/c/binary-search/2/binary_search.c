#include "binary_search.h"

int *binary_search(int value, int *arr, size_t length) {
  if (arr && length) {
    int mid = length / 2;
    int *address = &arr[mid];
    int val = *address;

    if (value == val) {
      return address;
    } else if (value > val) {
      return binary_search(value, address + 1, length - (mid +1));
    } else {
      return binary_search(value, arr, mid);
    }
  }

  return NULL;
}
