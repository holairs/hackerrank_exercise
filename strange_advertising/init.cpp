#include <iostream>

int viralAdvertising(int n) {
  int shared = 5;
  int cumulative = 0;
  for (int i = 1; i <= n; i++) {
		int liked = shared / 2;
		cumulative += liked;
		shared = liked * 3;
  }

	return cumulative;
}

int main() {
	int n = 5;
	std::cout << viralAdvertising(n) << std::endl;
}
