package main

import (
	"os"
	"sort"
	"strconv"
	"strings"
)

func get_calories(path string) ([]int, error) {
	bytes, _ := os.ReadFile(path)

	contents := string(bytes)
	lines := strings.Split(contents, "\n")

	calories := []int{0}

	for _, line := range lines {
		if line == "" {
			calories = append(calories, 0)
		} else {
			number, err := strconv.Atoi(line)

			if err != nil {
				return nil, err
			}

			calories[len(calories)-1] += number
		}
	}

	return calories, nil
}

func solve_part_one(path string) int {
	calories, err := get_calories(path)

	if err != nil {
		panic(err)
	}

	var max int
	for _, calory := range calories {
		if calory > max {
			max = calory
		}
	}

	return max
}

func solve_part_two(path string) int {
	calories, err := get_calories(path)

	if err != nil {
		panic(err)
	}

	sort.Slice(calories, func(i, j int) bool {
		return calories[i] > calories[j]
	})

	return calories[0] + calories[1] + calories[2]
}

func main() {
	println(solve_part_one("input.txt"))
	println(solve_part_two("input.txt"))
}
