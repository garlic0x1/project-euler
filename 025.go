/*
The Fibonacci sequence is defined by the recurrence relation:

Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
Hence the first 12 terms will be:

F1 = 1
F2 = 1
F3 = 2
F4 = 3
F5 = 5
F6 = 8
F7 = 13
F8 = 21
F9 = 34
F10 = 55
F11 = 89
F12 = 144
The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
*/

package main

import "fmt"

func main() {
	fibonacci()
}

func fibonacci() {
	index := 4
	reg1 := []int{1}
	reg2 := []int{2}
	for true {
		newval := addNums(reg1, reg2)
		if len(newval) >= 1000 {
			fmt.Println(index)
			break
		}
		reg1 = reg2
		reg2 = newval
		index++
	}
}

func addNums(a []int, b []int) []int {
	var result []int
	carry := 0
	if len(a) > len(b) {
		for i := 1; i <= len(a); i++ {
			sum := addPlace(i, carry, a, b)
			result = append(result, (sum % 10))
			carry = sum / 10
		}
	} else {
		for i := 1; i <= len(b); i++ {
			sum := addPlace(i, carry, a, b)
			result = append(result, (sum % 10))
			carry = sum / 10
		}
	}
	if carry != 0 {
		result = append(result, carry)
	}
	return result
}

func addPlace(n int, add int, a []int, b []int) int {
	aval := 0
	if len(a) >= n {
		aval = a[n-1]
	}
	bval := 0
	if len(b) >= n {
		bval = b[n-1]
	}
	sum := add
	sum += aval
	sum += bval
	return sum
}
