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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut odd = ListNode::new(-1);
        odd.next = head;

        let mut even = ListNode::new(-1);
        even.next = odd.next.as_mut().unwrap().next.take();

        let mut odd_next = &mut odd.next;
        let mut even_next = &mut even.next;

        while even_next.is_some() && even_next.as_ref().unwrap().next.is_some() {
            let mut odd_new_next = even_next.as_mut().unwrap().next.take();
            let even_new_next = odd_new_next.as_mut().unwrap().next.take();
            odd_next.as_mut().unwrap().next = odd_new_next;
            odd_next = &mut odd_next.as_mut().unwrap().next;
            even_next.as_mut().unwrap().next = even_new_next;
            even_next = &mut even_next.as_mut().unwrap().next;
        }

        odd_next.as_mut().unwrap().next = even.next;
        odd.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0328() {
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
        let mut r1_2 = ListNode::new(3);
        let mut r1_3 = ListNode::new(5);
        let mut r1_4 = ListNode::new(2);
        let r1_5 = ListNode::new(4);
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Some(Box::new(r1_1)),
            Solution::odd_even_list(Some(Box::new(t1_1)))
        );

        let mut t2_1 = ListNode::new(2);
        let mut t2_2 = ListNode::new(1);
        let mut t2_3 = ListNode::new(3);
        let mut t2_4 = ListNode::new(5);
        let mut t2_5 = ListNode::new(6);
        let mut t2_6 = ListNode::new(4);
        let t2_7 = ListNode::new(7);
        t2_6.next = Some(Box::new(t2_7));
        t2_5.next = Some(Box::new(t2_6));
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(2);
        let mut r2_2 = ListNode::new(3);
        let mut r2_3 = ListNode::new(6);
        let mut r2_4 = ListNode::new(7);
        let mut r2_5 = ListNode::new(1);
        let mut r2_6 = ListNode::new(5);
        let r2_7 = ListNode::new(4);
        r2_6.next = Some(Box::new(r2_7));
        r2_5.next = Some(Box::new(r2_6));
        r2_4.next = Some(Box::new(r2_5));
        r2_3.next = Some(Box::new(r2_4));
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Some(Box::new(r2_1)),
            Solution::odd_even_list(Some(Box::new(t2_1)))
        );

        assert_eq!(None, Solution::odd_even_list(None));

        let mut t3_1 = ListNode::new(2);
        let t3_2 = ListNode::new(1);
        t3_1.next = Some(Box::new(t3_2));

        let mut r3_1 = ListNode::new(2);
        let r3_2 = ListNode::new(1);
        r3_1.next = Some(Box::new(r3_2));

        assert_eq!(
            Some(Box::new(r3_1)),
            Solution::odd_even_list(Some(Box::new(t3_1)))
        );
    }
}
