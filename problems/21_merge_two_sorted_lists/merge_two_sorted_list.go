package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strings"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

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

func buildList(vals []int) *ListNode {
	dummy := &ListNode{}
	curr := dummy
	for _, v := range vals {
		curr.Next = &ListNode{Val: v}
		curr = curr.Next
	}
	return dummy.Next
}

func listToSlice(head *ListNode) []int {
	var out []int
	for head != nil {
		out = append(out, head.Val)
		head = head.Next
	}
	return out
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	// Read list1
	if !scanner.Scan() {
		fmt.Println("[]")
		return
	}
	line1 := scanner.Text()

	// Read list2
	if !scanner.Scan() {
		fmt.Println("[]")
		return
	}
	line2 := scanner.Text()

	var list1, list2 []int
	_ = json.Unmarshal([]byte(strings.ReplaceAll(line1, "'", "\"")), &list1)
	_ = json.Unmarshal([]byte(strings.ReplaceAll(line2, "'", "\"")), &list2)

	head1 := buildList(list1)
	head2 := buildList(list2)

	merged := mergeTwoLists(head1, head2)
	result := listToSlice(merged)

	json.NewEncoder(os.Stdout).Encode(result)
}
