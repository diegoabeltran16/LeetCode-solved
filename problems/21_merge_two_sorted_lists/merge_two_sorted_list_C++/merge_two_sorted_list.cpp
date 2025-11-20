/**
 * Repository version for Problem 21: Merge Two Sorted Lists
 *
 * Compiles under your C++ CI (cpp.yml) with:
 *   g++ -std=c++17 -c merge_two_sorted_list.cpp -o /tmp/merge_two_sorted_list.o
 */

#include <iostream>
#include <vector>

// Definition for singly‐linked list node.
struct ListNode {
    int val;
    ListNode* next;
    ListNode(): val(0), next(nullptr) {}
    ListNode(int x): val(x), next(nullptr) {}
    ListNode(int x, ListNode* next): val(x), next(next) {}
};

/**
 * Merge two sorted linked lists in‐place and return the head
 * of the merged sorted list.
 *
 * Time Complexity:  O(n + m)
 * Space Complexity: O(1) extra (reuses nodes)
 */
ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
    ListNode dummy;
    ListNode* tail = &dummy;
    dummy.next = nullptr;

    while (l1 && l2) {
        if (l1->val < l2->val) {
            tail->next = l1;
            l1 = l1->next;
        } else {
            tail->next = l2;
            l2 = l2->next;
        }
        tail = tail->next;
    }

    // Attach whichever list remains
    tail->next = l1 ? l1 : l2;
    return dummy.next;
}

/**
 * Helper: build a linked list from a vector of ints.
 */
ListNode* buildList(const std::vector<int>& vals) {
    ListNode dummy;
    ListNode* tail = &dummy;
    dummy.next = nullptr;
    for (int v : vals) {
        tail->next = new ListNode(v);
        tail = tail->next;
    }
    return dummy.next;
}

/**
 * Helper: convert a linked list to a vector of ints.
 */
std::vector<int> listToVector(ListNode* head) {
    std::vector<int> out;
    while (head) {
        out.push_back(head->val);
        head = head->next;
    }
    return out;
}

/**
 * Helper: free all nodes in a linked list.
 */
void freeList(ListNode* head) {
    while (head) {
        ListNode* nxt = head->next;
        delete head;
        head = nxt;
    }
}

/**
 * Demo main() to exercise the examples.
 */
int main() {
    std::vector<std::pair<std::vector<int>, std::vector<int>>> cases = {
        {{}, {}},
        {{}, {0}},
        {{1, 2, 4}, {1, 3, 4}}
    };

    for (auto& p : cases) {
        ListNode* l1 = buildList(p.first);
        ListNode* l2 = buildList(p.second);
        ListNode* merged = mergeTwoLists(l1, l2);
        auto res = listToVector(merged);

        std::cout << "[";
        for (size_t i = 0; i < res.size(); ++i) {
            std::cout << res[i] << (i + 1 < res.size() ? ", " : "");
        }
        std::cout << "]\n";

        freeList(merged);
    }

    return 0;
}
