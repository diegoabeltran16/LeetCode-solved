/// LeetCode #21: Merge Two Sorted Lists
///
/// Given the heads of two sorted linked lists `l1` and `l2`, merge them into a
/// single sorted linked list by reusing existing nodes (O(1) extra space) in O(n + m) time.
///
/// # Definition for singly-linked list
/// ```ignore
/// #[derive(PartialEq, Eq, Clone, Debug)]
/// pub struct ListNode {
///     pub val: i32,
///     pub next: Option<Box<ListNode>>,
/// }
///
/// impl ListNode {
///     #[inline]
///     pub fn new(val: i32) -> Self {
///         ListNode { val, next: None }
///     }
/// }
/// ```

/// Merges two sorted linked lists `l1` and `l2` in-place and returns the head of the merged list.
///
/// # Arguments
/// * `l1` - First sorted list.
/// * `l2` - Second sorted list.
///
/// # Returns
/// A new sorted linked list containing all nodes from `l1` and `l2`.
///
/// # Complexity
/// * Time: O(n + m)
/// * Space: O(1) extra
pub fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Dummy head to simplify edge cases
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    // Merge by selecting the smaller head from l1 or l2
    while l1.is_some() && l2.is_some() {
        let v1 = l1.as_ref().unwrap().val;
        let v2 = l2.as_ref().unwrap().val;
        if v1 < v2 {
            // Take node from l1
            let mut node = l1.unwrap();
            l1 = node.next.take();
            tail.next = Some(node);
        } else {
            // Take node from l2
            let mut node = l2.unwrap();
            l2 = node.next.take();
            tail.next = Some(node);
        }
        tail = tail.next.as_mut().unwrap();
    }

    // Attach remaining nodes
    tail.next = if l1.is_some() { l1 } else { l2 };
    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build a linked list from slice
    fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        for &v in vals {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }

    /// Helper: convert linked list to Vec
    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut curr = head;
        while let Some(node) = curr {
            v.push(node.val);
            curr = node.next;
        }
        v
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
            let out = merge_two_lists(l1, l2);
            assert_eq!(list_to_vec(out), expected);
        }
    }
}
