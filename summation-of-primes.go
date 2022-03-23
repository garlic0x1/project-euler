/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

package main

import "fmt"

func main() {
	sieve(2000000)
}

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

	sum := 0
	for i := 2; i < len(prime); i++ {
		if prime[i] == false {
			sum += i
		}
	}
	fmt.Println("Sum of primes below 2 million", sum)
}
