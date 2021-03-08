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
    pub fn split_list_to_parts(
        mut root: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        fn recurse(
            root: &mut Option<Box<ListNode>>,
            k: i32,
            d: i32,
            result: &mut Vec<Option<Box<ListNode>>>,
        ) -> Vec<i32> {
            if let Some(node) = root {
                let mut counts = recurse(&mut node.next, k, d + 1, result);
                if counts.last().unwrap() == &0 {
                    result[counts.len() - 1] = node.next.take();
                    counts.pop();
                }
                *counts.last_mut().unwrap() -= 1;
                counts
            } else {
                let mut v = vec![d / k; std::cmp::min(k, d) as usize];
                for i in 0..(d % k) {
                    v[i as usize] += 1;
                }
                v
            }
        }

        let mut result = vec![None; k as usize];
        recurse(&mut root, k, 0, &mut result);
        result[0] = root;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0725() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let t1_3 = ListNode::new(3);
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert_eq!(
            Solution::split_list_to_parts(Some(Box::new(t1_1)), 5),
            vec![
                Some(Box::new(ListNode::new(1))),
                Some(Box::new(ListNode::new(2))),
                Some(Box::new(ListNode::new(3))),
                None,
                None
            ]
        );

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(3);
        let mut t2_4 = ListNode::new(4);
        let mut t2_5 = ListNode::new(5);
        let mut t2_6 = ListNode::new(6);
        let mut t2_7 = ListNode::new(7);
        let mut t2_8 = ListNode::new(8);
        let mut t2_9 = ListNode::new(9);
        let t2_10 = ListNode::new(10);
        t2_9.next = Some(Box::new(t2_10));
        t2_8.next = Some(Box::new(t2_9));
        t2_7.next = Some(Box::new(t2_8));
        t2_6.next = Some(Box::new(t2_7));
        t2_5.next = Some(Box::new(t2_6));
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r1_1 = ListNode::new(1);
        let mut r1_2 = ListNode::new(2);
        let mut r1_3 = ListNode::new(3);
        let r1_4 = ListNode::new(4);
        r1_3.next = Some(Box::new(r1_4));
        r1_2.next = Some(Box::new(r1_3));
        r1_1.next = Some(Box::new(r1_2));

        let mut r2_1 = ListNode::new(5);
        let mut r2_2 = ListNode::new(6);
        let r2_3 = ListNode::new(7);
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        let mut r3_1 = ListNode::new(8);
        let mut r3_2 = ListNode::new(9);
        let r3_3 = ListNode::new(10);
        r3_2.next = Some(Box::new(r3_3));
        r3_1.next = Some(Box::new(r3_2));

        assert_eq!(
            Solution::split_list_to_parts(Some(Box::new(t2_1)), 3),
            vec![
                Some(Box::new(r1_1)),
                Some(Box::new(r2_1)),
                Some(Box::new(r3_1))
            ]
        )
    }
}
