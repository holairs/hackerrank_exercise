#include <iostream>

int jumpingOnClouds(std::vector<int> c, int k) {
  int n = c.size();
  int energy = 100;
  int position = 0;
  bool running = true;

  do {
    if (c[position] == 1) {
      energy -= 3;
    } else {
      energy -= 1;
    };

    position = (position + k) % n;

    if (position == 0) {
      running = false;
    }

  } while (running);

  return energy;
}

int main() {
  std::vector<int> c = {0, 0, 1, 0};
  int k = 2;
  std::cout << jumpingOnClouds(c, k) << std::endl;
}
