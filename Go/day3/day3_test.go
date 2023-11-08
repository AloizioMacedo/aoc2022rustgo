package main

import "testing"

func TestParsing(t *testing.T) {
	rucksacks := create_rucksacks("test_input.txt")

	if len(rucksacks) != 6 {
		t.Fail()
	}
}

func TestPartOne(t *testing.T) {
	sum := solve_part_one("test_input.txt")

	if sum != 157 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	sum := solve_part_two("test_input.txt")

	if sum != 70 {
		t.Fail()
	}
}
