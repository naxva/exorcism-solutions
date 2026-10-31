#include "kindergarten_garden.h"
#include <string.h>
#include <stdlib.h>

plants_t plants(const char *diagram, const char *student) {
  plants_t result;
  int student_index = student[0] - 'A';
  int start_index_row_one = student_index * 2;
  int start_index_row_two = (int)(strlen(diagram)-1) / 2 + 1 + start_index_row_one;
  char headers[4];
  headers[0] = diagram[start_index_row_one++];
  headers[1] = diagram[start_index_row_one];
  headers[2] = diagram[start_index_row_two++];
  headers[3] = diagram[start_index_row_two];
  for (int i = 0; i < 4; i++) {
    switch(headers[i]) {
      case 'V':
        result.plants[i] = VIOLETS;
        break;
      case 'R':
        result.plants[i] = RADISHES;
        break;
      case 'G':
        result.plants[i] = GRASS;
        break;
      case 'C':
        result.plants[i] = CLOVER;
        break;
      default:
        exit(0);
    }
  }

  return result;
}
