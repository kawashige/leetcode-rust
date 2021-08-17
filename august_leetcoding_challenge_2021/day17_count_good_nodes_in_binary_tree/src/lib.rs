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
    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            Self::recurse(&n.left, max.max(n.val))
                + Self::recurse(&n.right, max.max(n.val))
                + if n.val >= max { 1 } else { 0 }
        } else {
            0
        }
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::recurse(&root, -10_000)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day17() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(4);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(5);
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(Solution::good_nodes(Some(Rc::new(RefCell::new(t1)))), 4);

        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(3);
        let t3 = TreeNode::new(4);
        let t4 = TreeNode::new(2);
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(Solution::good_nodes(Some(Rc::new(RefCell::new(t1)))), 3);

        assert_eq!(
            Solution::good_nodes(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            1
        );
    }
}
