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
    fn collect(head: &Option<Box<ListNode>>, arr: &mut Vec<i32>) {
        if let Some(node) = head {
            if node.val != 0 {
                arr.push(node.val);
            }
            Self::collect(&node.next, arr);
        }
    }

    fn buid(arr: &mut Vec<i32>) -> Option<Box<ListNode>> {
        if let Some(x) = arr.pop() {
            Some(Box::new(ListNode {
                next: Self::buid(arr),
                val: x,
            }))
        } else {
            None
        }
    }

    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr = Vec::new();
        Self::collect(&head, &mut arr);

        if arr.is_empty() {
            return None;
        }

        let mut acc = vec![arr[0]];
        let mut new_arr = vec![arr[0]];

        for i in 1..arr.len() {
            let sum = acc.last().unwrap_or(&0) + arr[i];
            if sum == 0 {
                acc.clear();
                new_arr.clear();
            } else {
                if let Some(j) = (0..acc.len()).find(|j| acc[*j] == sum) {
                    acc.truncate(j + 1);
                    new_arr.truncate(j + 1);
                } else {
                    acc.push(sum);
                    new_arr.push(arr[i]);
                }
            }
        }

        let mut new_arr = new_arr.into_iter().rev().collect();
        Self::buid(&mut new_arr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1171() {
        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(2);
        let mut t1_3 = ListNode::new(-3);
        let mut t1_4 = ListNode::new(3);
        let t1_5 = ListNode::new(1);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        let mut r1_1 = ListNode::new(3);
        let r1_2 = ListNode::new(1);
        r1_1.next = Some(Box::new(r1_2));

        let result = Solution::remove_zero_sum_sublists(Some(Box::new(t1_1)));
        assert_eq!(result, Some(Box::new(r1_1)));

        let mut t2_1 = ListNode::new(1);
        let mut t2_2 = ListNode::new(2);
        let mut t2_3 = ListNode::new(3);
        let mut t2_4 = ListNode::new(-3);
        let t2_5 = ListNode::new(4);
        t2_4.next = Some(Box::new(t2_5));
        t2_3.next = Some(Box::new(t2_4));
        t2_2.next = Some(Box::new(t2_3));
        t2_1.next = Some(Box::new(t2_2));

        let mut r2_1 = ListNode::new(1);
        let mut r2_2 = ListNode::new(2);
        let r2_3 = ListNode::new(4);
        r2_2.next = Some(Box::new(r2_3));
        r2_1.next = Some(Box::new(r2_2));

        let result = Solution::remove_zero_sum_sublists(Some(Box::new(t2_1)));
        assert_eq!(result, Some(Box::new(r2_1)));

        let mut t3_1 = ListNode::new(1);
        let mut t3_2 = ListNode::new(2);
        let mut t3_3 = ListNode::new(3);
        let mut t3_4 = ListNode::new(-3);
        let t3_5 = ListNode::new(-2);
        t3_4.next = Some(Box::new(t3_5));
        t3_3.next = Some(Box::new(t3_4));
        t3_2.next = Some(Box::new(t3_3));
        t3_1.next = Some(Box::new(t3_2));

        let r3_1 = ListNode::new(1);

        let result = Solution::remove_zero_sum_sublists(Some(Box::new(t3_1)));
        assert_eq!(result, Some(Box::new(r3_1)));

        let t4_1 = ListNode::new(0);
        let result = Solution::remove_zero_sum_sublists(Some(Box::new(t4_1)));
        assert_eq!(result, None);
    }
}
