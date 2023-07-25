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
    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut values = Vec::new();
        let mut group = Vec::new();
        let mut g = 1;

        let mut node = head;

        while let Some(n) = node {
            group.push(n.val);
            if group.len() == g {
                if g % 2 == 0 {
                    values.append(&mut group.iter().rev().cloned().collect());
                    group.clear();
                } else {
                    values.append(&mut group);
                }
                g += 1;
            }
            node = n.next;
        }
        if group.len() % 2 == 0 {
            values.append(&mut group.iter().rev().cloned().collect());
        } else {
            values.append(&mut group);
        }

        let mut node = None;

        for i in (0..values.len()).rev() {
            let mut n = ListNode::new(values[i]);
            n.next = node;
            node = Some(Box::new(n));
        }

        node
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2074() {
        let mut t1_1 = ListNode::new(5);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(6);
        let mut t1_4 = ListNode::new(3);
        let mut t1_5 = ListNode::new(9);
        let mut t1_6 = ListNode::new(1);
        let mut t1_7 = ListNode::new(7);
        let mut t1_8 = ListNode::new(3);
        let mut t1_9 = ListNode::new(8);
        let t1_10 = ListNode::new(4);
        t1_9.next = Some(Box::new(t1_10));
        t1_8.next = Some(Box::new(t1_9));
        t1_7.next = Some(Box::new(t1_8));
        t1_6.next = Some(Box::new(t1_7));
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(5);
        let mut r1_2 = ListNode::new(6);
        let mut r1_3 = ListNode::new(2);
        let mut r1_4 = ListNode::new(3);
        let mut r1_5 = ListNode::new(9);
        let mut r1_6 = ListNode::new(1);
        let mut r1_7 = ListNode::new(4);
        let mut r1_8 = ListNode::new(8);
        let mut r1_9 = ListNode::new(3);
        let r1_10 = ListNode::new(7);
        r1_9.next = Some(Box::new(r1_10));
        r1_8.next = Some(Box::new(r1_9));
        r1_7.next = Some(Box::new(r1_8));
        r1_6.next = Some(Box::new(r1_7));
        r1_5.next = Some(Box::new(r1_6));
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::reverse_even_length_groups(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(1);
        let mut t2_3 = ListNode::new(0);
        let t2_4 = ListNode::new(6);
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(1);
        let mut r2_2 = ListNode::new(0);
        let mut r2_3 = ListNode::new(1);
        let r2_4 = ListNode::new(6);
        r2_3.next = Some(Box::new(r2_4));
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Solution::reverse_even_length_groups(Some(Box::new(t2_1))),
            Some(Box::new(r2_1))
        );

        let mut t3_1 = ListNode::new(1);
        let mut t3_2 = ListNode::new(1);
        let mut t3_3 = ListNode::new(0);
        let mut t3_4 = ListNode::new(6);
        let t3_5 = ListNode::new(5);
        t3_4.next = Some(Box::new(t3_5));
        t3_3.next = Some(Box::new(t3_4));
        t3_2.next = Some(Box::new(t3_3));
        t3_1.next = Some(Box::new(t3_2));

        let mut r3_1 = ListNode::new(1);
        let mut r3_2 = ListNode::new(0);
        let mut r3_3 = ListNode::new(1);
        let mut r3_4 = ListNode::new(5);
        let r3_5 = ListNode::new(6);
        r3_4.next = Some(Box::new(r3_5));
        r3_3.next = Some(Box::new(r3_4));
        r3_2.next = Some(Box::new(r3_3));
        r3_1.next = Some(Box::new(r3_2));

        assert_eq!(
            Solution::reverse_even_length_groups(Some(Box::new(t3_1))),
            Some(Box::new(r3_1))
        );
    }
}
