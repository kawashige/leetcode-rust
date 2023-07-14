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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut critical_points = Vec::new();
        let mut min_distance = std::i32::MAX;
        let mut values = [0; 3];

        let mut node = head;
        let mut i = 0;
        while let Some(n) = node {
            values[0] = values[1];
            values[1] = values[2];
            values[2] = n.val;

            if 1 < i
                && ((values[0] < values[1] && values[2] < values[1])
                    || (values[1] < values[0] && values[1] < values[2]))
            {
                if 0 < critical_points.len() {
                    min_distance = min_distance.min(i - critical_points[critical_points.len() - 1]);
                }
                critical_points.push(i);
            }
            i += 1;
            node = n.next;
        }

        if critical_points.len() < 2 {
            vec![-1, -1]
        } else {
            vec![
                min_distance,
                critical_points[critical_points.len() - 1] - critical_points[0],
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2058() {
        let mut t1_1 = ListNode::new(1);
        let t1_2 = ListNode::new(2);
        t1_1.next = Some(Box::new(t1_2));
        assert_eq!(
            Solution::nodes_between_critical_points(Some(Box::new(t1_1))),
            vec![-1, -1]
        );
        let mut t2_1 = ListNode::new(5);
        let mut t2_2 = ListNode::new(3);
        let mut t2_3 = ListNode::new(1);
        let mut t2_4 = ListNode::new(2);
        let mut t2_5 = ListNode::new(5);
        let mut t2_6 = ListNode::new(1);
        let t2_7 = ListNode::new(2);
        t2_6.next = Some(Box::new(t2_7));
        t2_5.next = Some(Box::new(t2_6));
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));
        assert_eq!(
            Solution::nodes_between_critical_points(Some(Box::new(t2_1))),
            vec![1, 3]
        );
        let mut t3_1 = ListNode::new(1);
        let mut t3_2 = ListNode::new(3);
        let mut t3_3 = ListNode::new(2);
        let mut t3_4 = ListNode::new(2);
        let mut t3_5 = ListNode::new(3);
        let mut t3_6 = ListNode::new(2);
        let mut t3_7 = ListNode::new(2);
        let mut t3_8 = ListNode::new(2);
        let t3_9 = ListNode::new(7);
        t3_8.next = Some(Box::new(t3_9));
        t3_7.next = Some(Box::new(t3_8));
        t3_6.next = Some(Box::new(t3_7));
        t3_5.next = Some(Box::new(t3_6));
        t3_4.next = Some(Box::new(t3_5));
        t3_3.next = Some(Box::new(t3_4));
        t3_2.next = Some(Box::new(t3_3));
        t3_1.next = Some(Box::new(t3_2));
        assert_eq!(
            Solution::nodes_between_critical_points(Some(Box::new(t3_1))),
            vec![3, 3]
        );
    }
}
