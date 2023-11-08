package main

import (
	"os"
	"strconv"
	"strings"
)

type Interval struct {
	start int
	end   int
}

func (i Interval) contains(other Interval) bool {
	return i.start <= other.start && i.end >= other.end
}

func (i Interval) overlaps(other Interval) bool {
	return (i.start >= other.start && i.start <= other.end) ||
		(i.end >= other.start && i.end <= other.end) ||
		(other.start >= i.start && other.start <= i.end) ||
		(other.end >= i.start && other.end <= i.end)

}

func get_intervals(path string) [][2]Interval {
	file, _ := os.ReadFile(path)
	contents := string(file)
	lines := strings.Split(contents, "\n")

	intervals := [][2]Interval{}
	for _, line := range lines {
		split_line_on_comma := strings.Split(line, ",")

		first_interval := strings.Split(split_line_on_comma[0], "-")
		second_interval := strings.Split(split_line_on_comma[1], "-")

		first_element_of_first_interval, _ := strconv.Atoi(first_interval[0])
		second_element_of_first_interval, _ := strconv.Atoi(first_interval[1])

		first_element_of_second_interval, _ := strconv.Atoi(second_interval[0])
		second_element_of_second_interval, _ := strconv.Atoi(second_interval[1])

		intervals = append(intervals, [2]Interval{
			{start: first_element_of_first_interval, end: second_element_of_first_interval},
			{start: first_element_of_second_interval, end: second_element_of_second_interval},
		})

	}

	return intervals
}

func solve_part_one(path string) int {
	intervals := get_intervals(path)
	counter := 0

	for _, interval := range intervals {
		if interval[0].contains(interval[1]) || interval[1].contains(interval[0]) {
			counter++
		}
	}

	return counter
}

func solve_part_two(path string) int {
	intervals := get_intervals(path)
	counter := 0

	for _, interval := range intervals {
		if interval[0].overlaps(interval[1]) {
			counter++
		}
	}

	return counter
}

func main() {
	println(solve_part_one("input.txt"))
	println(solve_part_two("input.txt"))
}
