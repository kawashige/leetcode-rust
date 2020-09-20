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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn insert(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            if let Some(mut n) = head {
                if val <= n.val {
                    let mut l = ListNode::new(val);
                    l.next = Some(n);
                    Some(Box::new(l))
                } else {
                    n.next = insert(n.next, val);
                    Some(n)
                }
            } else {
                Some(Box::new(ListNode::new(val)))
            }
        }

        let mut result = None;
        let mut node = head;
        while let Some(n) = node {
            result = insert(result, n.val);
            node = n.next;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0147() {
        let mut t1_1 = ListNode::new(4);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(1);
        let t1_4 = ListNode::new(3);
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(2);
        let mut r1_3 = ListNode::new(3);
        let r1_4 = ListNode::new(4);
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        let result = Solution::insertion_sort_list(Some(Box::new(t1_1))).unwrap();
        assert_eq!(result, Box::new(r1_1));

        let mut t2_1 = ListNode::new(-1);
        let mut t2_2 = ListNode::new(5);
        let mut t2_3 = ListNode::new(3);
        let mut t2_4 = ListNode::new(4);
        let t2_5 = ListNode::new(0);
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(-1);
        let mut r2_2 = ListNode::new(0);
        let mut r2_3 = ListNode::new(3);
        let mut r2_4 = ListNode::new(4);
        let r2_5 = ListNode::new(5);
        r2_4.next = Some(Box::new(r2_5));
        r2_3.next = Some(Box::new(r2_4));
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        let result = Solution::insertion_sort_list(Some(Box::new(t2_1))).unwrap();
        assert_eq!(result, Box::new(r2_1));
    }
}
