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
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut in_array = vec![false; 100_001];
        for num in nums {
            in_array[num as usize] = true;
        }
        let mut dummy = Box::new(ListNode::new(-1));
        let mut next = &mut dummy;
        let mut head = head;
        while let Some(node) = head {
            if !in_array[node.val as usize] {
                next = next.next.get_or_insert(Box::new(ListNode::new(node.val)));
            }
            head = node.next;
        }

        dummy.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3217() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(3);
        let mut t1_4 = ListNode::new(4);
        let t1_5 = ListNode::new(5);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(4);
        let r1_2 = ListNode::new(5);
        r1_1.next = Some(Box::new(r1_2));
        assert_eq!(
            Solution::modified_list(vec![1, 2, 3], Some(Box::new(t1_1))),
            Some(Box::new(r1_1))
        );

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(1);
        let mut t2_4 = ListNode::new(2);
        let mut t2_5 = ListNode::new(1);
        let t2_6 = ListNode::new(2);
        t2_5.next = Some(Box::new(t2_6));
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(2);
        let mut r2_2 = ListNode::new(2);
        let r2_3 = ListNode::new(2);
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));
        assert_eq!(
            Solution::modified_list(vec![1], Some(Box::new(t2_1))),
            Some(Box::new(r2_1))
        );

        let mut t3_1 = ListNode::new(1);
        let mut t3_2 = ListNode::new(2);
        let mut t3_3 = ListNode::new(3);
        let t3_4 = ListNode::new(4);
        t3_3.next = Some(Box::new(t3_4));
        t3_2.next = Some(Box::new(t3_3));
        t3_1.next = Some(Box::new(t3_2));

        let mut r3_1 = ListNode::new(1);
        let mut r3_2 = ListNode::new(2);
        let mut r3_3 = ListNode::new(3);
        let r3_4 = ListNode::new(4);
        r3_3.next = Some(Box::new(r3_4));
        r3_2.next = Some(Box::new(r3_3));
        r3_1.next = Some(Box::new(r3_2));
        assert_eq!(
            Solution::modified_list(vec![5], Some(Box::new(t3_1))),
            Some(Box::new(r3_1))
        );
    }
}
