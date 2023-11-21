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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            n.val == n.left.clone().unwrap().borrow().val + n.right.clone().unwrap().borrow().val
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2236() {
        let mut t1 = TreeNode::new(10);
        let t2 = TreeNode::new(4);
        let t3 = TreeNode::new(6);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::check_tree(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(5);
        let t2 = TreeNode::new(3);
        let t3 = TreeNode::new(1);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::check_tree(Some(Rc::new(RefCell::new(t1)))));
    }
}
