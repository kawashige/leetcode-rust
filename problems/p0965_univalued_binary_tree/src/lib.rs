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
    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            n.val == val && Self::recurse(&n.left, val) && Self::recurse(&n.right, val)
        } else {
            true
        }
    }

    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::recurse(&root, root.as_ref().unwrap().borrow().val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0965() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(1);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(1);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::is_unival_tree(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(1);
        let t3 = TreeNode::new(1);
        let t4 = TreeNode::new(5);
        let t5 = TreeNode::new(1);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::is_unival_tree(Some(Rc::new(RefCell::new(t1)))));
    }
}
