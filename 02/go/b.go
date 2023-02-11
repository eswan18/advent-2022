package main

import (
	"fmt"
	"os"
	"strings"
)

type outcome int

const (
	Lose outcome = -1
	Draw         = 0
	Win          = 1
)

type roundSpec struct {
	opponent Play
	outcome  outcome
}

func newOutcomeFromString(s string) outcome {
	if len(s) > 1 {
		panic("string too long")
	}
	// Use ascii values to convert.
	r := s[0]
	value := outcome(r-'X') - 1
	return outcome(value)
}

func (r roundSpec) toRound() round {
	// confusing stuff
	you := Play(int(r.opponent)+int(r.outcome)+3) % 3
	return round{
		opponent: r.opponent,
		you:      you,
	}
}

func partB(filename string) {
	byteContents, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	contents := string(byteContents)
	lines := strings.Split(contents, "\n")

	var rounds []round
	totalScore := 0
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		chars := strings.Split(line, " ")
		if len(chars) != 2 {
			panic("Invalid line")
		}
		rs := roundSpec{
			opponent: NewPlayFromString(chars[0]),
			outcome:  newOutcomeFromString(chars[1]),
		}
		r := rs.toRound()
		roundScore, err := r.score()
		if err != nil {
			panic(err.Error())
		}
		totalScore += roundScore
		rounds = append(rounds, r)
	}
	fmt.Println("totalScore", totalScore)
}
