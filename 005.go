/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
11, 12, 13, 14, 15, 16, 17, 18, 19, 20
*/
package main

import "fmt"

func main() {
	fmt.Println(smallestMultiple())
}

func smallestMultiple() int {
	for i := 20; true; i += 20 {
		//	fmt.Println(i)
		if i%19 == 0 &&
			i%18 == 0 &&
			i%17 == 0 &&
			i%16 == 0 &&
			i%15 == 0 &&
			i%14 == 0 &&
			i%13 == 0 &&
			i%11 == 0 &&
			i%11 == 0 {
			return i
		}
	}
	return 0
}
