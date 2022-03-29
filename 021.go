/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
*/

package main

import "fmt"

func main() {
	fmt.Println(sumAmicables(10000))
}

func sumAmicables(n int) int {
	return sumArr(amicables(n))
}

func amicables(n int) []int {
	var ret []int
	for a := 1; a < n; a++ {
		b := d(a)
		if a != b && d(b) == a {
			ret = append(ret, a)
		}
	}
	return ret
}

func d(n int) int {
	return sumArr(properDivisors(n))
}

func sumArr(arr []int) int {
	sum := 0
	for _, i := range arr {
		sum += i
	}
	return sum
}

func properDivisors(n int) []int {
	var ret []int
	limit := n
	for i := 1; i < limit; i++ {
		if (n % i) == 0 {
			if i == 1 {
				ret = append(ret, i)
			} else {
				ret = append(ret, i, n/i)
			}
			limit = n / i
		}
	}
	return ret
}
