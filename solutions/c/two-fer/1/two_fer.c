#include "two_fer.h"
#include <stdio.h>


void two_fer(char *buffer, const char *name) {
  const char *true_name = name ? name : "you";
  snprintf(buffer,
           sizeof("One for  , one for me.") + sizeof(true_name),
           "One for %s, one for me.",true_name);
}
