package main

import "testing"

func TestSolvePartOne(t *testing.T) {
	if solve_part_one("test_input1.txt") != 2 {
		t.Fail()
	}
}

func TestSolvePartTwo(t *testing.T) {
	if solve_part_two("test_input2.txt") != 4 {
		t.Fail()
	}
}
