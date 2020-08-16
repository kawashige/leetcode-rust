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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut vals = Vec::new();

        let mut node = head.clone();
        while let Some(n) = node {
            vals.push(n.val);
            node = n.next;
        }

        if vals.len() == 1 {
            return Some(Box::new(ListNode::new(vals[0])));
        }

        let i = vals.len() - (k as usize % vals.len());
        let mut node = None;
        for v in vals[..i].iter().rev().chain(vals[i..].iter().rev()) {
            let tmp = node;
            let mut new_node = ListNode::new(*v);
            new_node.next = tmp;
            node = Some(Box::new(new_node));
        }
        node
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0061() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(4);
        let t1_5 = ListNode::new(5);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(4);
        let mut r1_2 = ListNode::new(5);
        let mut r1_3 = ListNode::new(1);
        let mut r1_4 = ListNode::new(2);
        let r1_5 = ListNode::new(3);
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Box::new(r1_1),
            Solution::rotate_right(Some(Box::new(t1_1)), 2).unwrap()
        );

        let mut t2_1 = ListNode::new(0);
        let mut t2_2 = ListNode::new(1);
        let t2_3 = ListNode::new(2);
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(2);
        let mut r2_2 = ListNode::new(0);
        let r2_3 = ListNode::new(1);
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Box::new(r2_1),
            Solution::rotate_right(Some(Box::new(t2_1)), 4).unwrap()
        );
        assert_eq!(
            Box::new(ListNode::new(1)),
            Solution::rotate_right(Some(Box::new(ListNode::new(1))), 3).unwrap()
        );
    }
}
