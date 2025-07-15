#!/usr/bin/env python3
"""
Merge Two Sorted Lists (LeetCode #21)

Supports two modes:
1. **Demo mode** (no stdin data): runs hard-coded examples.
2. **CLI mode** (JSON on stdin): reads `[list1, list2]` and emits merged list as JSON.
"""

import sys
import json


class ListNode(object):
    """Definition for singly-linked list node."""
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self):
        return f"{self.val}->{repr(self.next)}" if self.next else f"{self.val}"


def merge_two_lists(l1, l2):
    """
    Merge two sorted linked lists in-place by reusing nodes.
    """
    dummy = ListNode(-1)
    tail = dummy
    while l1 and l2:
        if l1.val < l2.val:
            tail.next = l1
            l1 = l1.next
        else:
            tail.next = l2
            l2 = l2.next
        tail = tail.next
    tail.next = l1 or l2
    return dummy.next


def build_list(values):
    """Build a linked list from a Python list of ints."""
    dummy = ListNode(0)
    curr = dummy
    for v in values:
        curr.next = ListNode(v)
        curr = curr.next
    return dummy.next


def list_to_pylist(head):
    """Convert a linked list back into a Python list of ints."""
    result = []
    curr = head
    while curr:
        result.append(curr.val)
        curr = curr.next
    return result


def run_cli_mode():
    """
    Read JSON [list1, list2] from stdin, run merge, print JSON result, exit.
    """
    data = sys.stdin.read()
    try:
        xs, ys = json.loads(data)
    except Exception:
        # Malformed or empty stdin: fall back to demo
        return False

    l1 = build_list(xs)
    l2 = build_list(ys)
    merged = merge_two_lists(l1, l2)
    out = list_to_pylist(merged)
    sys.stdout.write(json.dumps(out))
    return True


if __name__ == "__main__":
    # If stdin contains JSON, handle it and exit
    if run_cli_mode():
        sys.exit(0)

    # Otherwise: original demo/tests
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
