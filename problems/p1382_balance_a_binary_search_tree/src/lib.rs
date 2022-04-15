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
    pub fn collect(root: &Option<Rc<RefCell<TreeNode>>>, array: &mut Vec<i32>) {
        if let Some(node) = root {
            let n = node.borrow();
            Self::collect(&n.left, array);
            array.push(n.val);
            Self::collect(&n.right, array);
        }
    }

    pub fn build(array: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if array.is_empty() {
            None
        } else {
            let i = array.len() / 2;
            let mut node = TreeNode::new(array[i]);
            node.left = Self::build(&array[..i]);
            node.right = Self::build(&array[(i + 1)..]);
            Some(Rc::new(RefCell::new(node)))
        }
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut array = Vec::new();
        Self::collect(&root, &mut array);
        Self::build(&array)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1382() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t3.right = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t3)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        let mut r1 = TreeNode::new(3);
        let mut r2 = TreeNode::new(2);
        let r3 = TreeNode::new(4);
        let r4 = TreeNode::new(1);
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::balance_bst(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(2);
        let t2 = TreeNode::new(1);
        let t3 = TreeNode::new(3);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(2);
        let r2 = TreeNode::new(1);
        let r3 = TreeNode::new(3);
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::balance_bst(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );
    }
}
