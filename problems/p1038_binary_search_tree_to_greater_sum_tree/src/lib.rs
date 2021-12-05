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
    pub fn convert(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            Self::convert(&n.right, sum);
            n.val += *sum;
            *sum = n.val;
            Self::convert(&n.left, sum);
        }
    }

    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::convert(&root, &mut 0);
        root
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1038() {
        let mut t1 = TreeNode::new(4);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(6);
        let t4 = TreeNode::new(0);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(5);
        let mut t7 = TreeNode::new(7);
        let t8 = TreeNode::new(3);
        let t9 = TreeNode::new(8);
        t7.right = Some(Rc::new(RefCell::new(t9)));
        t5.right = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(30);
        let mut r2 = TreeNode::new(36);
        let mut r3 = TreeNode::new(21);
        let r4 = TreeNode::new(36);
        let mut r5 = TreeNode::new(35);
        let r6 = TreeNode::new(26);
        let mut r7 = TreeNode::new(15);
        let r8 = TreeNode::new(33);
        let r9 = TreeNode::new(8);
        r7.right = Some(Rc::new(RefCell::new(r9)));
        r5.right = Some(Rc::new(RefCell::new(r8)));
        r3.left = Some(Rc::new(RefCell::new(r6)));
        r3.right = Some(Rc::new(RefCell::new(r7)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::bst_to_gst(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(0);
        let t2 = TreeNode::new(1);
        t1.right = Some(Rc::new(RefCell::new(t2)));

        let mut r1 = TreeNode::new(1);
        let r2 = TreeNode::new(1);
        r1.right = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Solution::bst_to_gst(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );
    }
}
