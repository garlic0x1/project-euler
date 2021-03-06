/*
Consider all integer combinations of ab for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:

22=4, 23=8, 24=16, 25=32
32=9, 33=27, 34=81, 35=243
42=16, 43=64, 44=256, 45=1024
52=25, 53=125, 54=625, 55=3125
If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:

4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125

How many distinct terms are in the sequence generated by ab for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
*/

package main

import (
	"fmt"
	"math/big"
)

func main() {
	fmt.Println(countSieve(sieve(5, 5)))
	fmt.Println(countSieve(sieve(100, 100)))
}

func countSieve(s map[string]bool) int {
	sum := 0
	for _, _ = range s {
		sum++
	}
	return sum
}

func sieve(a int, b int) map[string]bool {
	arr := make(map[string]bool)
	for i := 2; i <= a; i++ {
		for j := 2; j <= b; j++ {
			val := new(big.Int).Exp(big.NewInt(int64(i)), big.NewInt(int64(j)), nil)
			_, u := arr[val.String()]
			if u == false {
				arr[val.String()] = true
			}
		}
	}
	return arr
}
