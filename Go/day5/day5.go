package main

import (
	"os"
	"strconv"
	"strings"
)

type CraneStacks struct {
	stacks [][]rune
}

func (c CraneStacks) mov(m Movement) {
	for i := 0; i < m.amount; i++ {
		to_pop := c.stacks[m.origin-1][len(c.stacks[m.origin-1])-1]

		c.stacks[m.destination-1] = append(c.stacks[m.destination-1], to_pop)
		c.stacks[m.origin-1] = c.stacks[m.origin-1][:len(c.stacks[m.origin-1])-1]
	}
}

func (c CraneStacks) mov_9001(m Movement) {
	to_pop := c.stacks[m.origin-1][len(c.stacks[m.origin-1])-m.amount : len(c.stacks[m.origin-1])]

	for _, char := range to_pop {
		c.stacks[m.destination-1] = append(c.stacks[m.destination-1], char)
	}

	c.stacks[m.origin-1] = c.stacks[m.origin-1][:len(c.stacks[m.origin-1])-m.amount]
}

type Movement struct {
	amount      int
	origin      int
	destination int
}

func new_crane_with_stacks(n_stacks int) CraneStacks {
	return CraneStacks{
		stacks: make([][]rune, n_stacks),
	}
}

func parse_line_to_crane(line string) []rune {
	results := []rune{}
	for i := 0; i < len(line); i += 4 {
		results = append(results, rune(line[i+1]))
	}

	return results
}

func parse_line_to_movement(line string) Movement {
	split := strings.Split(line, " ")

	amount, _ := strconv.Atoi(split[1])
	origin, _ := strconv.Atoi(split[3])
	destination, _ := strconv.Atoi(split[5])

	return Movement{amount, origin, destination}
}

func build_crane(lines []string) CraneStacks {
	last_line := lines[len(lines)-1]
	n_stacks := len(strings.ReplaceAll(last_line, " ", ""))

	crane_stacks := new_crane_with_stacks(n_stacks)

	for i := len(lines) - 2; i >= 0; i-- {
		line := lines[i]
		chars := parse_line_to_crane(line)

		for i, c := range chars {
			if c != ' ' {
				crane_stacks.stacks[i] = append(crane_stacks.stacks[i], c)
			}
		}
	}

	return crane_stacks
}

func parse_file(path string) (CraneStacks, []Movement) {
	file, _ := os.ReadFile(path)

	contents := string(file)

	lines := strings.Split(contents, "\n")

	empty_position := -1
	for i, line := range lines {
		if line == "" {
			empty_position = i
			break
		}
	}

	first_part := lines[:empty_position]
	second_part := lines[empty_position+1:]

	crane_stacks := build_crane(first_part)

	movements := []Movement{}
	for _, line := range second_part {
		movements = append(movements, parse_line_to_movement(line))
	}

	return crane_stacks, movements
}

func solve_part_one(path string) string {
	crane_stacks, movements := parse_file(path)

	for _, m := range movements {
		crane_stacks.mov(m)
	}

	var tops string
	for _, s := range crane_stacks.stacks {
		tops += string(s[len(s)-1])
	}

	return tops
}

func solve_part_two(path string) string {
	crane_stacks, movements := parse_file(path)

	for _, m := range movements {
		crane_stacks.mov_9001(m)
	}

	var tops string
	for _, s := range crane_stacks.stacks {
		tops += string(s[len(s)-1])
	}

	return tops
}

func main() {
	println(solve_part_one("input.txt"))
	println(solve_part_two("input.txt"))
}
