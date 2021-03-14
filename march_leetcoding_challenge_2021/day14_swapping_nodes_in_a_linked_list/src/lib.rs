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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn recurse(head: &Option<Box<ListNode>>, nums: &mut Vec<i32>) {
            if let Some(node) = head {
                nums.push(node.val);
                recurse(&node.next, nums);
            }
        }

        let mut nums = Vec::new();
        recurse(&head, &mut nums);
        let l = nums.len();
        nums.swap(k as usize - 1, l - k as usize);

        let mut dummy = Box::new(ListNode::new(-1));
        let mut next = &mut dummy;
        for n in nums {
            next = next.next.get_or_insert(Box::new(ListNode::new(n)));
        }

        dummy.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day14() {
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
            Solution::swap_nodes(Some(Box::new(t1_1)), 2),
            Some(Box::new(r1_1))
        );

        assert_eq!(
            Solution::swap_nodes(Some(Box::new(ListNode::new(1))), 1),
            Some(Box::new(ListNode::new(1)))
        );

        let mut t2_1 = ListNode::new(1);
        let t2_2 = ListNode::new(2);
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(2);
        let r2_2 = ListNode::new(1);
        r2_1.next = Some(Box::new(r2_2));

        assert_eq!(
            Solution::swap_nodes(Some(Box::new(t2_1)), 1),
            Some(Box::new(r2_1))
        );

        let mut t3_1 = ListNode::new(1);
        let mut t3_2 = ListNode::new(2);
        let t3_3 = ListNode::new(3);
        t3_2.next = Some(Box::new(t3_3));
        t3_1.next = Some(Box::new(t3_2));

        let mut r3_1 = ListNode::new(1);
        let mut r3_2 = ListNode::new(2);
        let r3_3 = ListNode::new(3);
        r3_2.next = Some(Box::new(r3_3));
        r3_1.next = Some(Box::new(r3_2));

        assert_eq!(
            Solution::swap_nodes(Some(Box::new(t3_1)), 2),
            Some(Box::new(r3_1))
        );
    }
}
