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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut values = Vec::new();
        let mut node = &head;
        while let Some(n) = node {
            values.push(n.val);
            node = &n.next;
        }
        let mut i = 0;
        let mut j = values.len() - 1;
        while i < j {
            if values[i] != values[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0234() {
        let mut t1_1 = ListNode::new(1);
        let t1_2 = ListNode::new(2);
        t1_1.next = Some(Box::new(t1_2));

        assert!(!Solution::is_palindrome(Some(Box::new(t1_1))));

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(2);
        let t2_4 = ListNode::new(1);
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        assert!(Solution::is_palindrome(Some(Box::new(t2_1))));
        assert!(Solution::is_palindrome(None));
    }
}
