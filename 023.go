/*
A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.

Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
*/

package main

import "fmt"

func main() {
	fmt.Println(sumSieve(sieve()))
}

func sumSieve(arr []bool) int {
	sum := 0
	for i, b := range arr {
		if b == false {
			sum += i
		}
	}
	return sum
}

func sieve() []bool {
	max := 28124
	sumAbundants := make([]bool, max)
	for i := 0; i < max; i++ {
		sumAbundants[i] = false
	}

	abundants := getAbundants()
	for _, a := range abundants {
		for _, b := range abundants {
			if a+b >= max {
				break
			}
			sumAbundants[a+b] = true
		}
	}
	return sumAbundants
}

func getAbundants() []int {
	var ret []int
	for i := 12; i < 28123; i++ {
		if isAbundant(i) {
			ret = append(ret, i)
		}
	}
	return ret
}

func isAbundant(n int) bool {
	sum := 0
	for _, divisor := range properDivisors(n) {
		sum += divisor
	}
	return sum > n
}

func properDivisors(n int) []int {
	var ret []int
	limit := n
	for i := 1; i < limit; i++ {
		if (n % i) == 0 {
			if i == 1 || i == n/i {
				ret = append(ret, i)
			} else {
				ret = append(ret, i, n/i)
			}
			limit = n / i
		}
	}
	return ret
}
