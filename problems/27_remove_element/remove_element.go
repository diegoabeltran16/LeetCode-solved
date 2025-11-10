package main

import (
	"fmt"
)

// removeElement removes all occurrences of val in nums in-place.
// Returns the number of elements in nums which are not equal to val.
func removeElement(nums []int, val int) int {
	k := 0 // Pointer for the next position of non-val elements
	for _, num := range nums {
		if num != val {
			nums[k] = num
			k++
		}
	}
	return k
}

func main() {
	// Example usage and simple tests
	testCases := []struct {
		nums []int
		val  int
	}{{
		[]int{3, 2, 2, 3},
		3},
		{[]int{0, 1, 2, 2, 3, 0, 4, 2},
		2},
	}

	for _, tc := range testCases {
		originalNums := append([]int{}, tc.nums...)
		k := removeElement(tc.nums, tc.val)
		fmt.Printf("removeElement(%v, %d) -> k = %d, nums = %v + %v\n", originalNums, tc.val, k, tc.nums[:k], make([]string, len(tc.nums)-k))
	}
}
