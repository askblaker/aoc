package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type record struct {
	direction string
	amount    int
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	fileName := os.Args[1:][0]
	fmt.Println("fileName: ", fileName)
	content := readFile(fileName)
	stringArray := strings.Split(content, "\n")
	getPosition(stringArray)
	getPositionAndAim(stringArray)

}

func getPosition(stringArray []string) {
	depth := 0
	horizontal := 0
	for _, s := range stringArray {
		split := strings.Split(s, " ")
		amount, _ := strconv.Atoi(split[1])
		command := record{split[0], amount}
		if command.direction == "forward" {
			horizontal += command.amount
		} else if command.direction == "down" {
			depth += command.amount
		} else if command.direction == "up" {
			depth -= command.amount
		}
	}
	fmt.Println("Depth: ", depth, " Horizontal: ", horizontal, "Multiplied: ", depth*horizontal)
}

func getPositionAndAim(stringArray []string) {
	depth := 0
	horizontal := 0
	aim := 0
	for _, s := range stringArray {
		split := strings.Split(s, " ")
		amount, _ := strconv.Atoi(split[1])
		command := record{split[0], amount}
		if command.direction == "forward" {
			horizontal += command.amount
			depth += command.amount * aim
		} else if command.direction == "down" {
			aim += command.amount
		} else if command.direction == "up" {
			aim -= command.amount
		}
	}
	fmt.Println("Depth: ", depth, " Horizontal: ", horizontal, "Multiplied: ", depth*horizontal)
}

func readFile(fileName string) string {
	dat, err := os.ReadFile(fileName)
	check(err)
	return string(dat)
}
