use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct Value {
    val: i32,
    pos: usize,
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Solution {
    pub fn to_list_node(nums: &[i32]) -> Option<Box<ListNode>> {
        if nums.is_empty() {
            None
        } else {
            let mut node = ListNode::new(nums[0]);
            node.next = Self::to_list_node(&nums[1..]);
            Some(Box::new(node))
        }
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut heap = BinaryHeap::new();

        for i in 0..lists.len() {
            let node = lists[i].take();
            if let Some(n) = node {
                heap.push(Value { val: n.val, pos: i });
                lists[i] = n.next;
            }
        }

        let mut results = Vec::new();
        while let Some(Value { val: v, pos: i }) = heap.pop() {
            results.push(v);
            let node = lists[i].take();
            if let Some(n) = node {
                heap.push(Value { val: n.val, pos: i });
                lists[i] = n.next;
            }
        }

        Self::to_list_node(&results)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day24() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(4);
        let t1_3 = ListNode::new(5);
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(3);
        let t2_3 = ListNode::new(4);
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut t3_1 = ListNode::new(2);
        let t3_2 = ListNode::new(6);
        t3_1.next = Some(Box::new(t3_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(1);
        let mut r1_3 = ListNode::new(2);
        let mut r1_4 = ListNode::new(3);
        let mut r1_5 = ListNode::new(4);
        let mut r1_6 = ListNode::new(4);
        let mut r1_7 = ListNode::new(5);
        let r1_8 = ListNode::new(6);
        r1_7.next = Some(Box::new(r1_8));
        r1_6.next = Some(Box::new(r1_7));
        r1_5.next = Some(Box::new(r1_6));
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::merge_k_lists(vec![
                Some(Box::new(t1_1)),
                Some(Box::new(t2_1)),
                Some(Box::new(t3_1))
            ]),
            Some(Box::new(r1_1))
        );
    }
}
