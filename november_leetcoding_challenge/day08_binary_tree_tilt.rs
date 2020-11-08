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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let left = recurse(&n.left);
                    let right = recurse(&n.right);
                    (
                        left.0 + right.0 + n.val,
                        left.1 + right.1 + (left.0 - right.0).abs(),
                    )
                }
                None => (0, 0),
            }
        }
        let result = recurse(&root);
        result.1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day8() {
        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(1, Solution::find_tilt(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(4);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(9);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(5);
        let t6 = TreeNode::new(7);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(15, Solution::find_tilt(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(21);
        let mut t2 = TreeNode::new(7);
        let mut t3 = TreeNode::new(14);
        let mut t4 = TreeNode::new(1);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(2);
        let t7 = TreeNode::new(2);
        let t8 = TreeNode::new(3);
        let t9 = TreeNode::new(3);
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t4.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(9, Solution::find_tilt(Some(Rc::new(RefCell::new(t1)))));
    }
}
