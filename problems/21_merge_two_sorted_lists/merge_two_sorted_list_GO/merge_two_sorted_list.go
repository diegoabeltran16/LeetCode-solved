// Package main provides an in-place merge of two sorted singly-linked lists.
package main

import (
	"fmt"
)

// ListNode defines a node in a singly-linked list.
// Val holds the integer value; Next points to the next node.
type ListNode struct {
	Val  int
	Next *ListNode
}

// mergeTwoLists merges two sorted linked lists l1 and l2 by reusing existing nodes.
// It returns the head of the new sorted list.
// Time Complexity: O(n + m), where n and m are the lengths of l1 and l2.
// Space Complexity: O(1) extra space (in-place).
func mergeTwoLists(l1, l2 *ListNode) *ListNode {
	dummy := &ListNode{}       // Sentinel node to simplify edge cases
	tail := dummy             // Tail builds the merged list

	// Traverse both lists, picking the smaller head each time.
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

	// Attach any remaining nodes (only one of l1 or l2 will be non-nil)
	tail.Next = l1
	if tail.Next == nil {
		tail.Next = l2
	}

	return dummy.Next
}

// buildList constructs a linked list from a slice of ints and returns its head.
func buildList(vals []int) *ListNode {
	dummy := &ListNode{}
	curr := dummy
	for _, v := range vals {
		curr.Next = &ListNode{Val: v}
		curr = curr.Next
	}
	return dummy.Next
}

// listToSlice converts a linked list to a slice of ints for easy printing/comparison.
func listToSlice(head *ListNode) []int {
	var out []int
	for p := head; p != nil; p = p.Next {
		out = append(out, p.Val)
	}
	return out
}

func main() {
	// Example runs demonstrating mergeTwoLists
	tests := []struct {
		l1   []int
		l2   []int
	}{{
		[]int{}, []int{},
	}, {
		[]int{}, []int{0},
	}, {
		[]int{1, 2, 4}, []int{1, 3, 4},
	}}

	for _, tc := range tests {
		r1 := buildList(tc.l1)
		r2 := buildList(tc.l2)
		merged := mergeTwoLists(r1, r2)
		fmt.Printf("mergeTwoLists(%v, %v) -> %v\n", tc.l1, tc.l2, listToSlice(merged))
	}
}
