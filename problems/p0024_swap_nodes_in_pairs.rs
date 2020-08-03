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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        pub fn swap(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match head {
                Some(mut node) => match (*node).next {
                    Some(mut next) => {
                        let tmp = next.next;
                        (*node).next = swap(tmp);
                        next.next = Some(node);
                        Some(next)
                    }
                    None => Some(node),
                },
                None => None,
            }
        }
        swap(head)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0024() {
        let mut l1_1 = ListNode::new(1);
        let mut l1_2 = ListNode::new(2);
        let mut l1_3 = ListNode::new(3);
        let l1_4 = ListNode::new(4);
        l1_3.next = Some(Box::new(l1_4));
        l1_2.next = Some(Box::new(l1_3));
        l1_1.next = Some(Box::new(l1_2));

        let mut l2_1 = ListNode::new(2);
        let mut l2_2 = ListNode::new(1);
        let mut l2_3 = ListNode::new(4);
        let l2_4 = ListNode::new(3);
        l2_3.next = Some(Box::new(l2_4));
        l2_2.next = Some(Box::new(l2_3));
        l2_1.next = Some(Box::new(l2_2));

        let result = Solution::swap_pairs(Some(Box::new(l1_1))).unwrap();

        assert_eq!(l2_1, *result);
    }
}
