package main

import (
	"errors"
	"os"
	"strings"
)

type Rucksack struct {
	compartment string
}

func (r Rucksack) split_compartment() (string, string) {
	return r.compartment[:len(r.compartment)/2], r.compartment[len(r.compartment)/2:]
}

func (r Rucksack) get_priority() (int, error) {
	compartment1, compartment2 := r.split_compartment()

	for _, char := range compartment1 {
		if strings.ContainsRune(compartment2, char) {
			if char >= 'a' && char <= 'z' {
				return int(char) - int('a') + 1, nil
			} else if char >= 'A' && char <= 'Z' {
				return int(char) - int('A') + 27, nil
			}
		}
	}

	return 0, errors.New("A common character was not found")
}

func get_priority(rucksacks []Rucksack) (int, error) {
	first_rucksack := rucksacks[0]

	for _, char := range first_rucksack.compartment {
		all_have_rune := true
		for i := 1; i < len(rucksacks); i++ {
			if !strings.ContainsRune(rucksacks[i].compartment, char) {
				all_have_rune = false
				break
			}
		}

		if all_have_rune {
			if char >= 'a' && char <= 'z' {
				return int(char) - int('a') + 1, nil
			} else if char >= 'A' && char <= 'Z' {
				return int(char) - int('A') + 27, nil
			}
		}
	}

	return 0, errors.New("A common character was not found")
}

func create_rucksacks(path string) []Rucksack {
	rucksacks := []Rucksack{}

	file, _ := os.ReadFile(path)
	contents := string(file)

	lines := strings.Split(contents, "\n")

	for _, line := range lines {
		rucksacks = append(rucksacks, Rucksack{line})
	}

	return rucksacks
}

func chunk_rucksacks(rucksacks []Rucksack) [][3]Rucksack {
	chunks := [][3]Rucksack{}

	for i := 0; i < len(rucksacks); i += 3 {
		chunks = append(chunks, [3]Rucksack{rucksacks[i], rucksacks[i+1], rucksacks[i+2]})
	}

	return chunks
}

func solve_part_one(path string) int {
	rucksacks := create_rucksacks(path)

	sum := 0
	for _, rucksack := range rucksacks {
		priority, _ := rucksack.get_priority()
		sum += priority
	}

	return sum
}

func solve_part_two(path string) int {
	rucksacks := create_rucksacks(path)
	chunked := chunk_rucksacks(rucksacks)

	sum := 0
	for _, chunk := range chunked {
		priority, _ := get_priority([]Rucksack{chunk[0], chunk[1], chunk[2]})
		sum += priority
	}

	return sum
}

func main() {
	println(solve_part_one("input.txt"))
	println(solve_part_two("input.txt"))
}
