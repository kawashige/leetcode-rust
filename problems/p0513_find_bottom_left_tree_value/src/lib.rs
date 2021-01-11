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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize) -> (usize, i32) {
            if let Some(node) = root {
                let n = node.borrow();
                let mut result = (depth, n.val);
                let left = recurse(&n.left, depth + 1);
                if result.0 < left.0 {
                    result = left;
                }
                let right = recurse(&n.right, depth + 1);
                if result.0 < right.0 {
                    result = right;
                }
                result
            } else {
                (0, 0)
            }
        }
        recurse(&root, 0).1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0513() {
        let mut t1 = TreeNode::new(2);
        let t2 = TreeNode::new(1);
        let t3 = TreeNode::new(3);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            1,
            Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let mut t5 = TreeNode::new(5);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        t5.left = Some(Rc::new(RefCell::new(t7)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            7,
            Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
