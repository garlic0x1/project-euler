/*
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
*/

package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

func main() {
	fmt.Println(sumScores(sortedWords("022_names.txt")))
}

func sumScores(arr []string) int {
	sum := 0
	for i, str := range arr {
		sum += (alphaValue(str) * (i + 1))
	}
	return sum
}

func alphaValue(str string) int {
	sum := 0
	for _, c := range str {
		sum += int(c) - 64
	}
	return sum
}

func sortedWords(file string) []string {
	bytes, _ := ioutil.ReadFile(file)
	ret := strings.Split(string(bytes), ",")
	sort.Strings(ret)
	return ret
}
