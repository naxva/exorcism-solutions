#include "beer_song.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFSIZE 80

#define BOTTLE_VERSE "%s %s of beer on the wall, %s %s of beer."
#define TAKE_DOWN_VERSE                                                        \
  "Take %s down and pass it around, %s %s of beer on the wall."
#define END_VERSE                                                              \
  "Go to the store and buy some more, 99 bottles of beer on the wall."

static char *itoa(int i, char *s, int radix) {
  if (radix == 10)
    sprintf(s, "%d", i);
  else if (radix == 8)
    sprintf(s, "%o", i);
  else if (radix == 16)
    sprintf(s, "%x", i);
  else
    return NULL;
  return s;
}

void recite(uint8_t start_bottles, uint8_t take_down, char **song) {
  for (uint8_t i = 0; i < take_down; ++i) {
    uint8_t num = start_bottles - i;
    char bottle_num[10] = {0};
    char bottle_noun[10] = {0};
    if (num)
      itoa(num, bottle_num, 10);
    else
      strcpy(bottle_num, "No more");
    if (num == 1)
      strcpy(bottle_noun, "bottle");
    else
      strcpy(bottle_noun, "bottles");

    song[3 * i + 0] = calloc(BUFFSIZE, sizeof(char));
    sprintf(song[3 * i + 0], BOTTLE_VERSE, bottle_num, bottle_noun,
            bottle_num[0] == 'N' ? "no more" : bottle_num, bottle_noun);

    song[3 * i + 1] = calloc(BUFFSIZE, sizeof(char));
    if (start_bottles - i > 0) {
      char bottle_num[10] = {0};
      char bottle_noun[10] = {0};
      char bottle_proun[5] = {0};
      if (num - 1)
        itoa(num - 1, bottle_num, 10);
      else
        strcpy(bottle_num, "no more");
      if (num - 1 == 1)
        strcpy(bottle_noun, "bottle");
      else
        strcpy(bottle_noun, "bottles");
      if (num == 1)
        strcpy(bottle_proun, "it");
      else
        strcpy(bottle_proun, "one");

      sprintf(song[3 * i + 1], TAKE_DOWN_VERSE, bottle_proun, bottle_num,
              bottle_noun);
    } else {
      sprintf(song[3 * i + 1], END_VERSE);
      break;
    }

    if (i != take_down - 1) {
      song[3 * i + 2] = calloc(BUFFSIZE, sizeof(char));
      strcpy(song[3 * i + 2], "");
    }
  }
}
