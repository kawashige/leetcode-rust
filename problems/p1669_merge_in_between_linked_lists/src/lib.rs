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
        list1: &mut Option<Box<ListNode>>,
        depth: usize,
        a: usize,
        b: usize,
        list2: Option<Box<ListNode>>,
    ) {
        if let Some(node) = list1 {
            if depth == a - 1 {
                let mut rest = node.next.take();
                node.next = list2;
                Self::add_tail(&mut node.next, Self::get_tail(&mut rest, depth + 1, b));
            } else {
                Self::recurse(&mut node.next, depth + 1, a, b, list2);
            }
        }
    }

    pub fn add_tail(list: &mut Option<Box<ListNode>>, tail: Option<Box<ListNode>>) {
        if let Some(node) = list {
            if node.next.is_none() {
                node.next = tail;
            } else {
                Self::add_tail(&mut node.next, tail);
            }
        }
    }

    pub fn get_tail(
        list: &mut Option<Box<ListNode>>,
        depth: usize,
        b: usize,
    ) -> Option<Box<ListNode>> {
        if let Some(node) = list {
            if depth == b {
                node.next.take()
            } else {
                Self::get_tail(&mut node.next, depth + 1, b)
            }
        } else {
            None
        }
    }

    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::recurse(&mut list1, 0, a as usize, b as usize, list2);
        list1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1669() {
        let mut t1_1 = ListNode::new(0);
        let mut t1_2 = ListNode::new(1);
        let mut t1_3 = ListNode::new(2);
        let mut t1_4 = ListNode::new(3);
        let mut t1_5 = ListNode::new(4);
        let t1_6 = ListNode::new(5);
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut t2_1 = ListNode::new(1000000);
        let mut t2_2 = ListNode::new(1000001);
        let t2_3 = ListNode::new(1000002);
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r1_1 = ListNode::new(0);
        let mut r1_2 = ListNode::new(1);
        let mut r1_3 = ListNode::new(2);
        let mut r1_4 = ListNode::new(1000000);
        let mut r1_5 = ListNode::new(1000001);
        let mut r1_6 = ListNode::new(1000002);
        let r1_7 = ListNode::new(5);
        r1_6.next = Some(Box::new(r1_7));
        r1_5.next = Some(Box::new(r1_6));
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::merge_in_between(Some(Box::new(t1_1)), 3, 4, Some(Box::new(t2_1))),
            Some(Box::new(r1_1))
        );
    }
}
