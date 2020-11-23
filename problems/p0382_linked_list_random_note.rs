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

use rand::{thread_rng, Rng};
pub struct Solution {
    values: Vec<i32>,
}

impl Solution {
    /** @param head The linked list's head.
    Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut values = Vec::new();
        let mut node = head;
        while let Some(n) = node {
            values.push(n.val);
            node = n.next;
        }
        Self { values }
    }

    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        self.values[rng.gen_range(0, self.values.len())]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0382() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let t1_3 = ListNode::new(3);
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut obj = Solution::new(Some(Box::new(t1_1)));
        assert!([1, 2, 3].contains(&obj.get_random()));
    }
}
