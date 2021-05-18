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
    pub fn recurse(root: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            let left = Self::recurse(&mut n.left);
            let right = Self::recurse(&mut n.right);
            if !left {
                n.left = None;
            }
            if !right {
                n.right = None;
            }
            left || right || n.val == 1
        } else {
            false
        }
    }

    pub fn prune_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::recurse(&mut root) {
            root
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0814() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(0);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(0);
        let t5 = TreeNode::new(0);
        let t6 = TreeNode::new(0);
        let t7 = TreeNode::new(1);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(1);
        let r3 = TreeNode::new(1);
        r2.right = Some(Rc::new(RefCell::new(r3)));
        r1.right = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Solution::prune_tree(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(0);
        let mut t4 = TreeNode::new(1);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(0);
        let t7 = TreeNode::new(1);
        let t8 = TreeNode::new(0);
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(1);
        let mut r3 = TreeNode::new(0);
        let r4 = TreeNode::new(1);
        let r5 = TreeNode::new(1);
        let r6 = TreeNode::new(1);
        r3.right = Some(Rc::new(RefCell::new(r6)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::prune_tree(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );
    }
}
