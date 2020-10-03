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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        fn recurse(head: &Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            match head {
                Some(n) => {
                    if n.val == val {
                        recurse(&n.next, val)
                    } else {
                        let mut l = ListNode::new(n.val);
                        l.next = recurse(&n.next, val);
                        Some(Box::new(l))
                    }
                }
                None => None,
            }
        }
        recurse(&head, val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0203() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(6);
        let mut t1_4 = ListNode::new(3);
        let mut t1_5 = ListNode::new(4);
        let mut t1_6 = ListNode::new(5);
        let t1_7 = ListNode::new(6);
        t1_6.next = Some(Box::new(t1_7));
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(2);
        let mut r1_3 = ListNode::new(3);
        let mut r1_4 = ListNode::new(4);
        let r1_5 = ListNode::new(5);
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Box::new(r1_1),
            Solution::remove_elements(Some(Box::new(t1_1)), 6).unwrap()
        );
    }
}
