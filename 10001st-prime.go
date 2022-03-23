/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

package main

import "fmt"

func main() {
	sieve(10000000)
}

/*
func nthPrime(n int) int {

}
*/

func sieve(num int) {
	prime := make([]bool, num+1)

	for i := 0; i < num+1; i++ {
		prime[i] = false
	}

	for i := 2; i*i <= num; i++ {
		if prime[i] == false {
			for j := i * 2; j <= num; j += i {
				prime[j] = true // cross
			}
		}

	}

	c := 0
	for i := 2; i < len(prime); i++ {
		if prime[i] == false {
			//fmt.Println(i)
			c++
			if c == 10001 {
				fmt.Println(i)
			}
		}
	}
	fmt.Println("Number of primes under", num, c)
}
