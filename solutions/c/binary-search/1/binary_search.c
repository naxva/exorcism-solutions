#include "binary_search.h"

const int *binary_search(int value, const int *arr, size_t length) {
  int low = 0;
  int high = length - 1;
  int mid;
  if (length <= 0 || value < arr[low] || value > arr[high]) return NULL;

  if (arr[low] == value) return (int *)(arr+low);
  if (arr[high] == value) return (int *)(arr+high);


  while (1) {
    if ((low+high) / 2 == mid) break;

    mid = (low+high) / 2;

    if (arr[mid] < value) {
      low = mid;
      continue;
    }

    if (arr[high] < value) {
      high = mid;
      continue;
    }

    if (arr[mid] == value) {
      return (int *)arr+mid;
    }
  }

  return NULL;
}
