package main

import (
	"fmt"
	"os"
	"sort"
	"strings"
)

func partB(filename string) {
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

	// Sort the groups in descending order.
	sort.Slice(sums, func(i, j int) bool {
		return sums[i] > sums[j]
	})

	topThree := sums[:3]
	total := 0
	for _, sum := range topThree {
		total += sum
	}
	fmt.Println(total)
}
