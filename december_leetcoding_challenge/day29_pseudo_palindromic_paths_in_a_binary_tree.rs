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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, counter: i32) -> i32 {
            if let Some(node) = root {
                let n = node.borrow();
                let new_counter = counter ^ (1 << n.val);
                if n.left.is_none() && n.right.is_none() {
                    if 1 < new_counter.count_ones() {
                        0
                    } else {
                        1
                    }
                } else {
                    let mut result = 0;
                    if n.left.is_some() {
                        result += recurse(&n.left, new_counter);
                    }
                    if n.right.is_some() {
                        result += recurse(&n.right, new_counter);
                    }
                    result
                }
            } else {
                0
            }
        }

        recurse(&root, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day29() {
        let mut t1 = TreeNode::new(2);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(1);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            2,
            Solution::pseudo_palindromic_paths(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(2);
        let mut t2 = TreeNode::new(1);
        let t3 = TreeNode::new(1);
        let t4 = TreeNode::new(1);
        let mut t5 = TreeNode::new(3);
        let t6 = TreeNode::new(1);
        t5.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            1,
            Solution::pseudo_palindromic_paths(Some(Rc::new(RefCell::new(t1))))
        );

        assert_eq!(
            1,
            Solution::pseudo_palindromic_paths(Some(Rc::new(RefCell::new(TreeNode::new(9)))))
        );
    }
}
