package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Outcome int64

const (
	Player1 Outcome = iota
	Player2
	Tie
)

type Move int64

const (
	Rock Move = iota
	Paper
	Scissors
)

type Match struct {
	player1 Move
	player2 Move
}

func move_score(m Move) int64 {
	switch m {
	case Rock:
		return 1
	case Paper:
		return 2
	case Scissors:
		return 3
	}

	panic("Invalid move")
}

func match_score(m Match) int64 {
	var winner_score int64
	switch winner(m) {
	case Player1:
		winner_score = 0
	case Player2:
		winner_score = 6
	case Tie:
		winner_score = 3
	}

	return move_score(m.player2) + winner_score
}

func scores(matches []Match) []int64 {
	var scores []int64
	for _, m := range matches {
		scores = append(scores, match_score(m))
	}
	return scores
}

func winner(m Match) Outcome {
	switch m.player1 {
	case Rock:
		switch m.player2 {
		case Rock:
			return Tie
		case Paper:
			return Player2
		case Scissors:
			return Player1
		}
	case Paper:
		switch m.player2 {
		case Rock:
			return Player1
		case Paper:
			return Tie
		case Scissors:
			return Player2
		}
	case Scissors:
		switch m.player2 {
		case Rock:
			return Player2
		case Paper:
			return Player1
		case Scissors:
			return Tie
		}
	}

	panic("Invalid match")
}

func part1(c1 string, c2 string) Match {
	var player1, player2 Move

	switch c1 {
	case "A":
		player1 = Rock
	case "B":
		player1 = Paper
	case "C":
		player1 = Scissors
	default:
		panic(fmt.Sprintf("Invalid move '%s'", c1))
	}

	switch c2 {
	case "X":
		player2 = Rock
	case "Y":
		player2 = Paper
	case "Z":
		player2 = Scissors
	default:
		panic(fmt.Sprintf("Invalid move '%s'", c2))
	}

	return Match{
		player1: player1,
		player2: player2,
	}
}

func part2(c1 string, c2 string) Match {
	var player1, player2 Move

	switch c1 {
	case "A":
		player1 = Rock
		switch c2 {
		case "X":
			player2 = Scissors
		case "Y":
			player2 = Rock
		case "Z":
			player2 = Paper
		default:
			panic(fmt.Sprintf("Invalid move '%s'", c2))
		}
	case "B":
		player1 = Paper
		switch c2 {
		case "X":
			player2 = Rock
		case "Y":
			player2 = Paper
		case "Z":
			player2 = Scissors
		default:
			panic(fmt.Sprintf("Invalid move '%s'", c2))
		}
	case "C":
		player1 = Scissors
		switch c2 {
		case "X":
			player2 = Paper
		case "Y":
			player2 = Scissors
		case "Z":
			player2 = Rock
		default:
			panic(fmt.Sprintf("Invalid move '%s'", c2))
		}
	default:
		panic(fmt.Sprintf("Invalid move '%s'", c1))
	}

	return Match{
		player1: player1,
		player2: player2,
	}
}

func load_data(converter func(string, string) Match) []Match {
	readFile, err := os.Open("data/input.txt")
	defer readFile.Close()

	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	fileScanner := bufio.NewScanner(readFile)

	fileScanner.Split(bufio.ScanLines)
	var matches []Match
	for fileScanner.Scan() {
		l := fileScanner.Text()
		parts := strings.Split(l, " ")
		matches = append(matches, converter(parts[0], parts[1]))
	}

	return matches
}

func sum(vals []int64) int64 {
	var s int64
	for _, v := range vals {
		s += v
	}
	return s
}

func main() {
	part1_score := sum(scores(load_data(part1)))
	part2_score := sum(scores(load_data(part2)))

	fmt.Printf("Part 1 final score: %d\n", part1_score)
	fmt.Printf("Part 1 final score: %d\n", part2_score)
}
