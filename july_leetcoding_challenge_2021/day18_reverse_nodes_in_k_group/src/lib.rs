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

    pub fn reverse(
        head: Option<Box<ListNode>>,
        nodes: Option<Box<ListNode>>,
        d: i32,
        k: i32,
        max: i32,
    ) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if d == max {
                Some(node)
            } else if d % k == 0 {
                let next = node.next.take();
                node.next = nodes;

                if node.next.is_some() {
                    let mut tail = node.next.as_mut().unwrap();
                    while tail.next.is_some() {
                        tail = tail.next.as_mut().unwrap();
                    }
                    tail.next = Self::reverse(next, None, d + 1, k, max);
                } else {
                    node.next = Self::reverse(next, None, d + 1, k, max);
                }

                Some(node)
            } else {
                let next = node.next.take();
                node.next = nodes;
                Self::reverse(next, Some(node), d + 1, k, max)
            }
        } else {
            None
        }
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let len = Self::len(&head);

        Self::reverse(head, None, 1, k, (len / k) * k + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day18() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(4);
        let t1_5 = ListNode::new(5);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(2);
        let mut r1_2 = ListNode::new(1);
        let mut r1_3 = ListNode::new(4);
        let mut r1_4 = ListNode::new(3);
        let r1_5 = ListNode::new(5);
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::reverse_k_group(Some(Box::new(t1_1)), 2),
            Some(Box::new(r1_1))
        );

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(3);
        let mut t2_4 = ListNode::new(4);
        let t2_5 = ListNode::new(5);
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(3);
        let mut r2_2 = ListNode::new(2);
        let mut r2_3 = ListNode::new(1);
        let mut r2_4 = ListNode::new(4);
        let r2_5 = ListNode::new(5);
        r2_4.next = Some(Box::new(r2_5));
        r2_3.next = Some(Box::new(r2_4));
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Solution::reverse_k_group(Some(Box::new(t2_1)), 3),
            Some(Box::new(r2_1))
        );

        let mut t3_1 = ListNode::new(1);
        let mut t3_2 = ListNode::new(2);
        let mut t3_3 = ListNode::new(3);
        let mut t3_4 = ListNode::new(4);
        let t3_5 = ListNode::new(5);
        t3_4.next = Some(Box::new(t3_5));
        t3_3.next = Some(Box::new(t3_4));
        t3_2.next = Some(Box::new(t3_3));
        t3_1.next = Some(Box::new(t3_2));

        let mut r3_1 = ListNode::new(1);
        let mut r3_2 = ListNode::new(2);
        let mut r3_3 = ListNode::new(3);
        let mut r3_4 = ListNode::new(4);
        let r3_5 = ListNode::new(5);
        r3_4.next = Some(Box::new(r3_5));
        r3_3.next = Some(Box::new(r3_4));
        r3_2.next = Some(Box::new(r3_3));
        r3_1.next = Some(Box::new(r3_2));

        assert_eq!(
            Solution::reverse_k_group(Some(Box::new(t3_1)), 1),
            Some(Box::new(r3_1))
        );
    }
}
