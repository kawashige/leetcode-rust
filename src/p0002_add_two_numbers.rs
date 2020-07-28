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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn add(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
            carry: i32,
        ) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (Some(node1), Some(node2)) => {
                    let sum = node1.val + node2.val + carry;
                    let mut result = ListNode::new(sum % 10);
                    result.next = add(node1.next, node2.next, sum / 10);
                    Some(Box::new(result))
                }
                (Some(node1), None) => {
                    let sum = node1.val + carry;
                    let mut result = ListNode::new(sum % 10);
                    result.next = add(node1.next, None, sum / 10);
                    Some(Box::new(result))
                }
                (None, Some(node2)) => {
                    let sum = node2.val + carry;
                    let mut result = ListNode::new(sum % 10);
                    result.next = add(None, node2.next, sum / 10);
                    Some(Box::new(result))
                }
                (None, None) => match carry {
                    1 => Some(Box::new(ListNode::new(carry))),
                    _ => None,
                },
            }
        }

        add(l1, l2, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2() {
        let mut l1_1 = ListNode::new(2);
        let mut l1_2 = ListNode::new(4);
        let l1_3 = ListNode::new(3);
        l1_2.next = Some(Box::new(l1_3));
        l1_1.next = Some(Box::new(l1_2));

        let mut l2_1 = ListNode::new(5);
        let mut l2_2 = ListNode::new(6);
        let l2_3 = ListNode::new(4);
        l2_2.next = Some(Box::new(l2_3));
        l2_1.next = Some(Box::new(l2_2));

        let result = Solution::add_two_numbers(Some(Box::new(l1_1)), Some(Box::new(l2_1))).unwrap();

        let mut l3_1 = ListNode::new(7);
        let mut l3_2 = ListNode::new(0);
        let l3_3 = ListNode::new(8);
        l3_2.next = Some(Box::new(l3_3));
        l3_1.next = Some(Box::new(l3_2));

        assert_eq!(l3_1, *result);

        let l4_1 = ListNode::new(5);
        let l5_1 = ListNode::new(5);

        let result2 =
            Solution::add_two_numbers(Some(Box::new(l4_1)), Some(Box::new(l5_1))).unwrap();

        let mut l6_1 = ListNode::new(0);
        let l6_2 = ListNode::new(1);
        l6_1.next = Some(Box::new(l6_2));

        assert_eq!(l6_1, *result2);
    }
}
