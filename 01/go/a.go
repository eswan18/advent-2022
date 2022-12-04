package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func partA(filename string) {
	byteContents, err := os.ReadFile(filename)
	contents := string(byteContents)
	if err != nil {
		panic(err)
	}
	lines := strings.Split(contents, "\n")

	groups, err := groupsFromLines(lines)
	if err != nil {
		// What a beautiful language Go is.
		panic(err)
	}

	// Sum up the groups.
	var sums []int
	for _, group := range groups {
		sum := 0
		for _, number := range group {
			sum += number
		}
		sums = append(sums, sum)
	}

	// Get the max number.
	max := -1
	for _, sum := range sums {
		if sum > max {
			max = sum
		}
	}
	fmt.Println(max)
}

func groupsFromLines(lines []string) ([][]int, error) {
	var groups [][]int
	var currentGroup []int
	for _, line := range lines {
		isBlank := len(line) == 0
		if isBlank {
			// Add the previous current group to the groups slice.
			groups = append(groups, currentGroup)
			// Reset currentGroup.
			currentGroup = []int{}
			continue
		}
		// Regular lines can just be converted to int and added to the current group.
		lineAsInt, err := strconv.Atoi(line)
		if err != nil {
			// Wow I love Go.
			return nil, err
		}
		currentGroup = append(currentGroup, lineAsInt)
	}
	// Add the final current group to our groups.
	if len(currentGroup) > 0 {
		groups = append(groups, currentGroup)
	}
	return groups, nil
}
