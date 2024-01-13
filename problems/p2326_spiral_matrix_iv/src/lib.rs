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
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![-1; n as usize]; m as usize];

        let mut node = head;
        let mut pos = (0, 0);
        let direction = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut d = 0;

        while let Some(n) = node {
            matrix[pos.0 as usize][pos.1 as usize] = n.val;
            if !(0..matrix.len() as i32).contains(&(pos.0 + direction[d].0))
                || !(0..matrix[0].len() as i32).contains(&(pos.1 + direction[d].1))
                || matrix[(pos.0 + direction[d].0) as usize][(pos.1 + direction[d].1) as usize]
                    != -1
            {
                d = (d + 1) % direction.len();
            }
            pos = (pos.0 + direction[d].0, pos.1 + direction[d].1);
            node = n.next;
        }

        matrix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2326() {
        let mut t1_1 = ListNode::new(3);
        let mut t1_2 = ListNode::new(0);
        let mut t1_3 = ListNode::new(2);
        let mut t1_4 = ListNode::new(6);
        let mut t1_5 = ListNode::new(8);
        let mut t1_6 = ListNode::new(1);
        let mut t1_7 = ListNode::new(7);
        let mut t1_8 = ListNode::new(9);
        let mut t1_9 = ListNode::new(4);
        let mut t1_10 = ListNode::new(2);
        let mut t1_11 = ListNode::new(5);
        let mut t1_12 = ListNode::new(5);
        let t1_13 = ListNode::new(0);
        t1_12.next = Some(Box::new(t1_13));
        t1_11.next = Some(Box::new(t1_12));
        t1_10.next = Some(Box::new(t1_11));
        t1_9.next = Some(Box::new(t1_10));
        t1_8.next = Some(Box::new(t1_9));
        t1_7.next = Some(Box::new(t1_8));
        t1_6.next = Some(Box::new(t1_7));
        t1_5.next = Some(Box::new(t1_6));
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert_eq!(
            Solution::spiral_matrix(3, 5, Some(Box::new(t1_1))),
            vec![
                vec![3, 0, 2, 6, 8],
                vec![5, 0, -1, -1, 1],
                vec![5, 2, 4, 9, 7]
            ]
        );

        let mut t1_1 = ListNode::new(0);
        let mut t1_2 = ListNode::new(1);
        let t1_3 = ListNode::new(2);
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert_eq!(
            Solution::spiral_matrix(1, 4, Some(Box::new(t1_1))),
            vec![vec![0, 1, 2, -1]]
        );
    }
}
