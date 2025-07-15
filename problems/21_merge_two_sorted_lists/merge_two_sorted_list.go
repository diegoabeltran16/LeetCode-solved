// problems/21_merge_two_sorted_lists/merge_two_sorted_list.go
package main

import (
	"encoding/json"
	"io"
	"os"
)

// ListNode defines a node in a singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// mergeTwoLists merges two sorted linked lists in-place.
func mergeTwoLists(l1, l2 *ListNode) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for l1 != nil && l2 != nil {
		if l1.Val < l2.Val {
			tail.Next = l1
			l1 = l1.Next
		} else {
			tail.Next = l2
			l2 = l2.Next
		}
		tail = tail.Next
	}
	if l1 != nil {
		tail.Next = l1
	} else {
		tail.Next = l2
	}
	return dummy.Next
}

// buildList constructs a linked list from a slice of ints.
func buildList(vals []int) *ListNode {
	dummy := &ListNode{}
	curr := dummy
	for _, v := range vals {
		curr.Next = &ListNode{Val: v}
		curr = curr.Next
	}
	return dummy.Next
}

// listToSlice converts a linked list to a slice of ints.
func listToSlice(head *ListNode) []int {
	var out []int
	for p := head; p != nil; p = p.Next {
		out = append(out, p.Val)
	}
	return out
}

func main() {
	// Read entire stdin as JSON [[...], [...]]
	data, err := io.ReadAll(os.Stdin)
	if err != nil {
		os.Exit(1)
	}

	var lists [][]int
	if err := json.Unmarshal(data, &lists); err != nil {
		os.Exit(1)
	}

	l1Vals := lists[0]
	l2Vals := lists[1]

	// Build linked lists, merge, then print result as JSON
	l1 := buildList(l1Vals)
	l2 := buildList(l2Vals)
	merged := mergeTwoLists(l1, l2)
	result := listToSlice(merged)

	enc := json.NewEncoder(os.Stdout)
	enc.Encode(result)
}
