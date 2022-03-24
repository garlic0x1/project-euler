/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

package main

import (
	"fmt"
	"math"
)

func main() {
	for i := 998001; i > 0; i-- {
		if isPalindromic(i) && factors3Digit(i) {
			fmt.Println(i)
			return
		}
	}
}

func factors3Digit(n int) bool {
	for i := 999; i > 100; i-- {
		if n%i == 0 && n/i > 100 && n/i < 999 {
			return true
		}
	}
	return false
}

func isPalindromic(n int) bool {
	max := int(math.Floor(math.Log10(float64(n)))) + 1
	for i := 0; i < max/2; i++ {
		place1 := int(math.Pow(10, float64(i)))
		digit1 := (n % (place1 * 10)) / place1
		place2 := int(math.Pow(10, float64(max-(i+1))))
		digit2 := (n % (place2 * 10)) / place2

		if digit1 != digit2 {
			return false
		}
	}
	return true
}
