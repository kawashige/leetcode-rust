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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(root: Option<Rc<RefCell<TreeNode>>>, h: i32) -> Option<i32> {
            match root {
                Some(n) => {
                    let n_inner = Rc::try_unwrap(n).unwrap().into_inner();
                    match (check(n_inner.left, h + 1), check(n_inner.right, h + 1)) {
                        (Some(l), Some(r)) => {
                            if (l - r).abs() > 1 {
                                None
                            } else {
                                Some(std::cmp::max(l, r))
                            }
                        }
                        (_, _) => None,
                    }
                }
                None => Some(h),
            }
        }

        match check(root, 0) {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0110() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(t1)))));

        let mut t2_1 = TreeNode::new(1);
        let mut t2_2 = TreeNode::new(2);
        let t2_3 = TreeNode::new(2);
        let mut t2_4 = TreeNode::new(3);
        let t2_5 = TreeNode::new(3);
        let t2_6 = TreeNode::new(4);
        let t2_7 = TreeNode::new(4);
        t2_4.left = Some(Rc::new(RefCell::new(t2_6)));
        t2_4.right = Some(Rc::new(RefCell::new(t2_7)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert!(!Solution::is_balanced(Some(Rc::new(RefCell::new(t2_1)))));
    }
}
