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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut node = head;
        let mut result = 0;
        while let Some(n) = node {
            result <<= 1;
            if n.val == 1 {
                result |= 1;
            }
            node = n.next;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        assert_eq!(
            5,
            Solution::get_decimal_value(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(1)))
                }))
            })))
        );

        assert_eq!(
            4,
            Solution::get_decimal_value(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(0)))
                }))
            })))
        );

        assert_eq!(
            0,
            Solution::get_decimal_value(Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(0)))
            })))
        );
    }
}
