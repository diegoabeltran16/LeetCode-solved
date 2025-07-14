/**
 * Repository version for Problem 21: Merge Two Sorted Lists
 *
 * This file runs under your JavaScript CI (javascript.yml).
 * Execute with `node merge_two_sorted_list.js`.
 */

/** Definition for singly-linked list node. */
function ListNode(val, next) {
    this.val = (val === undefined ? 0 : val);
    this.next = (next === undefined ? null : next);
}

/**
 * Merge two sorted linked lists in-place and return the head of the merged list.
 *
 * @param {ListNode} list1
 * @param {ListNode} list2
 * @return {ListNode}
 */
function mergeTwoLists(list1, list2) {
    const dummy = new ListNode(-1);
    let tail = dummy;

    while (list1 !== null && list2 !== null) {
        if (list1.val < list2.val) {
            tail.next = list1;
            list1 = list1.next;
        } else {
            tail.next = list2;
            list2 = list2.next;
        }
        tail = tail.next;
    }

    // Attach any remaining nodes
    tail.next = list1 !== null ? list1 : list2;
    return dummy.next;
}

/** Helper: build a linked list from an array, returning its head. */
function buildList(vals) {
    const dummy = new ListNode(0);
    let tail = dummy;
    for (const v of vals) {
        tail.next = new ListNode(v);
        tail = tail.next;
    }
    return dummy.next;
}

/** Helper: convert a linked list to a plain array for easy inspection. */
function listToArray(head) {
    const out = [];
    let cur = head;
    while (cur !== null) {
        out.push(cur.val);
        cur = cur.next;
    }
    return out;
}

/** Demo runner to exercise example cases. */
function main() {
    const cases = [
        [[], []],
        [[], [0]],
        [[1, 2, 4], [1, 3, 4]]
    ];

    for (const [a, b] of cases) {
        const l1 = buildList(a);
        const l2 = buildList(b);
        const merged = mergeTwoLists(l1, l2);
        console.log(listToArray(merged));
    }
}

// If run directly, execute main()
if (require.main === module) {
    main();
}
