#include "grains.h"
#include <math.h>
#include <limits.h>

// Calculate the number of grains of wheat on a chessboard.
//
// A chessboard has 64 squares.
// Square 1 has one grain, square 2 has two grains, square 3 has four grains, and so on, doubling each time.
//
// Write code that calculates:
//
// - the number of grains on a given square
// - the total number of grains on the chessboard

uint64_t square(uint8_t index) {
    if (index < 1 || index > 64) {
        return 0;
    }
    return pow(2, index - 1);
}
uint64_t total(void) {
    return ULLONG_MAX;
}
