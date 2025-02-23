#include <iostream>

int extraLongFactorials(int n) {
  int result = n;

  for (int i = 1; i <= n; i++) {
    result = result * (n - i);
  }

  std::cout << result << std::endl;
  return result;
}

int main() {
  // let n = 25;
  //  15511210043330985984000000
  int n = 45;
  // 119622220865480194561963161495657715064383733760000000000
  extraLongFactorials(n);
}
