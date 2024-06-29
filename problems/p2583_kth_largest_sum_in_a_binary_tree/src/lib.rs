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
    pub fn recurse(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, level_sum: &mut Vec<i64>) {
        if let Some(node) = node {
            let node = node.borrow();
            if level_sum.len() <= depth {
                level_sum.push(node.val as i64);
            } else {
                level_sum[depth] += node.val as i64;
            }
            Self::recurse(&node.left, depth + 1, level_sum);
            Self::recurse(&node.right, depth + 1, level_sum);
        }
    }

    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut level_sum = Vec::new();
        Self::recurse(&root, 0, &mut level_sum);
        if level_sum.len() < k as usize {
            return -1;
        }
        level_sum.sort_unstable_by(|a, b| b.cmp(&a));
        level_sum[k as usize - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2580() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(8);
        let mut t3 = TreeNode::new(9);
        let mut t4 = TreeNode::new(2);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(3);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(4);
        let t9 = TreeNode::new(6);
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t4.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::kth_largest_level_sum(Some(Rc::new(RefCell::new(t1))), 2),
            13
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::kth_largest_level_sum(Some(Rc::new(RefCell::new(t1))), 1),
            3
        );
    }
}
