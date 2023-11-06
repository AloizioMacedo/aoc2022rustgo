package main

import (
	"reflect"
	"testing"
)

func TestParse(t *testing.T) {
	expected := []int{6000, 4000, 11000, 24000, 10000}
	actual, _ := get_calories("test_input.txt")

	if !reflect.DeepEqual(expected, actual) {
		t.Fail()
	}
}

func TestPartOne(t *testing.T) {
	if solve_part_one("test_input.txt") != 24000 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	if solve_part_two("test_input.txt") != 45000 {
		t.Fail()
	}
}
