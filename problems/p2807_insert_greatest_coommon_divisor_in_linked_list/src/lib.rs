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
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut inserted = &mut dummy;

        let mut node = head;
        while let Some(n) = node {
            inserted = inserted.next.get_or_insert(Box::new(ListNode::new(n.val)));
            if n.next.is_some() {
                inserted = inserted
                    .next
                    .get_or_insert(Box::new(ListNode::new(Self::gcd(
                        n.val,
                        n.next.as_ref().unwrap().val,
                    ))));
            }

            node = n.next;
        }

        dummy.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2807() {
        let mut t1_1 = ListNode::new(18);
        let mut t1_2 = ListNode::new(6);
        let mut t1_3 = ListNode::new(10);
        let t1_4 = ListNode::new(3);
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(18);
        let mut r1_2 = ListNode::new(6);
        let mut r1_3 = ListNode::new(6);
        let mut r1_4 = ListNode::new(2);
        let mut r1_5 = ListNode::new(10);
        let mut r1_6 = ListNode::new(1);
        let r1_7 = ListNode::new(3);
        r1_6.next = Some(Box::new(r1_7));
        r1_5.next = Some(Box::new(r1_6));
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Solution::insert_greatest_common_divisors(Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );
        assert_eq!(
            Solution::insert_greatest_common_divisors(Some(Box::new(ListNode::new(7)))),
            Some(Box::new(ListNode::new(7)))
        );
    }
}
