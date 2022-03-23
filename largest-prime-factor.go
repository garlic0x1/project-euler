/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

package main

import "fmt"

func main() {
	fmt.Println("Largest prime of 600851475143:", largestPrime(600851475143))
}

func largestPrime(n int) int {
	for i := 2; i <= (n / 2); i++ {
		if (n % i) == 0 {
			return largestPrime(n / i)
		}
	}
	return n
}
