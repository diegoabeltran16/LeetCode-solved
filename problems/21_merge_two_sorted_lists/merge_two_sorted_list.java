/**
 * Repository version for Problem 21: Merge Two Sorted Lists
 *
 * This file compiles under your Java CI and includes:
 *  - ListNode definition
 *  - MergeTwoSortedList class with mergeTwoLists method
 *  - Helper methods buildList, listToList
 *  - main() for quick demonstration
 */
public class MergeTwoSortedList {

    /** Definition for singly-linked list. */
    public static class ListNode {
        public int val;
        public ListNode next;
        public ListNode(int val) { this.val = val; }
    }

    /**
     * Merge two sorted linked lists in-place and return the head of the merged list.
     *
     * @param list1 Head of the first sorted list.
     * @param list2 Head of the second sorted list.
     * @return Head of the merged sorted list.
     */
    public ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        ListNode dummy = new ListNode(-1);
        ListNode tail = dummy;

        // Merge until one list runs out
        while (list1 != null && list2 != null) {
            if (list1.val < list2.val) {
                tail.next = list1;
                list1 = list1.next;
            } else {
                tail.next = list2;
                list2 = list2.next;
            }
            tail = tail.next;
        }

        // Attach the remainder
        tail.next = (list1 != null) ? list1 : list2;
        return dummy.next;
    }

    /** Build a linked list from an int array, return its head. */
    public static ListNode buildList(int[] values) {
        ListNode dummy = new ListNode(0);
        ListNode tail = dummy;
        for (int v : values) {
            tail.next = new ListNode(v);
            tail = tail.next;
        }
        return dummy.next;
    }

    /** Convert a linked list to a java.util.List<Integer> for easy printing. */
    public static java.util.List<Integer> listToList(ListNode head) {
        java.util.List<Integer> result = new java.util.ArrayList<>();
        while (head != null) {
            result.add(head.val);
            head = head.next;
        }
        return result;
    }

    /** Demo in main(): runs the examples and prints the merged lists. */
    public static void main(String[] args) {
        int[][] a = {{}, {}, {1, 2, 4}};
        int[][] b = {{}, {0}, {1, 3, 4}};
        for (int i = 0; i < a.length; i++) {
            ListNode l1 = buildList(a[i]);
            ListNode l2 = buildList(b[i]);
            ListNode merged = new MergeTwoSortedList().mergeTwoLists(l1, l2);
            System.out.println(listToList(merged));
        }
    }
}
