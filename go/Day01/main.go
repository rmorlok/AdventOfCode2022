package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	elves := make([]int, 1)

	readFile, err := os.Open("data/input.txt")
	defer readFile.Close()

	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fileScanner := bufio.NewScanner(readFile)

	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		l := fileScanner.Text()

		if len(l) == 0 {
			elves = append(elves, 0)
		} else {
			load, _ := strconv.Atoi(l)
			elves[len(elves)-1] += load
		}
	}

	sort.Ints(elves)

	fmt.Printf("Top elf is carrying: %d\n", elves[len(elves)-1])
	fmt.Printf("Top three elves are carrying: %d\n", elves[len(elves)-1]+elves[len(elves)-2]+elves[len(elves)-3])
}
