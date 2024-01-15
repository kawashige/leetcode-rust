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
    pub fn recurse(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = node {
            let n = node.borrow();
            match n.val {
                0 => false,
                1 => true,
                2 => Self::recurse(&n.left) || Self::recurse(&n.right),
                3 => Self::recurse(&n.left) && Self::recurse(&n.right),
                _ => unreachable!(),
            }
        } else {
            false
        }
    }
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::recurse(&root)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2331() {
        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::evaluate_tree(Some(Rc::new(RefCell::new(t1)))));

        let t1 = TreeNode::new(0);
        assert!(!Solution::evaluate_tree(Some(Rc::new(RefCell::new(t1)))));
    }
}
