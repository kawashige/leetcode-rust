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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (Some(x), Some(y)) => {
                    if x.val <= y.val {
                        let mut tmp = ListNode::new(x.val);
                        tmp.next = merge((*x).next, Some(y));
                        Some(Box::new(tmp))
                    } else {
                        let mut tmp = ListNode::new(y.val);
                        tmp.next = merge((*y).next, Some(x));
                        Some(Box::new(tmp))
                    }
                }
                (Some(x), None) => {
                    let mut tmp = ListNode::new(x.val);
                    tmp.next = merge((*x).next, None);
                    Some(Box::new(tmp))
                }
                (None, Some(y)) => {
                    let mut tmp = ListNode::new(y.val);
                    tmp.next = merge((*y).next, None);
                    Some(Box::new(tmp))
                }
                (_, _) => None,
            }
        }
        merge(l1, l2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_21() {
        let mut l1_1 = ListNode::new(1);
        let mut l1_2 = ListNode::new(2);
        let l1_3 = ListNode::new(4);
        l1_2.next = Some(Box::new(l1_3));
        l1_1.next = Some(Box::new(l1_2));

        let mut l2_1 = ListNode::new(1);
        let mut l2_2 = ListNode::new(3);
        let l2_3 = ListNode::new(4);
        l2_2.next = Some(Box::new(l2_3));
        l2_1.next = Some(Box::new(l2_2));

        let result = Solution::merge_two_lists(Some(Box::new(l1_1)), Some(Box::new(l2_1))).unwrap();

        let mut l3_1 = ListNode::new(1);
        let mut l3_2 = ListNode::new(1);
        let mut l3_3 = ListNode::new(2);
        let mut l3_4 = ListNode::new(3);
        let mut l3_5 = ListNode::new(4);
        let l3_6 = ListNode::new(4);
        l3_5.next = Some(Box::new(l3_6));
        l3_4.next = Some(Box::new(l3_5));
        l3_3.next = Some(Box::new(l3_4));
        l3_2.next = Some(Box::new(l3_3));
        l3_1.next = Some(Box::new(l3_2));

        assert_eq!(l3_1, *result);
    }
}
