package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

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
	ints := make([]int, len(stringArray))
	for i, s := range stringArray {
		ints[i], _ = strconv.Atoi(s)
	}
	count_highers(ints)
	count_highers_triple(ints)
}

func readFile(fileName string) string {
	dat, err := os.ReadFile(fileName)
	check(err)
	return string(dat)
}

func count_highers(ints []int) {
	higher_count := 0
	previous := ints[0]
	for i := 1; i < len(ints); i++ {
		current := ints[i]
		if current > previous {
			higher_count += 1
		}
		previous = current
	}
	fmt.Println("Higher count: ", higher_count)
}

func count_highers_triple(ints []int) {
	higher_count := 0
	for i := 3; i < len(ints); i++ {
		previousSet := ints[i-1] + ints[i-2] + ints[i-3]
		currentSet := ints[i] + ints[i-1] + ints[i-2]
		if currentSet > previousSet {
			higher_count += 1
		}
	}
	fmt.Println("Higher count: ", higher_count)
}
