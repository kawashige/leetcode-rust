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
    pub fn len(head: &Option<Box<ListNode>>) -> i32 {
        if let Some(node) = head {
            Self::len(&node.next) + 1
        } else {
            0
        }
    }

    pub fn recurse(head: Option<Box<ListNode>>, d: i32, k: i32) -> Option<Box<ListNode>> {
        if d == k {
            return head;
        } else if let Some(node) = head {
            Self::recurse(node.next, d + 1, k)
        } else {
            None
        }
    }

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l = Self::len(&head);
        Self::recurse(head, 1, l / 2 + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0876() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(4);
        let t1_5 = ListNode::new(5);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(3);
        let mut r1_2 = ListNode::new(4);
        let r1_3 = ListNode::new(5);
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::middle_node(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(3);
        let mut t2_4 = ListNode::new(4);
        let mut t2_5 = ListNode::new(5);
        let t2_6 = ListNode::new(6);
        t2_5.next = Some(Box::new(t2_6));
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(4);
        let mut r2_2 = ListNode::new(5);
        let r2_3 = ListNode::new(6);
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Solution::middle_node(Some(Box::new(t2_1))),
            Some(Box::new(r2_1))
        );
    }
}
