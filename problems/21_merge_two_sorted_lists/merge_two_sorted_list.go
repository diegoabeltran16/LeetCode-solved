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
	// Read the entire JSON payload from stdin
	data, err := io.ReadAll(os.Stdin)
	if err != nil {
		os.Exit(1)
	}

	// Expecting a JSON array: [[...], [...]]
	var lists [][]int
	if err := json.Unmarshal(data, &lists); err != nil {
		// Invalid JSON → exit (or you could fallback to demos)
		os.Exit(1)
	}

	// lists[0] is l1, lists[1] is l2
	l1 := buildList(lists[0])
	l2 := buildList(lists[1])

	// Merge and emit JSON result
	merged := mergeTwoLists(l1, l2)
	result := listToSlice(merged)

	enc := json.NewEncoder(os.Stdout)
	enc.Encode(result)
}
