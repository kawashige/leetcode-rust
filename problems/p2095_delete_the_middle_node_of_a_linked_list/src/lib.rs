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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone();
        let mut slow = None;

        while let Some(node) = fast {
            match node.next {
                None => {
                    break;
                }
                Some(fast_next) => {
                    fast = fast_next.next;
                    if slow.is_none() {
                        slow = head.as_mut()
                    } else {
                        slow = slow.unwrap().next.as_mut();
                    }
                }
            }
        }

        if let Some(before_middle) = slow {
            if let Some(mut node) = before_middle.next.take() {
                before_middle.next = node.next.take();
            }
        } else {
            return None;
        }

        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2095() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(3);
        let mut t1_3 = ListNode::new(4);
        let mut t1_4 = ListNode::new(7);
        let mut t1_5 = ListNode::new(1);
        let mut t1_6 = ListNode::new(2);
        let t1_7 = ListNode::new(6);
        t1_6.next = Some(Box::new(t1_7));
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(3);
        let mut r1_3 = ListNode::new(4);
        let mut r1_4 = ListNode::new(1);
        let mut r1_5 = ListNode::new(2);
        let r1_6 = ListNode::new(6);
        r1_5.next = Some(Box::new(r1_6));
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::delete_middle(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let t1_4 = ListNode::new(4);
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(2);
        let r1_3 = ListNode::new(4);
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::delete_middle(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        let mut t1_1 = ListNode::new(2);
        let t1_2 = ListNode::new(1);
        t1_1.next = Some(Box::new(t1_2));

        let r1_1 = ListNode::new(2);

        assert_eq!(
            Solution::delete_middle(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        assert_eq!(
            Solution::delete_middle(Some(Box::new(ListNode::new(1)))),
            None
        );
    }
}
