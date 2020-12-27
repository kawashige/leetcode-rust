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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = root {
                let n = n.borrow();
                let mut t = TreeNode::new(n.val);
                t.right = if n.right.is_some() {
                    recurse(&n.right, right)
                } else {
                    right
                };
                if n.left.is_some() {
                    return recurse(&n.left, Some(Rc::new(RefCell::new(t))));
                } else {
                    return Some(Rc::new(RefCell::new(t)));
                }
            }
            None
        }
        recurse(&root, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day3() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(6);
        let mut t4 = TreeNode::new(2);
        let t5 = TreeNode::new(4);
        let mut t6 = TreeNode::new(8);
        let t7 = TreeNode::new(1);
        let t8 = TreeNode::new(7);
        let t9 = TreeNode::new(9);
        t6.left = Some(Rc::new(RefCell::new(t8)));
        t6.right = Some(Rc::new(RefCell::new(t9)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(2);
        let mut r3 = TreeNode::new(3);
        let mut r4 = TreeNode::new(4);
        let mut r5 = TreeNode::new(5);
        let mut r6 = TreeNode::new(6);
        let mut r7 = TreeNode::new(7);
        let mut r8 = TreeNode::new(8);
        let r9 = TreeNode::new(9);
        r8.right = Some(Rc::new(RefCell::new(r9)));
        r7.right = Some(Rc::new(RefCell::new(r8)));
        r6.right = Some(Rc::new(RefCell::new(r7)));
        r5.right = Some(Rc::new(RefCell::new(r6)));
        r4.right = Some(Rc::new(RefCell::new(r5)));
        r3.right = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r3)));
        r1.right = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Some(Rc::new(RefCell::new(r1))),
            Solution::increasing_bst(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(5);
        let t2 = TreeNode::new(1);
        let t3 = TreeNode::new(7);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(5);
        let r3 = TreeNode::new(7);
        r2.right = Some(Rc::new(RefCell::new(r3)));
        r1.right = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Some(Rc::new(RefCell::new(r1))),
            Solution::increasing_bst(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
