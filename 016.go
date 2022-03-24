/*
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
*/

package main

import "fmt"

func main() {
	fmt.Println(powerTwoSum(1000))
}

func powerTwoSum(power int) int {
	arr := []int{1}
	for i := 0; i < power; i++ {

		// double the array
		carry := 0
		for n := 0; n < len(arr); n++ {
			carry = doublePlace(arr, n, carry)
		}
		// add more digits if needed
		if carry != 0 {
			arr = append(arr, carry)
		}
	}

	// sum up the result
	sum := 0
	for _, digit := range arr {
		sum += digit
	}
	return sum
}

func doublePlace(arr []int, n int, carry int) int {
	ret := (arr[n] * 2) / 10
	arr[n] = ((arr[n] * 2) + carry) % 10
	return ret
}
