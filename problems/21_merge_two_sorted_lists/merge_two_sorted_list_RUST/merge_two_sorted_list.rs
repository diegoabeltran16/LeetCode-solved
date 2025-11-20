/// Definition of a singly-linked list node.
/// Uses `Option<Box<ListNode>>` for ownership-safe, in-place manipulation.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Creates a new list node with given value.
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// Solution struct to group our method for LeetCode.
pub struct Solution;

impl Solution {
    /// Merge two sorted linked lists `l1` and `l2` in-place and return the head of the merged list.
    ///
    /// # Complexity
    /// - Time: O(n + m) where n and m are the lengths of `l1` and `l2`.
    /// - Space: O(1) extra, reuses existing nodes.
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Dummy node simplifies edge cases (empty inputs, head insertion)
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        // Continue until one list is exhausted
        while l1.is_some() && l2.is_some() {
            // Compare head values safely with as_ref().unwrap().val
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                // Detach node from l1
                let mut node = l1.unwrap();
                l1 = node.next.take();
                tail.next = Some(node);
            } else {
                // Detach node from l2
                let mut node = l2.unwrap();
                l2 = node.next.take();
                tail.next = Some(node);
            }
            // Advance the tail
            tail = tail.next.as_mut().unwrap();
        }

        // Attach any remaining nodes (only one of l1/l2 is non-empty)
        tail.next = if l1.is_some() { l1 } else { l2 };

        // Skip dummy and return the real merged head
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a linked list from a slice, returning its head.
    fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        for &v in vals {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }

    /// Converts a linked list to a Vec<i32> for easy comparison.
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_merge_two_lists() {
        let cases = vec![
            (vec![], vec![], vec![]),
            (vec![], vec![0], vec![0]),
            (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
        ];

        for (a, b, expected) in cases {
            let l1 = build_list(&a);
            let l2 = build_list(&b);
            let merged = Solution::merge_two_lists(l1, l2);
            assert_eq!(list_to_vec(merged), expected);
        }
    }
}
