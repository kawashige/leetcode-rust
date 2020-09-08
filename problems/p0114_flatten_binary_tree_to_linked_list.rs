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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn recurse(
            root: &mut Option<Rc<RefCell<TreeNode>>>,
            tails: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) {
            match root {
                Some(n) => {
                    let mut n = n.borrow_mut();
                    if n.right.is_some() && n.left.is_some() {
                        tails.push(n.right.clone());
                        n.right = n.left.clone();
                        n.left = None;
                        recurse(&mut n.right, tails);
                    } else if n.right.is_some() {
                        recurse(&mut n.right, tails);
                    } else if n.left.is_some() {
                        n.right = n.left.clone();
                        n.left = None;
                        recurse(&mut n.right, tails);
                    } else if tails.len() > 0 {
                        n.right = tails.pop().unwrap();
                        recurse(&mut n.right, tails);
                    }
                }
                None => {}
            }
        }
        recurse(root, &mut Vec::new());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0114() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(5);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(4);
        let t6 = TreeNode::new(6);
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
        let r6 = TreeNode::new(6);
        r5.right = Some(Rc::new(RefCell::new(r6)));
        r4.right = Some(Rc::new(RefCell::new(r5)));
        r3.right = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r3)));
        r1.right = Some(Rc::new(RefCell::new(r2)));

        let mut input = Some(Rc::new(RefCell::new(t1)));
        Solution::flatten(&mut input);
        assert_eq!(Rc::new(RefCell::new(r1)), input.unwrap());
    }
}
