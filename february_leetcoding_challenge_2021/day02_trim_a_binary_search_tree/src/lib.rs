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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(
            root: Option<Rc<RefCell<TreeNode>>>,
            low: i32,
            high: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                if n.val < low {
                    recurse(n.right.take(), low, high)
                } else if high < n.val {
                    recurse(n.left.take(), low, high)
                } else {
                    n.left = recurse(n.left.take(), low, high);
                    n.right = recurse(n.right.take(), low, high);
                    std::mem::drop(n);
                    Some(node)
                }
            } else {
                None
            }
        }

        recurse(root, low, high)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day02() {
        let mut t1_1 = TreeNode::new(1);
        let t1_2 = TreeNode::new(0);
        let t1_3 = TreeNode::new(2);
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut r1_1 = TreeNode::new(1);
        let r1_2 = TreeNode::new(2);
        r1_1.right = Some(Rc::new(RefCell::new(r1_2)));

        assert_eq!(
            Solution::trim_bst(Some(Rc::new(RefCell::new(t1_1))), 1, 2),
            Some(Rc::new(RefCell::new(r1_1)))
        );

        let mut t2_1 = TreeNode::new(3);
        let mut t2_2 = TreeNode::new(0);
        let t2_3 = TreeNode::new(4);
        let mut t2_4 = TreeNode::new(2);
        let t2_5 = TreeNode::new(1);
        t2_4.left = Some(Rc::new(RefCell::new(t2_5)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_4)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        let mut r2_1 = TreeNode::new(3);
        let mut r2_2 = TreeNode::new(2);
        let r2_3 = TreeNode::new(1);
        r2_2.left = Some(Rc::new(RefCell::new(r2_3)));
        r2_1.left = Some(Rc::new(RefCell::new(r2_2)));

        assert_eq!(
            Solution::trim_bst(Some(Rc::new(RefCell::new(t2_1))), 1, 3),
            Some(Rc::new(RefCell::new(r2_1)))
        );

        assert_eq!(
            Solution::trim_bst(Some(Rc::new(RefCell::new(TreeNode::new(1)))), 1, 2),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        );
    }
}
