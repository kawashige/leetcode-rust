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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut l = 1;
        let mut node = head.clone();
        use std::collections::VecDeque;
        let mut values = VecDeque::new();
        while let Some(n) = node {
            if l > 1 {
                values.push_back(n.val);
            }
            l += 1;
            node = n.next;
        }

        let mut node = head;
        let mut j = 2;
        while let Some(n) = node {
            let val = if j % 2 == 0 {
                values.pop_back()
            } else {
                values.pop_front()
            };
            n.next = match val {
                Some(v) => Some(Box::new(ListNode::new(v))),
                None => None,
            };
            node = &mut n.next;
            j += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day20() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let t1_4 = ListNode::new(4);
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(4);
        let mut r1_3 = ListNode::new(2);
        let r1_4 = ListNode::new(3);
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        let mut input = Some(Box::new(t1_1));
        Solution::reorder_list(&mut input);
        println!("{:?}", input);
        assert_eq!(Box::new(r1_1), input.unwrap());

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(3);
        let mut t2_4 = ListNode::new(4);
        let t2_5 = ListNode::new(5);
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(1);
        let mut r2_2 = ListNode::new(5);
        let mut r2_3 = ListNode::new(2);
        let mut r2_4 = ::new(4);
        let r2_5 = ListNode::new(3);
        r2_4.next = Some(Box::new(r2_5));
        r2_3.next = Some(Box::new(r2_4));
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        let mut input = Some(Box::new(t2_1));
        Solution::reorder_list(&mut input);
        println!("{:?}", input);
        assert_eq!(Box::new(r2_1), input.unwrap());

        let t3_1 = ListNode::new(1);
        let r3_1 = ListNode::new(1);

        let mut input = Some(Box::new(t3_1));
        Solution::reorder_list(&mut input);
        println!("{:?}", input);
        assert_eq!(Box::new(r3_1), input.unwrap());
    }
}
