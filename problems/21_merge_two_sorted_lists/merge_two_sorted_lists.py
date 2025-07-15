#!/usr/bin/env python3
"""
Merge Two Sorted Lists (LeetCode #21)

Supports both standalone usage and polyglot_runner JSON input.
"""

import sys
import json


class ListNode:
    """Definition for singly-linked list node."""
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self):
        return f"{self.val}->{repr(self.next)}" if self.next else f"{self.val}"


def merge_two_lists(l1, l2):
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
    dummy = ListNode(0)
    current = dummy
    for v in values:
        current.next = ListNode(v)
        current = current.next
    return dummy.next


def list_to_pylist(head):
    result = []
    while head:
        result.append(head.val)
        head = head.next
    return result


def run_polyglot_mode():
    try:
        if sys.stdin.isatty():
            return False
        data = sys.stdin.read().strip()
        if not data:
            return False
        xs, ys = json.loads(data)
        l1 = build_list(xs)
        l2 = build_list(ys)
        merged = merge_two_lists(l1, l2)
        sys.stdout.write(json.dumps(list_to_pylist(merged)))
        return True
    except Exception:
        return False


if __name__ == "__main__":
    if not run_polyglot_mode():
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
