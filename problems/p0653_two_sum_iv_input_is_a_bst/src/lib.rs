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
use std::rc::Rc;
use std::{cell::RefCell, collections::HashSet};
pub struct Solution {}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn recurse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            targets: &mut HashSet<i32>,
            k: i32,
        ) -> bool {
            if let Some(node) = root {
                let n = node.borrow();
                if recurse(&n.left, targets, k) {
                    return true;
                }
                if targets.contains(&n.val) {
                    return true;
                }
                targets.insert(k - n.val);
                recurse(&n.right, targets, k)
            } else {
                false
            }
        }
        recurse(&root, &mut HashSet::new(), k)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0653() {
        let mut t1_1 = TreeNode::new(5);
        let mut t1_2 = TreeNode::new(3);
        let mut t1_3 = TreeNode::new(6);
        let t1_4 = TreeNode::new(2);
        let t1_5 = TreeNode::new(4);
        let t1_6 = TreeNode::new(7);
        t1_3.right = Some(Rc::new(RefCell::new(t1_6)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert!(Solution::find_target(Some(Rc::new(RefCell::new(t1_1))), 9));

        let mut t2_1 = TreeNode::new(5);
        let mut t2_2 = TreeNode::new(3);
        let mut t2_3 = TreeNode::new(6);
        let t2_4 = TreeNode::new(2);
        let t2_5 = TreeNode::new(4);
        let t2_6 = TreeNode::new(7);
        t2_3.right = Some(Rc::new(RefCell::new(t2_6)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert!(!Solution::find_target(
            Some(Rc::new(RefCell::new(t2_1))),
            28
        ));
    }
}
