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
    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            let mut sum = n.val;
            let left = Self::recurse(&n.left, max);
            let right = Self::recurse(&n.right, max);
            sum += left.max(0) + right.max(0);
            if *max < sum {
                *max = sum;
            }
            n.val + left.max(right).max(0)
        } else {
            0
        }
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = std::i32::MIN;
        Self::recurse(&root, &mut max);
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0124() {
        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        assert_eq!(Solution::max_path_sum(Some(Rc::new(RefCell::new(t1)))), 6);

        let mut t1 = TreeNode::new(-10);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        assert_eq!(Solution::max_path_sum(Some(Rc::new(RefCell::new(t1)))), 42);

        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(8);
        let mut t4 = TreeNode::new(11);
        let t5 = TreeNode::new(13);
        let mut t6 = TreeNode::new(4);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(2);
        let t9 = TreeNode::new(1);
        t6.right = Some(Rc::new(RefCell::new(t9)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        assert_eq!(Solution::max_path_sum(Some(Rc::new(RefCell::new(t1)))), 48);
    }
}
