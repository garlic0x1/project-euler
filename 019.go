/*
You are given the following information, but you may prefer to do some research for yourself.

1 Jan 1900 was a Monday.
Thirty days has September,
April, June and November.
All the rest have thirty-one,
Saving February alone,
Which has twenty-eight, rain or shine.
And on leap years, twenty-nine.
A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
*/

package main

import "fmt"

var months = []int{
	31, // January
	28, // February
	31, // March
	30, // April
	31, // May
	30, // June
	31, // July
	31, // August
	30, // September
	31, // October
	30, // November
	31, // December
}

func main() {
	fmt.Println(firstSundays(1901, 2000, 1))
}

func firstSundays(start int, end int, startDay int) int {
	sum := 0
	day := startDay - 1
	for i := start; i <= end; i++ {
		for j, month := range months {
			if day%7 == 0 {
				sum++
			}
			if j == 1 && i%4 == 0 {
				day += 1
			}
			day += month
		}
	}
	return sum
}
