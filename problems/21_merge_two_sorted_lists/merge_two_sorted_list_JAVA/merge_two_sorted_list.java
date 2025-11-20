package merge_two_sorted_list_JAVA;
/**
 * Repository version for Problem 21: Merge Two Sorted Lists
 *
 * This file compiles under your Java CI (java.yml).
 * The class is package-private so the filename (merge_two_sorted_list.java)
 * does not have to match the class name.
 */

class MergeTwoSortedList {

    /** Definition for singly-linked list node. */
    static class ListNode {
        int val;
        ListNode next;
        ListNode(int val) { this.val = val; }
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

        // Attach whichever list remains
        tail.next = (list1 != null) ? list1 : list2;
        return dummy.next;
    }

    /** Helper: build a linked list from an int array. */
    public static ListNode buildList(int[] vals) {
        ListNode dummy = new ListNode(0);
        ListNode tail = dummy;
        for (int v : vals) {
            tail.next = new ListNode(v);
            tail = tail.next;
        }
        return dummy.next;
    }

    /** Helper: convert a linked list to a List<Integer> for easy output. */
    public static java.util.List<Integer> listToList(ListNode head) {
        java.util.List<Integer> out = new java.util.ArrayList<>();
        while (head != null) {
            out.add(head.val);
            head = head.next;
        }
        return out;
    }

    /** Demo main() to exercise the code. */
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
