// LeetCode #21: Merge Two Sorted Lists
// Merge two sorted linked lists into a single sorted linked list in-place.

/**
 * Definition for singly-linked list.
 */
class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

public class MergeTwoSortedList {

    /**
     * Merges two sorted linked lists `list1` and `list2` in-place.
     * 
     * @param list1 head of the first sorted linked list
     * @param list2 head of the second sorted linked list
     * @return head of the merged sorted linked list
     *
     * Time complexity: O(n + m), where n and m are lengths of `list1` and `list2`.
     * Space complexity: O(1) extra (in-place).
     */
    public static ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        ListNode dummy = new ListNode(-1);
        ListNode tail = dummy;

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

        tail.next = (list1 != null) ? list1 : list2;

        return dummy.next;
    }

    // Helper method to create linked list from array
    public static ListNode buildList(int[] values) {
        ListNode dummy = new ListNode(0);
        ListNode current = dummy;
        for (int val : values) {
            current.next = new ListNode(val);
            current = current.next;
        }
        return dummy.next;
    }

    // Helper method to print linked list
    public static void printList(ListNode head) {
        while (head != null) {
            System.out.print(head.val + " -> ");
            head = head.next;
        }
        System.out.println("null");
    }

    // Example usage and simple tests
    public static void main(String[] args) {
        int[][][] tests = {
            {{}, {}},
            {{}, {0}},
            {{1, 2, 4}, {1, 3, 4}}
        };

        for (int[][] test : tests) {
            ListNode l1 = buildList(test[0]);
            ListNode l2 = buildList(test[1]);
            ListNode merged = mergeTwoLists(l1, l2);
            printList(merged);
        }
    }
}
