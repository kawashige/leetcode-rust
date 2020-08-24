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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        fn part(
            head: Option<Box<ListNode>>,
            tails: &mut Vec<i32>,
            x: i32,
        ) -> Option<Box<ListNode>> {
            match head {
                Some(n) => {
                    if n.val >= x {
                        tails.push(n.val);
                        part(n.next, tails, x)
                    } else {
                        let mut new = ListNode::new(n.val);
                        new.next = part(n.next, tails, x);
                        Some(Box::new(new))
                    }
                }
                None => {
                    let mut tail = None;
                    for t in tails.iter().rev() {
                        let mut n = ListNode::new(*t);
                        n.next = tail;
                        tail = Some(Box::new(n));
                    }
                    tail
                }
            }
        }
        part(head, &mut Vec::new(), x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0086() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(4);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(2);
        let mut t1_5 = ListNode::new(5);
        let t1_6 = ListNode::new(2);
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(2);
        let mut r1_3 = ListNode::new(2);
        let mut r1_4 = ListNode::new(4);
        let mut r1_5 = ListNode::new(3);
        let r1_6 = ListNode::new(5);
        r1_5.next = Some(Box::new(r1_6));
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Box::new(r1_1),
            Solution::partition(Some(Box::new(t1_1)), 3).unwrap()
        );
    }
}
