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
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();

        let mut next = &head;
        while let Some(node) = next {
            while let Some(val) = stack.pop() {
                if node.val <= val {
                    stack.push(val);
                    break;
                }
            }
            stack.push(node.val);
            next = &node.next;
        }

        let mut next = None;

        for i in (0..stack.len()).rev() {
            let mut node = ListNode::new(stack[i]);
            node.next = next;
            next = Some(Box::new(node));
        }

        next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2487() {
        let mut t1_1 = ListNode::new(5);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(13);
        let mut t1_4 = ListNode::new(3);
        let t1_5 = ListNode::new(8);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(13);
        let r1_2 = ListNode::new(8);
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::remove_nodes(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(1);
        let mut t2_3 = ListNode::new(1);
        let t2_4 = ListNode::new(1);
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(1);
        let mut r2_2 = ListNode::new(1);
        let mut r2_3 = ListNode::new(1);
        let r2_4 = ListNode::new(1);
        r2_3.next = Some(Box::new(r2_4));
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Solution::remove_nodes(Some(Box::new(t2_1))),
            Some(Box::new(r2_1))
        );
    }
}
