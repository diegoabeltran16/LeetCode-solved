/**
 * Repository version for Problem 21: Merge Two Sorted Lists
 *
 * Compiles under your C CI (c.yml) with:
 *   gcc -c merge_two_sorted_list.c -o /tmp/merge_two_sorted_list.o
 */

#include <stdio.h>
#include <stdlib.h>

/* Definition for singly-linked list node. */
struct ListNode {
    int val;
    struct ListNode *next;
};

/**
 * Merge two sorted linked lists in-place and return the head
 * of the merged sorted list.
 *
 * Time Complexity: O(n + m)
 * Space Complexity: O(1) extra (in-place)
 */
struct ListNode* mergeTwoLists(struct ListNode* l1, struct ListNode* l2) {
    struct ListNode dummy;
    struct ListNode *tail = &dummy;
    dummy.next = NULL;

    while (l1 != NULL && l2 != NULL) {
        if (l1->val < l2->val) {
            tail->next = l1;
            l1 = l1->next;
        } else {
            tail->next = l2;
            l2 = l2->next;
        }
        tail = tail->next;
    }
    /* Attach whichever list remains */
    tail->next = (l1 != NULL) ? l1 : l2;
    return dummy.next;
}

/**
 * Helper: build a linked list from an array of ints.
 * Returns the head of the new list.
 */
struct ListNode* buildList(const int *vals, size_t n) {
    struct ListNode dummy;
    struct ListNode *tail = &dummy;
    dummy.next = NULL;

    for (size_t i = 0; i < n; ++i) {
        struct ListNode *node = malloc(sizeof(*node));
        if (!node) exit(EXIT_FAILURE);
        node->val = vals[i];
        node->next = NULL;
        tail->next = node;
        tail = node;
    }
    return dummy.next;
}

/**
 * Helper: print a linked list to stdout in `[v1, v2, ...]` format.
 */
void printList(const struct ListNode *head) {
    const struct ListNode *cur = head;
    printf("[");
    while (cur) {
        printf("%d", cur->val);
        cur = cur->next;
        if (cur) printf(", ");
    }
    printf("]\n");
}

/**
 * Helper: free all nodes in a linked list.
 */
void freeList(struct ListNode *head) {
    while (head) {
        struct ListNode *next = head->next;
        free(head);
        head = next;
    }
}

/**
 * Demo main() exercising the examples:
 *   [] + []      -> []
 *   [] + [0]     -> [0]
 *   [1,2,4]+[1,3,4] -> [1,1,2,3,4,4]
 */
int main(void) {
    /* Case 1: both empty */
    printList(mergeTwoLists(NULL, NULL));

    /* Case 2: one empty, one has [0] */
    int arr1[] = {0};
    struct ListNode *l2 = buildList(arr1, 1);
    printList(mergeTwoLists(NULL, l2));
    freeList(l2);

    /* Case 3: [1,2,4] + [1,3,4] */
    int a3[] = {1, 2, 4};
    int b3[] = {1, 3, 4};
    struct ListNode *l3 = buildList(a3, 3);
    struct ListNode *l4 = buildList(b3, 3);
    struct ListNode *merged = mergeTwoLists(l3, l4);
    printList(merged);
    freeList(merged);

    return 0;
}
