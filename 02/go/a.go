package main

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

type Play int

const (
	Rock Play = iota
	Paper
	Scissors
)

func NewPlayFromString(s string) Play {
	switch s {
	case "A":
		return Rock
	case "X":
		return Rock
	case "B":
		return Paper
	case "Y":
		return Paper
	case "C":
		return Scissors
	case "Z":
		return Scissors
	}
	panic("bad type")
}

type round struct {
	opponent Play
	you      Play
}

func (r round) score() (int, error) {
	shapeScore, err := r.scoreShape()
	if err != nil {
		return 0, err
	}
	outcomeScore, err := r.scoreOutcome()
	if err != nil {
		return 0, err
	}
	return shapeScore + outcomeScore, nil
}

func (r round) scoreShape() (int, error) {
	switch r.you {
	case Rock:
		return 1, nil
	case Paper:
		return 2, nil
	case Scissors:
		return 3, nil
	default:
		return 0, errors.New("invalid shape")
	}

}

func (r round) scoreOutcome() (int, error) {
	difference := r.you - r.opponent
	difMod := difference % 3
	if difMod < 0 {
		difMod += 3
	}
	switch difMod {
	case 0:
		return 3, nil // draw
	case 1:
		return 6, nil // win
	case 2:
		return 0, nil // loss
	}
	return 0, errors.New("invalid game state")
}

func partA(filename string) {
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
		r := round{
			opponent: NewPlayFromString(chars[0]),
			you:      NewPlayFromString(chars[1]),
		}
		roundScore, err := r.score()
		if err != nil {
			panic(err.Error())
		}
		totalScore += roundScore
		rounds = append(rounds, r)
	}
	fmt.Println("totalScore", totalScore)
}
