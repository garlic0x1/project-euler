package main

import "fmt"

func main() {
	fmt.Println(largestQuad(1000))
}

func largestQuad(limit int) int {
	a := 0
	b := 0
	max := 0
	for i := ((-1 * limit) + 1); i < limit; i++ {
		for j := (-1 * limit); j <= limit; j++ {
			n := quadPrimes(i, j)
			if n > max {
				a = i
				b = j
				max = n
			}
		}
	}
	return a * b
}

func quadPrimes(a int, b int) int {
	loop := true
	c := 0
	for loop {
		if isPrime(quadratic(c, a, b)) {
			c++
		} else {
			loop = false
		}
	}
	loop = true
	d := 0
	for loop {
		if isPrime(quadratic(d, a, b)) {
			d--
		} else {
			loop = false
		}
	}
	if (d * -1) > c {
		return (d * -1)
	}
	return c
}

func quadratic(n int, a int, b int) int {
	sum := n * n
	sum += a * n
	sum += b
	return sum
}

func isPrime(n int) bool {
	if n > 0 {
		for i := 2; i < n/2; i++ {
			if n%i == 0 {
				return false
			}
		}
		return true
	}
	return false
}
