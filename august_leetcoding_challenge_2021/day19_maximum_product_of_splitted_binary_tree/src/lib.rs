// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}

impl Solution {
    pub fn subtree_sum(root: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i32>) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            let l = Self::subtree_sum(&n.left, sums);
            let r = Self::subtree_sum(&n.right, sums);
            sums.push(n.val + l + r);
            *sums.last().unwrap()
        } else {
            0
        }
    }

    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums = Vec::new();

        Self::subtree_sum(&root, &mut sums);

        ((0..(sums.len() - 1))
            .map(|i| (sums.last().unwrap() - sums[i]) as u64 * sums[i] as u64)
            .max()
            .unwrap()
            % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day19() {
        let mut t1_1 = TreeNode::new(1);
        let mut t1_2 = TreeNode::new(2);
        let mut t1_3 = TreeNode::new(3);
        let t1_4 = TreeNode::new(4);
        let t1_5 = TreeNode::new(5);
        let t1_6 = TreeNode::new(6);
        t1_3.left = Some(Rc::new(RefCell::new(t1_6)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert_eq!(
            Solution::max_product(Some(Rc::new(RefCell::new(t1_1)))),
            110
        );

        let mut t2_1 = TreeNode::new(1);
        let mut t2_2 = TreeNode::new(2);
        let t2_3 = TreeNode::new(3);
        let mut t2_4 = TreeNode::new(4);
        let t2_5 = TreeNode::new(5);
        let t2_6 = TreeNode::new(6);
        t2_4.left = Some(Rc::new(RefCell::new(t2_5)));
        t2_4.right = Some(Rc::new(RefCell::new(t2_6)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_3)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_4)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_2)));

        assert_eq!(Solution::max_product(Some(Rc::new(RefCell::new(t2_1)))), 90);

        let mut t3_1 = TreeNode::new(1);
        let t3_2 = TreeNode::new(1);
        t3_1.right = Some(Rc::new(RefCell::new(t3_2)));

        assert_eq!(Solution::max_product(Some(Rc::new(RefCell::new(t3_1)))), 1);
    }
}
