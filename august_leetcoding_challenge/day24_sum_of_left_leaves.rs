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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn left_leaves_val(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            match node {
                Some(n) => {
                    let n_inner = Rc::try_unwrap(n).unwrap().into_inner();
                    if is_left && n_inner.left.is_none() && n_inner.right.is_none() {
                        n_inner.val
                    } else {
                        left_leaves_val(n_inner.left, true) + left_leaves_val(n_inner.right, false)
                    }
                }
                None => 0,
            }
        }
        left_leaves_val(root, false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day24() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            24,
            Solution::sum_of_left_leaves(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
