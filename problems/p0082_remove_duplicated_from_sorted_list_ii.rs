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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn delete(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() {
                return head;
            }
            let head = head.unwrap();
            let v = head.val;
            let next = head.next;
            match next {
                Some(n) => {
                    if v == n.val {
                        let mut next_next = n.next;
                        while let Some(nn) = next_next {
                            if nn.val != v {
                                return delete(Some(nn));
                            }
                            next_next = nn.next;
                        }
                        None
                    } else {
                        let mut new_node = ListNode::new(v);
                        new_node.next = delete(Some(n));
                        Some(Box::new(new_node))
                    }
                }
                None => Some(Box::new(ListNode::new(v))),
            }
        }

        delete(head)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0082() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(3);
        let mut t1_5 = ListNode::new(4);
        let mut t1_6 = ListNode::new(4);
        let t1_7 = ListNode::new(5);
        t1_6.next = Some(Box::new(t1_7));
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(2);
        let r1_3 = ListNode::new(5);
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Box::new(r1_1),
            Solution::delete_duplicates(Some(Box::new(t1_1))).unwrap()
        );

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(1);
        let mut t2_3 = ListNode::new(2);
        let t2_4 = ListNode::new(3);
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(2);
        let r2_2 = ListNode::new(3);
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Box::new(r2_1),
            Solution::delete_duplicates(Some(Box::new(t2_1))).unwrap()
        );

        let mut t3_1 = ListNode::new(1);
        let t3_2 = ListNode::new(1);
        t3_1.next = Some(Box::new(t3_2));

        assert!(Solution::delete_duplicates(Some(Box::new(t3_1))).is_none());
    }
}
