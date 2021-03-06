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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut values = Vec::new();
        let mut tmp = &head;
        while let Some(n) = tmp {
            values.push(n.val);
            tmp = &n.as_ref().next;
        }
        values.sort_by(|a, b| b.cmp(&a));

        let mut tmp = head;
        let mut current = &mut tmp;
        while let Some(n) = current {
            n.val = values.pop().unwrap();
            current = &mut n.next;
        }

        tmp
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0148() {
        let mut t1_1 = ListNode::new(-1);
        let mut t1_2 = ListNode::new(5);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(4);
        let t1_5 = ListNode::new(0);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(-1);
        let mut r1_2 = ListNode::new(0);
        let mut r1_3 = ListNode::new(3);
        let mut r1_4 = ListNode::new(4);
        let r1_5 = ListNode::new(5);
        r1_4.next = Some(Box::new(r1_5));
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        assert_eq!(
            Box::new(r1_1),
            Solution::sort_list(Some(Box::new(t1_1))).unwrap()
        );
    }
}
