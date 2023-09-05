package main

import (
	"fmt"

	m "leetcode.com/maximum_subarray"
)

func main() {
	n1 := []int{-2, 1, -3, 4, -1, 2, 1, -5, 4}
	n2 := []int{-2, 1}
	n3 := []int{5, 4, -1, 7, 8}

	fmt.Printf("Result %d\n", m.MaxSubArray(n1))
	fmt.Printf("Next\n")
	fmt.Printf("Result %d\n", m.MaxSubArray(n2))
	fmt.Printf("Next\n")
	fmt.Printf("Result %d\n", m.MaxSubArray(n3))
}
