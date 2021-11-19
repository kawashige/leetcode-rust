use std::{cmp::Reverse, collections::BinaryHeap};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution {}

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut r = Vec::new();
        let mut heap = BinaryHeap::new();

        let mut node = head;
        while let Some(n) = node {
            while let Some((Reverse(v), i)) = heap.peek() {
                if &n.val <= v {
                    break;
                }
                r[*i] = n.val;
                heap.pop();
            }
            heap.push((Reverse(n.val), r.len()));
            r.push(0);
            node = n.next;
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1019() {
        let mut t1_1 = ListNode::new(2);
        let mut t1_2 = ListNode::new(1);
        let t1_3 = ListNode::new(5);
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert_eq!(
            Solution::next_larger_nodes(Some(Box::new(t1_1))),
            vec![5, 5, 0]
        );

        let mut t1_1 = ListNode::new(2);
        let mut t1_2 = ListNode::new(7);
        let mut t1_3 = ListNode::new(4);
        let mut t1_4 = ListNode::new(3);
        let t1_5 = ListNode::new(5);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert_eq!(
            Solution::next_larger_nodes(Some(Box::new(t1_1))),
            vec![7, 0, 5, 5, 0]
        );
    }
}
