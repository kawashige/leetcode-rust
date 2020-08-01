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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn find_nth(n: i32, node: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
            match node {
                Some(node) => {
                    let (next_n, next_node) = find_nth(n, node.next);
                    if next_n + 1 == n {
                        (next_n + 1, next_node)
                    } else {
                        let mut new_node = ListNode::new(node.val);
                        new_node.next = next_node;
                        (next_n + 1, Some(Box::new(new_node)))
                    }
                }
                None => (0, None),
            }
        }

        find_nth(n, head).1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0019() {
        let mut l1_1 = ListNode::new(1);
        let mut l1_2 = ListNode::new(2);
        let mut l1_3 = ListNode::new(3);
        let mut l1_4 = ListNode::new(4);
        let l1_5 = ListNode::new(5);
        l1_4.next = Some(Box::new(l1_5));
        l1_3.next = Some(Box::new(l1_4));
        l1_2.next = Some(Box::new(l1_3));
        l1_1.next = Some(Box::new(l1_2));

        let mut l2_1 = ListNode::new(1);
        let mut l2_2 = ListNode::new(2);
        let mut l2_3 = ListNode::new(3);
        let l2_4 = ListNode::new(5);
        l2_3.next = Some(Box::new(l2_4));
        l2_2.next = Some(Box::new(l2_3));
        l2_1.next = Some(Box::new(l2_2));

        let result = Solution::remove_nth_from_end(Some(Box::new(l1_1)), 2).unwrap();
        assert_eq!(l2_1, *result);
    }
}
