#include <stdio.h>
#include <stdlib.h>

int penta(int n) {
  return (n * n * 3 - n) / 2;
}

int binary_search(int n, int* buf, int length) {
  int low = 0;
  int high = length - 1;
  int mid;

  while (low <= high) {
    mid = (low + high) / 2;
    
    if (buf[mid] > n)
      high = mid - 1;
    else if (buf[mid] < n)
      low = mid + 1;
    else
      return mid;
  }
  
  return -1;
}

int main() {
  // generate pentagons
  static int LEN = 10000;
  int* buf = malloc(LEN * 4);
  for (int i = 0; i < LEN; i++)
    buf[i] = penta(i);
  
  for (int diff = 1; diff < LEN; diff++) {
    for (int a = 1; a < LEN; a++) {
      int b = binary_search((buf[diff] + buf[a]), buf, LEN);
      if (b > 0 && binary_search((buf[a] + buf[b]), buf, LEN) > 0) {
        printf("%d", buf[diff]);
        return 0;
      }
    }
  }
  return 1;
}