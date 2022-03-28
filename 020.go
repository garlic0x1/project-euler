/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/
package main

import "fmt"

func main() {
	fmt.Println(sumFactorial(100))
}

func sumArr(arr []int) int {
	sum := 0
	for _, i := range arr {
		sum += i
	}
	return sum
}

func sumFactorial(n int) int {
	factorial := factorialArr(n)
	res := []int{1}
	for _, factor := range factorial {
		res = productArr(res, factor)
	}
	return sumArr(res)
}

func productArr(arr []int, factor int) []int {
	// multiply the array
	carry := 0
	for j := 0; j < len(arr); j++ {
		carry = multiplyPlace(arr, j, carry, factor)
	}
	if carry != 0 {
		if carry/10 < 1 {
			arr = append(arr, carry)
		} else {
			arr = append(arr, carry%10)
			arr = append(arr, carry/10)
		}
	}
	return arr
}

func factorialArr(n int) []int {
	var ret []int
	for i := 1; i <= n; i++ {
		ret = append(ret, i)
	}
	return ret
}

func multiplyPlace(arr []int, n int, carry int, factor int) int {
	ret := ((arr[n] * factor) + carry) / 10
	arr[n] = ((arr[n] * factor) + carry) % 10
	return ret
}
