package main

import (
	"testing"
)

func TestPartOne(t *testing.T) {
	if "CMZ" != solve_part_one("test_input.txt") {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	if "MCD" != solve_part_two("test_input.txt") {
		t.Fail()
	}
}
