#include <iostream>

std::string catAndMouse(int x, int y, int z) {
  if (std::abs(y - z) > std::abs(x - z)) {
    return "Cat A";
  };

  if (std::abs(y - z) < std::abs(x - z)) {
    return "Cat B";
  };

  return "Mouse C";
}

int main() {
  int x = 1;
  int y = 2;
  int z = 3;
  std::cout << catAndMouse(x, y, z);
}
