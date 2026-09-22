#include "isogram.h"
#include <ctype.h>
#include <string.h>

bool is_isogram(const char phrase[]) {
  int length, count;
  if (phrase == NULL)
    return false;

  length = strlen(phrase);
  for (int i = 0; i < length; i++) {
    count = 0;
    for (int j = 0; j < length; j++) {
      if (phrase[i] == ' ' || phrase[i] == '-')
        continue;
      if (tolower(phrase[i]) == tolower(phrase[j]))
        count++;
    }
    if (count > 1)
      return false;
  }

  return true;
}
