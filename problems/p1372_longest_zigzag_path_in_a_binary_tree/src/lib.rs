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
    fn longest_path(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(node) = root {
            let n = node.borrow();
            let (l_max, _, r) = Self::longest_path(&n.left);
            let (r_max, l, _) = Self::longest_path(&n.right);
            (l_max.max(r_max).max(r + 1).max(l + 1), r + 1, l + 1)
        } else {
            (0, -1, -1)
        }
    }
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::longest_path(&root).0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1372() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let mut t4 = TreeNode::new(4);
        let mut t5 = TreeNode::new(5);
        let t6 = TreeNode::new(6);
        let mut t7 = TreeNode::new(7);
        let t8 = TreeNode::new(8);
        t7.right = Some(Rc::new(RefCell::new(t8)));
        t5.right = Some(Rc::new(RefCell::new(t7)));
        t4.left = Some(Rc::new(RefCell::new(t5)));
        t4.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::longest_zig_zag(Some(Rc::new(RefCell::new(t1)))),
            3
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(1);
        let t3 = TreeNode::new(1);
        let mut t4 = TreeNode::new(1);
        let mut t5 = TreeNode::new(1);
        let t6 = TreeNode::new(1);
        let t7 = TreeNode::new(1);
        t5.right = Some(Rc::new(RefCell::new(t7)));
        t4.left = Some(Rc::new(RefCell::new(t5)));
        t4.right = Some(Rc::new(RefCell::new(t6)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::longest_zig_zag(Some(Rc::new(RefCell::new(t1)))),
            4
        );

        assert_eq!(
            Solution::longest_zig_zag(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            0
        );
    }
}
