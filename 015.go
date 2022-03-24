/*
Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

How many such routes are there through a 20×20 grid?

(2n)!/(n!)^2
*/

package main

import "fmt"

func main() {
	fmt.Println(latticeRoutes(20))
}

func latticeRoutes(n int) int {
	dividend := factorialTop(n)
	divisor := factorialArr(n)
	fmt.Println(dividend, " / ", divisor)

	// need to simplify fraction in order to
	// fit numbers
	simplify(dividend, divisor)
	fmt.Println(dividend, " / ", divisor)
	product1 := 1
	for _, i := range dividend {
		product1 *= i
	}
	product2 := 1
	for _, i := range divisor {
		product2 *= i
	}
	return product1 / product2
}

func simplify(dividend []int, divisor []int) {
	for i, _ := range dividend {
		for j, _ := range divisor {
			if (divisor[j] != 1) && (dividend[i]%divisor[j]) == 0 {
				fmt.Println("dividing:", dividend[i], divisor[j])
				dividend[i] = dividend[i] / divisor[j]
				divisor[j] = 1
			}
		}
	}
}

func factorialTop(n int) []int {
	var ret []int
	for i := (n + 1); i <= (n * 2); i++ {
		ret = append(ret, i)
	}
	return ret
}
func factorialArr(n int) []int {
	var ret []int
	for i := 1; i <= n; i++ {
		ret = append(ret, i)
	}
	return ret
}

/*
func factorial(n int) int {
	product := 1
	for i := 1; i <= n; i++ {
		product *= i
	}
	return product
}
*/
