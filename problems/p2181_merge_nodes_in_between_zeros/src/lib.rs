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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut sums = vec![];
        while let Some(node) = current {
            if node.val == 0 {
                sums.push(0);
            } else {
                *sums.last_mut().unwrap() += node.val;
            }

            current = node.next;
        }
        sums.pop();

        let mut next = None;
        for sum in sums.into_iter().rev() {
            let mut n = ListNode::new(sum);
            n.next = next;
            next = Some(Box::new(n))
        }
        next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2181() {
        let mut t1_1 = ListNode::new(0);
        let mut t1_2 = ListNode::new(3);
        let mut t1_3 = ListNode::new(1);
        let mut t1_4 = ListNode::new(0);
        let mut t1_5 = ListNode::new(4);
        let mut t1_6 = ListNode::new(5);
        let mut t1_7 = ListNode::new(2);
        let t1_8 = ListNode::new(0);
        t1_7.next = Some(Box::new(t1_8));
        t1_6.next = Some(Box::new(t1_7));
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(4);
        let r1_2 = ListNode::new(11);
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::merge_nodes(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );
    }
}
