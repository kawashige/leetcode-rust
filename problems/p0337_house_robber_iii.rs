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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(n) = root {
                let n = n.borrow();
                let left = recurse(&n.left);
                let right = recurse(&n.right);
                (
                    std::cmp::max(left.0, left.1) + std::cmp::max(right.0, right.1),
                    n.val + left.0 + right.0,
                )
            } else {
                (0, 0)
            }
        }

        let (include, exclude) = recurse(&root);
        std::cmp::max(include, exclude)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0337() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(1);
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(7, Solution::rob(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(5);
        let t4 = TreeNode::new(1);
        let t5 = TreeNode::new(3);
        let t6 = TreeNode::new(1);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t5)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(9, Solution::rob(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(10);
        let mut t2 = TreeNode::new(1);
        let t3 = TreeNode::new(1);
        let t4 = TreeNode::new(10);
        let mut t5 = TreeNode::new(1);
        let t6 = TreeNode::new(10);
        let mut t7 = TreeNode::new(1);
        let t8 = TreeNode::new(10);
        t7.right = Some(Rc::new(RefCell::new(t8)));
        t5.left = Some(Rc::new(RefCell::new(t6)));
        t5.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(40, Solution::rob(Some(Rc::new(RefCell::new(t1)))));
    }
}
