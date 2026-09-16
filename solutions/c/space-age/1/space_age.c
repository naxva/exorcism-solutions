#include "space_age.h"
#define EARTH_SECONDS 31557600

float orbital_periods[] = { 0.2408467f, 0.61519726f, 1.0f, 1.8808158f, 11.862615f, 29.447498f, 84.016846f, 164.79132f };
float age(planet_t planet, int64_t seconds) {
  if ((int) planet < 0) return -1.0f;
  return seconds / (orbital_periods[planet] * EARTH_SECONDS);
}
