#!/usr/bin/env python3
"""
Merge Two Sorted Lists (LeetCode #21)

Given the heads of two sorted linked lists `l1` and `l2`, merge them into a single
sorted linked list by splicing together the nodes of the input lists.
This reuses existing nodes (O(1) extra space) and runs in O(n + m) time.

Example:
    $ python merge_two_sorted_lists.py
    merge_two_lists([], []) -> []
    merge_two_lists([], [0]) -> [0]
    merge_two_lists([1, 2, 4], [1, 3, 4]) -> [1, 1, 2, 3, 4, 4]
"""


class ListNode(object):
    """Definition for singly-linked list node."""
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self):
        # Represent the list from this node onward for debugging
        return f"{self.val}->{repr(self.next)}" if self.next else f"{self.val}"


def merge_two_lists(l1, l2):
    """
    Merge two sorted linked lists in-place by reusing nodes.

    Args:
        l1 (ListNode): Head of the first sorted linked list.
        l2 (ListNode): Head of the second sorted linked list.

    Returns:
        ListNode: Head of the merged sorted linked list.
    """
    dummy = ListNode(-1)  # Sentinel node to ease edge cases
    tail = dummy          # Pointer to build the merged list

    # Merge nodes until one list is exhausted
    while l1 and l2:
        if l1.val < l2.val:
            tail.next = l1
            l1 = l1.next
        else:
            tail.next = l2
            l2 = l2.next
        tail = tail.next

    # Attach the remaining non-empty list (if any)
    tail.next = l1 or l2
    return dummy.next


def build_list(values):
    """
    Build a linked list from a Python list of integer values.

    Args:
        values (List[int]): List of node values.

    Returns:
        ListNode: Head of the constructed linked list.
    """
    dummy = ListNode(0)
    curr = dummy
    for v in values:
        curr.next = ListNode(v)
        curr = curr.next
    return dummy.next


def list_to_pylist(head):
    """
    Convert a linked list back into a Python list of integer values.

    Args:
        head (ListNode): Head of the linked list.

    Returns:
        List[int]: List of node values for easy comparison.
    """
    result = []
    curr = head
    while curr:
        result.append(curr.val)
        curr = curr.next
    return result


if __name__ == "__main__":
    # Example usage and simple tests
    test_cases = [
        ([], []),
        ([], [0]),
        ([1, 2, 4], [1, 3, 4]),
    ]

    for xs, ys in test_cases:
        l1 = build_list(xs)
        l2 = build_list(ys)
        merged = merge_two_lists(l1, l2)
        print(f"merge_two_lists({xs}, {ys}) -> {list_to_pylist(merged)}")
