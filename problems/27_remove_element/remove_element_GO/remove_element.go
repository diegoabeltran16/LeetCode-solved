package main

import "fmt"
import "C"

// Optimized for performance
func removeElementPerformance(nums []int, val int) int {
	k := 0
	for i := 0; i < len(nums); i++ {
		if nums[i] != val {
			if k != i {
				nums[k] = nums[i]
			}
			k++
		}
	}
	return k
}

// Optimized for memory usage
func removeElementMemory(nums []int, val int) []int {
	result := []int{}
	for _, num := range nums {
		if num != val {
			result = append(result, num)
		}
	}
	return result
}

// Optimized for readability
func removeElementReadable(nums []int, val int) int {
	nextPosition := 0 // Pointer for the next position of non-val elements
	for current := 0; current < len(nums); current++ {
		if nums[current] != val {
			nums[nextPosition] = nums[current]
			nextPosition++
		}
	}
	return nextPosition
}

func main() {
	nums := []int{3, 2, 2, 3}
	val := 3

	// Performance optimization
	k := removeElementPerformance(nums, val)
	fmt.Printf("Performance: k = %d, nums = %v\n", k, nums[:k])

	// Memory optimization
	nums = []int{3, 2, 2, 3}
	result := removeElementMemory(nums, val)
	fmt.Printf("Memory: nums = %v\n", result)

	// Readability optimization
	nums = []int{3, 2, 2, 3}
	k = removeElementReadable(nums, val)
	fmt.Printf("Readability: k = %d, nums = %v\n", k, nums[:k])
}
