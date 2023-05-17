use std::borrow::Borrow;

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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut nums = Vec::new();
        let mut current = head;

        while let Some(node) = current {
            nums.push(node.val);
            current = node.next;
        }

        (0..nums.len() / 2)
            .map(|i| nums[i] + nums[nums.len() - 1 - i])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2130() {
        let mut t1_1 = ListNode::new(5);
        let mut t1_2 = ListNode::new(4);
        let mut t1_3 = ListNode::new(2);
        let t1_4 = ListNode::new(1);
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));
        assert_eq!(Solution::pair_sum(Some(Box::new(t1_1))), 6);

        let mut t2_1 = ListNode::new(4);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(2);
        let t2_4 = ListNode::new(3);
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));
        assert_eq!(Solution::pair_sum(Some(Box::new(t2_1))), 7);

        let mut t3_1 = ListNode::new(1);
        let t3_2 = ListNode::new(100000);
        t3_1.next = Some(Box::new(t3_2));
        assert_eq!(Solution::pair_sum(Some(Box::new(t3_1))), 100001);
    }
}
