package main

import "os"

func solve_part_one(buffer string) int {
	for i := 0; i < len(buffer); i++ {
		chars := buffer[i : i+4]

		chars_map := make(map[rune]interface{})
		for _, char := range chars {
			chars_map[char] = nil
		}

		if len(chars_map) == 4 {
			return i + 4
		}
	}

	return -1
}

func solve_part_two(buffer string) int {
	for i := 0; i < len(buffer); i++ {
		chars := buffer[i : i+14]

		chars_map := make(map[rune]interface{})
		for _, char := range chars {
			chars_map[char] = nil
		}

		if len(chars_map) == 14 {
			return i + 14
		}
	}

	return -1
}

func main() {
	input, _ := os.ReadFile("input.txt")
	buffer := string(input)

	println(solve_part_one(buffer))
	println(solve_part_two(buffer))
}
