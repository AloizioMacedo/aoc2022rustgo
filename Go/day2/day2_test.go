package main

import (
	"reflect"
	"testing"
)

func TestParse(t *testing.T) {
	choices := get_choices("test_input.txt")

	if !reflect.DeepEqual(choices, [][2]Choice{
		{Rock, Paper},
		{Paper, Rock},
		{Scissors, Scissors},
	}) {
		t.Fail()
	}
}

func TestPartOne(t *testing.T) {
	actual := solve_part_one("test_input.txt")

	if actual != 15 {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	actual := solve_part_two("test_input.txt")

	if actual != 12 {
		t.Fail()
	}
}
