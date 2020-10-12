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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(root: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = root {
                let mut n = n.borrow_mut();
                let left = n.left.take();
                let right = n.right.take();
                n.left = right;
                n.right = left;
                recurse(&mut n.left);
                recurse(&mut n.right)
            }
        }

        let mut dummy = TreeNode::new(-1);
        dummy.left = root;
        recurse(&mut dummy.left);
        dummy.left
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0226() {
        let mut t1 = TreeNode::new(4);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(7);
        let t4 = TreeNode::new(1);
        let t5 = TreeNode::new(3);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(9);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(4);
        let mut r2 = TreeNode::new(7);
        let mut r3 = TreeNode::new(2);
        let r4 = TreeNode::new(9);
        let r5 = TreeNode::new(6);
        let r6 = TreeNode::new(3);
        let r7 = TreeNode::new(1);
        r3.left = Some(Rc::new(RefCell::new(r6)));
        r3.right = Some(Rc::new(RefCell::new(r7)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Rc::new(RefCell::new(r1)),
            Solution::invert_tree(Some(Rc::new(RefCell::new(t1)))).unwrap()
        );
    }
}
