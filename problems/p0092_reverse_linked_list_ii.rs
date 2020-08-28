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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        fn reverse(
            head: Option<Box<ListNode>>,
            rev: &mut Vec<i32>,
            m: i32,
            n: i32,
        ) -> Option<Box<ListNode>> {
            match head {
                Some(h) => {
                    if m <= 0 {
                        rev.push(h.val);
                        if n > 0 {
                            return reverse(h.next, rev, m - 1, n - 1);
                        } else {
                            let mut l = reverse(h.next, &mut Vec::new(), m - 1, n - 1);
                            for v in rev {
                                let mut new = ListNode::new(*v);
                                new.next = l;
                                l = Some(Box::new(new));
                            }
                            return l;
                        }
                    } else {
                        let mut new = ListNode::new(h.val);
                        new.next = reverse(h.next, rev, m - 1, n - 1);
                        return Some(Box::new(new));
                    }
                }
                None => None,
            }
        }
        reverse(head, &mut Vec::new(), m - 1, n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0092() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(4);
        let t1_5 = ListNode::new(5);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(4);
        let mut r1_3 = ListNode::new(3);
        let mut r1_4 = ListNode::new(2);
        let r1_5 = ListNode::new(5);
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            r1_1,
            *Solution::reverse_between(Some(Box::new(t1_1)), 2, 4).unwrap()
        );
    }
}
