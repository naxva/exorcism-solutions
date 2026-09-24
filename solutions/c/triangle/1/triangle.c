#include "triangle.h"


bool is_valid(triangle_t triangle) {
  if (triangle.a == 0 || triangle.b == 0 || triangle.c == 0 )
    return false;
  if (triangle.a + triangle.b < triangle.c) return false;
  if (triangle.a + triangle.c < triangle.b) return false;
  if (triangle.b + triangle.c < triangle.a) return false;

  return true;

}

bool is_equilateral(triangle_t triangle) {
  return is_valid(triangle) &&
        (triangle.a == triangle.b &&
         triangle.b == triangle.c);
}
bool is_isosceles(triangle_t triangle) {
  return is_valid(triangle) &&
        (triangle.a == triangle.b ||
         triangle.a == triangle.c ||
         triangle.b == triangle.c);
}
bool is_scalene(triangle_t triangle) {
  return is_valid(triangle) &&
        (triangle.a != triangle.b &&
         triangle.a != triangle.c &&
         triangle.b != triangle.c);
}
