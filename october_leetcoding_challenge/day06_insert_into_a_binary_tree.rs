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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
            match root {
                Some(n) => {
                    let mut n = n.borrow_mut();
                    if n.val < val {
                        if n.right.is_none() {
                            n.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        } else {
                            recurse(&mut n.right, val);
                        }
                    } else {
                        if n.left.is_none() {
                            n.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        } else {
                            recurse(&mut n.left, val);
                        }
                    }
                }
                None => {}
            }
        }

        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        let mut dummy = TreeNode::new(-1);
        dummy.right = root;
        recurse(&mut dummy.right, val);
        dummy.right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day6() {
        let mut t1 = TreeNode::new(4);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(7);
        let t4 = TreeNode::new(1);
        let t5 = TreeNode::new(3);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(4);
        let mut r2 = TreeNode::new(2);
        let mut r3 = TreeNode::new(7);
        let r4 = TreeNode::new(1);
        let r5 = TreeNode::new(3);
        let r6 = TreeNode::new(5);
        r3.left = Some(Rc::new(RefCell::new(r6)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Rc::new(RefCell::new(r1)),
            Solution::insert_into_bst(Some(Rc::new(RefCell::new(t1))), 5).unwrap()
        );

        assert_eq!(
            Rc::new(RefCell::new(TreeNode::new(5))),
            Solution::insert_into_bst(None, 5).unwrap()
        );
    }
}
