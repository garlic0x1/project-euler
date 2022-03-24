/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/

package main

import "fmt"

func main() {
	//fmt.Println(chainLen(13), generateChain(13))
	fmt.Println(longestChain(1000000))
}

func longestChain(limit int) int {
	maxval := 0
	maxindex := 0
	for i := 1; i < limit; i++ {
		length := chainLen(i)
		if length > maxval {
			maxval = length
			maxindex = i
		}
	}

	return maxindex
}

func chainLen(n int) int {
	c := 1
	reg := n
	for reg != 1 {
		c++
		if (reg % 2) == 0 {
			reg = reg / 2
		} else {
			reg = (reg * 3) + 1
		}
	}
	return c

}

func generateChain(n int) []int {
	var ret []int
	reg := n
	for reg != 1 {
		ret = append(ret, reg)
		if (reg % 2) == 0 {
			reg = reg / 2
		} else {
			reg = (reg * 3) + 1
		}
	}
	ret = append(ret, 1)
	return ret
}
