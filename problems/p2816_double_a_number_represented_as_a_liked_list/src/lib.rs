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
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = &head;
        let mut digits = Vec::new();
        while let Some(n) = node {
            digits.push(n.val);
            node = &n.next;
        }

        let mut head = None;
        let mut carry = 0;
        while let Some(d) = digits.pop() {
            let val = d * 2 + carry;
            carry = val / 10;
            let mut node = ListNode::new(val % 10);
            node.next = head;
            head = Some(Box::new(node));
        }
        if carry == 1 {
            let mut node = ListNode::new(1);
            node.next = head;
            head = Some(Box::new(node));
        }

        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2816() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(8);
        let t1_3 = ListNode::new(9);
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(3);
        let mut r1_2 = ListNode::new(7);
        let r1_3 = ListNode::new(8);
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::double_it(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        let mut t2_1 = ListNode::new(9);
        let mut t2_2 = ListNode::new(9);
        let t2_3 = ListNode::new(9);
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(1);
        let mut r2_2 = ListNode::new(9);
        let mut r2_3 = ListNode::new(9);
        let r2_4 = ListNode::new(8);
        r2_3.next = Some(Box::new(r2_4));
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Solution::double_it(Some(Box::new(t2_1))),
            Some(Box::new(r2_1))
        );
    }
}
