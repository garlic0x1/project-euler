/*
A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:

1/2	= 	0.5
1/3	= 	0.(3)
1/4	= 	0.25
1/5	= 	0.2
1/6	= 	0.1(6)
1/7	= 	0.(142857)
1/8	= 	0.125
1/9	= 	0.(1)
1/10	= 	0.1
Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
*/

package main

import "fmt"

type Place struct {
	Quotient  int
	Remainder int
}

func main() {
	fmt.Println(longestCycle(999, 1000))
}

func longestCycle(n int, p int) (int, int) {
	longest := 0
	index := 0
	for i := n; i > 0; i-- {
		reg := cycleLength(divide(i, p, 1))
		if reg > longest {
			longest = reg
			index = i
		}
	}
	return index, longest
}

func cycleLength(arr []Place) int {
	m := make(map[Place]int)
	for i, place := range arr {
		value, exists := m[place]
		if exists {
			return i - value
		}
		m[place] = i
	}
	return 0
}

func divide(n int, depth int, rem int) []Place {
	quotient := 10 * rem / n
	remainder := 10 * rem % n
	if depth > 0 {
		return append([]Place{Place{Quotient: quotient, Remainder: remainder}}, divide(n, depth-1, remainder)...)
	}
	return []Place{Place{Quotient: quotient, Remainder: remainder}}
}
