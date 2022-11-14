#include <stdio.h>

int smallest_prime(int n)
{
  if (!(n % 2)) return 2;
  if (!(n % 3)) return 3;
  
  int i = 5;
  int step = 2;
  
  while (i * i <= n)
  {
    if (!(n % i)) return i;
    
    i += step;
    step = 6 - step;
  }
  
  return n;
}

int contains(int n, int* buf, int length)
{
  for (int i = 0; i < length; i++)
    if (n == buf[i])
      return 1;
  return 0;
}

int distinct_primes(int n, int length)
{
  int fact_buf[length];
  int count = 0;
  
  while (count < length)
  {
    int fact = smallest_prime(n);
    int add_new = 1;
    
    if (!contains(fact, fact_buf, count))
    {
      fact_buf[count] = fact;
      count++;
    }

    if (fact == n) return (count == length);

    n /= fact;
  }

  return 0;
}

int main() {
  static int LEN = 4;
  int c = 0;
  int i = 2;
  
  while (c < LEN) {
    if (distinct_primes(i, LEN))
      c++;
    else 
      c = 0;
    i++;
  }

  printf("%d", i-4);
}
