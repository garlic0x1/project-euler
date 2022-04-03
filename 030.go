/*
Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

1634 = 1^4 + 6^4 + 3^4 + 4^4
8208 = 8^4 + 2^4 + 0^4 + 8^4
9474 = 9^4 + 4^4 + 7^4 + 4^4
As 1 = 1^4 is not a sum it is not included.

The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
*/

package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(test())
}

func test() int {
	sum := 0
	for i := 2; i < 1000000; i++ {
		if valid(i) {
			sum += i
		}
	}
	return sum
}

func valid(n int) bool {
	arr := itoa(n)
	sum := 0
	for _, v := range arr {
		sum += (v * v * v * v * v)
	}
	return sum == n
}

func itoa(n int) []int {
	var arr []int
	max := int(math.Floor(math.Log10(float64(n)))) + 1
	for i := max - 1; i >= 0; i-- {
		place1 := int(math.Pow(10, float64(i)))
		digit1 := (n % (place1 * 10)) / place1
		arr = append(arr, digit1)
	}
	return arr
}
