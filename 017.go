/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?


NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
*/

package main

import (
	"fmt"
	"math"
	"strings"
)

var ones = []string{
	"one",
	"two",
	"three",
	"four",
	"five",
	"six",
	"seven",
	"eight",
	"nine",
	"ten",
	"eleven",
	"twelve",
	"thirteen",
	"fourteen",
	"fifteen",
	"sixteen",
	"seventeen",
	"eighteen",
	"nineteen",
}

var tens = []string{
	"none",
	"twenty",
	"thirty",
	"forty",
	"fifty",
	"sixty",
	"seventy",
	"eighty",
	"ninety",
}

func main() {
	fmt.Println(sumBritish(1, 1000))
}

func sumBritish(nadir int, xenith int) int {
	sum := 0
	for i := nadir; i <= xenith; i++ {
		sum += lengthBritishNum(i)
	}
	return sum
}

func lengthBritishNum(n int) int {
	return len(strings.Join(strings.Fields(britishNum(n)), ""))
}

func britishNum(n int) string {
	result := ""
	arr := itoa(n)
	c := 0
	for i := len(arr) - 1; i >= 0; i-- {
		if c == 0 {
			if arr[i] != 0 {
				// ones place, including teens
				if len(arr) > (c+1) && arr[i-1] == 1 {
					result = ones[9+arr[i]]
				} else {
					result = ones[arr[i]-1]
				}
			} else {
				if len(arr) > (c+1) && arr[i-1] == 1 {
					result = ones[9+arr[i]]
				}
			}
		} else if c == 1 && arr[i] > 1 {
			// tens place, excluding teens
			result = tens[arr[i]-1] + " " + result
		} else if c == 2 && arr[i] > 0 {
			// hundreds place
			if result != "" {
				result = ones[arr[i]-1] + " hundred and " + result
			} else {
				result = ones[arr[i]-1] + " hundred" + result
			}
		} else if c == 3 && arr[i] != 0 {
			result = ones[arr[i]-1] + " thousand " + result
		}
		c++
	}
	fmt.Println(result)
	return result
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
