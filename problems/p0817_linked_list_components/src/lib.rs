use std::collections::HashSet;

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
    pub fn recurse(
        head: &Option<Box<ListNode>>,
        nums: &HashSet<i32>,
        prev: bool,
        count: i32,
    ) -> i32 {
        if let Some(node) = head {
            let current = nums.contains(&node.val);
            Self::recurse(
                &node.next,
                nums,
                current,
                if prev && current { count - 1 } else { count },
            )
        } else {
            count
        }
    }

    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let count = nums.len() as i32;
        Self::recurse(&head, &nums.into_iter().collect(), false, count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0817() {
        let mut t1_1 = ListNode::new(0);
        let mut t1_2 = ListNode::new(1);
        let mut t1_3 = ListNode::new(2);
        let t1_4 = ListNode::new(3);
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert_eq!(
            Solution::num_components(Some(Box::new(t1_1)), vec![0, 1, 3]),
            2
        );

        let mut t1_1 = ListNode::new(0);
        let mut t1_2 = ListNode::new(1);
        let mut t1_3 = ListNode::new(2);
        let mut t1_4 = ListNode::new(3);
        let t1_5 = ListNode::new(4);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert_eq!(
            Solution::num_components(Some(Box::new(t1_1)), vec![0, 1, 2, 3]),
            1
        );
    }
}
