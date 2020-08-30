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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            match root {
                Some(n) => {
                    let n_inner = Rc::try_unwrap(n).unwrap().into_inner();
                    (min.is_none() || min.unwrap() < n_inner.val)
                        && (max.is_none() || n_inner.val < max.unwrap())
                        && check(n_inner.left, min, Some(n_inner.val))
                        && check(n_inner.right, Some(n_inner.val), max)
                }
                None => true,
            }
        }
        check(root, None, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0098() {
        let mut t1_1 = TreeNode::new(2);
        let t1_2 = TreeNode::new(1);
        let t1_3 = TreeNode::new(3);
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(t1_1)))));

        let mut t2_1 = TreeNode::new(5);
        let t2_2 = TreeNode::new(1);
        let mut t2_3 = TreeNode::new(4);
        let t2_4 = TreeNode::new(3);
        let t2_5 = TreeNode::new(6);
        t2_3.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_3.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert!(!Solution::is_valid_bst(Some(Rc::new(RefCell::new(t2_1)))));
        assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(
            TreeNode::new(2147483647)
        )))));
    }
}
