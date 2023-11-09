package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPartOne(t *testing.T) {
	assert.Equal(t, solve_part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5)
	assert.Equal(t, solve_part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6)
	assert.Equal(t, solve_part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10)
	assert.Equal(t, solve_part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11)
}

func TestPartTwo(t *testing.T) {
	assert.Equal(t, solve_part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19)
	assert.Equal(t, solve_part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23)
	assert.Equal(t, solve_part_two("nppdvjthqldpwncqszvftbrmjlhg"), 23)
	assert.Equal(t, solve_part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29)
	assert.Equal(t, solve_part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26)
}
