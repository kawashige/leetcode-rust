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
    pub fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            if Self::traverse(&n.left, target) {
                n.left = None;
            }
            if Self::traverse(&n.right, target) {
                n.right = None;
            }

            n.left.is_none() && n.right.is_none() && n.val == target
        } else {
            false
        }
    }

    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::traverse(&root, target) {
            None
        } else {
            root
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1325() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(2);
        let t5 = TreeNode::new(2);
        let t6 = TreeNode::new(4);
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(3);
        let r3 = TreeNode::new(4);
        r2.right = Some(Rc::new(RefCell::new(r3)));
        r1.right = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Solution::remove_leaf_nodes(Some(Rc::new(RefCell::new(t1))), 2),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(3);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(2);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(3);
        let r3 = TreeNode::new(2);
        r2.right = Some(Rc::new(RefCell::new(r3)));
        r1.left = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Solution::remove_leaf_nodes(Some(Rc::new(RefCell::new(t1))), 3),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(2);
        let t4 = TreeNode::new(2);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        let r1 = TreeNode::new(1);

        assert_eq!(
            Solution::remove_leaf_nodes(Some(Rc::new(RefCell::new(t1))), 2),
            Some(Rc::new(RefCell::new(r1)))
        );
    }
}
