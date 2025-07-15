// File: problems/21_merge_two_sorted_lists/csharp/merge_two_sorted_list_solution.cs

using System;
using System.Collections.Generic;

/**
 * Repository Solution for Problem 21: Merge Two Sorted Lists
 */
public class Solution
{
    /**
     * Merge two sorted linked lists in-place and return the head of the merged list.
     *
     * @param list1 Head of the first sorted list.
     * @param list2 Head of the second sorted list.
     * @return Head of the merged sorted list.
     */
    public ListNode MergeTwoLists(ListNode list1, ListNode list2)
    {
        // Dummy node to simplify edge cases
        var dummy = new ListNode(0);
        var tail = dummy;

        // Merge until one list is exhausted
        while (list1 != null && list2 != null)
        {
            if (list1.val < list2.val)
            {
                tail.next = list1;
                list1 = list1.next;
            }
            else
            {
                tail.next = list2;
                list2 = list2.next;
            }
            tail = tail.next;
        }

        // Attach any remaining nodes
        tail.next = list1 ?? list2;
        return dummy.next;
    }
}

/**
 * Demo runner using top‐level statements is also possible,
 * but for clarity we include a Program class here.
 */
class Program
{
    static void Main()
    {
        var solution = new Solution();
        var testCases = new List<(int[], int[])>
        {
            (new int[]{}, new int[]{}),
            (new int[]{}, new int[]{0}),
            (new int[]{1,2,4}, new int[]{1,3,4})
        };

        foreach (var (a, b) in testCases)
        {
            var l1 = BuildList(a);
            var l2 = BuildList(b);
            var merged = solution.MergeTwoLists(l1, l2);
            Console.WriteLine(ListToArray(merged));
        }
    }

    // Build a linked list from an array
    static ListNode BuildList(int[] vals)
    {
        var dummy = new ListNode(0);
        var tail = dummy;
        foreach (var v in vals)
        {
            tail.next = new ListNode(v);
            tail = tail.next;
        }
        return dummy.next;
    }

    // Convert a linked list to a string for printing
    static string ListToArray(ListNode head)
    {
        var result = new List<int>();
        while (head != null)
        {
            result.Add(head.val);
            head = head.next;
        }
        return "[" + string.Join(", ", result) + "]";
    }
}
