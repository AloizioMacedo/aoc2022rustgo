package main

import (
	"os"
	"strings"
)

type Choice int

const (
	Rock Choice = iota
	Paper
	Scissors
)

func get_choice_score(c Choice) int {
	switch c {
	case Rock:
		return 1
	case Paper:
		return 2
	case Scissors:
		return 3
	}

	return 0
}

func to_choice(c rune) (Choice, error) {
	switch c {
	case 'A':
		return Rock, nil
	case 'B':
		return Paper, nil
	case 'C':
		return Scissors, nil
	case 'X':
		return Rock, nil
	case 'Y':
		return Paper, nil
	case 'Z':
		return Scissors, nil
	default:
		return 0, nil
	}
}

func to_result(c rune) (GameResult, error) {
	switch c {
	case 'X':
		return Loss, nil
	case 'Y':
		return Draw, nil
	case 'Z':
		return Win, nil
	default:
		return 0, nil
	}
}

type GameResult int

const (
	Win  GameResult = 1
	Draw            = 0
	Loss            = 2
)

func get_result_score(r GameResult) int {
	switch r {
	case Win:
		return 6
	case Draw:
		return 3
	case Loss:
		return 0
	}
	return 0
}

func get_result(you Choice, other Choice) GameResult {
	return GameResult((you - other + 3) % 3)
}

func get_choice_for_result(them Choice, result GameResult) Choice {
	return Choice((int(them) + int(result) + 3) % 3)
}

func get_choices(path string) [][2]Choice {
	bytes, _ := os.ReadFile(path)

	contents := string(bytes)
	lines := strings.Split(contents, "\n")

	choices := [][2]Choice{}
	for _, line := range lines {
		split := strings.Split(line, " ")
		choice1, _ := to_choice(rune(split[0][0]))
		choice2, _ := to_choice(rune(split[1][0]))

		choices = append(choices, [2]Choice{choice1, choice2})
	}

	return choices
}

type ChoiceResultPair struct {
	their_choice Choice
	result       GameResult
}

func get_choices_and_results(path string) []ChoiceResultPair {
	bytes, _ := os.ReadFile(path)

	contents := string(bytes)
	lines := strings.Split(contents, "\n")

	choices := []ChoiceResultPair{}
	for _, line := range lines {
		split := strings.Split(line, " ")
		choice, _ := to_choice(rune(split[0][0]))
		result, _ := to_result(rune(split[1][0]))

		choices = append(choices, ChoiceResultPair{choice, result})
	}

	return choices
}

func solve_part_one(path string) int {
	choices := get_choices(path)

	score := 0
	for _, choice_pair := range choices {
		my_choice := choice_pair[1]
		their_choice := choice_pair[0]

		result := get_result(my_choice, their_choice)

		score += get_result_score(result) + get_choice_score(my_choice)
	}

	return score
}

func solve_part_two(path string) int {
	choices := get_choices_and_results(path)

	score := 0
	for _, choice_pair := range choices {
		result := choice_pair.result
		their_choice := choice_pair.their_choice

		my_choice := get_choice_for_result(their_choice, result)

		score += get_result_score(result) + get_choice_score(my_choice)
	}

	return score
}

func main() {
	println(solve_part_one("input.txt"))
	println(solve_part_two("input.txt"))
}
