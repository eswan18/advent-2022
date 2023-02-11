package main

import (
	"os"
)

func main() {
	if len(os.Args) < 3 {
		panic("need a part (a or b) argument and a filename")
	}
	filename := os.Args[2]
	switch os.Args[1] {
	case "a":
		partA(filename)
	case "b":
		partB(filename)
	default:
		panic("Unknown part argument")
	}
}
