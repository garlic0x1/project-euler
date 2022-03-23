/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

package main

import "fmt"

func main() {
	findTriple()
}

func findTriple() {
	// a must be 0-333
	// b must be 9-500
	// c must be 333-1000
	for a := 0; a < 333; a++ {
		for b := 0; b < 500; b++ {
			for c := 333; c < 1000; c++ {
				if isPythagoreanTriple(a, b, c) && a+b+c == 1000 {
					fmt.Println(a * b * c)
					break
				}
			}
		}
	}
}

func isPythagoreanTriple(a int, b int, c int) bool {
	if (a*a)+(b*b) == (c * c) {
		return true
	}
	return false
}
