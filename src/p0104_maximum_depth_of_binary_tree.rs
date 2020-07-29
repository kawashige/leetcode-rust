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

pub struct Solution {}

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn find_max_depth(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            match root {
                Some(n) => {
                    let node = Rc::try_unwrap(n).unwrap().into_inner();
                    let left = find_max_depth(node.left, depth + 1);
                    let right = find_max_depth(node.right, depth + 1);
                    cmp::max(left, right)
                }
                _ => depth,
            }
        }

        find_max_depth(root, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0104() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(3, Solution::max_depth(Some(Rc::new(RefCell::new(t1)))));
    }
}
