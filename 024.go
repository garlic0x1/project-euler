/*
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
*/

/*
The nth place of the permutation increments every X permutations
where x = the factorial of lesser places
*/

package main

import "fmt"

func main() {
	fmt.Println(permutation(1000000, buildArr(10)))
}

func permutation(n int, arr []int) []int {
	if len(arr) > 1 {
		add := place(n, arr)
		return append([]int{arr[add]}, permutation(n, removeFrom(arr, arr[add]))...)
	}
	return []int{arr[0]}
}

func place(n int, arr []int) int {
	if len(arr) == 0 {
		return 0
	}
	return (n - 1) / factorial(len(arr)-1) % len(arr)
}

func factorial(n int) int {
	product := 1
	for i := 1; i <= n; i++ {
		product *= i
	}
	return product
}

func removeFrom(arr []int, value int) []int {
	var result []int
	for _, x := range arr {
		if x != value {
			result = append(result, x)
		}
	}
	return result
}

func buildArr(n int) []int {
	var ret []int
	for i := 0; i < n; i++ {
		ret = append(ret, i)
	}
	return ret
}
