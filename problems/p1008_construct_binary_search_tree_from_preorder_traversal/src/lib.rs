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
    pub fn recurse(preorder: &mut Vec<i32>, min: i32, max: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if !preorder.is_empty() && (min + 1..max).contains(preorder.last().unwrap()) {
            let mut node = TreeNode::new(preorder.pop().unwrap());
            node.left = Self::recurse(preorder, min, node.val);
            node.right = Self::recurse(preorder, node.val, max);
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }

    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = preorder.into_iter().rev().collect();
        Self::recurse(&mut preorder, 0, 10000001)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1008() {
        let mut t1 = TreeNode::new(8);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(10);
        let t4 = TreeNode::new(1);
        let t5 = TreeNode::new(7);
        let t6 = TreeNode::new(12);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
            Some(Rc::new(RefCell::new(t1)))
        );

        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(3);
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::bst_from_preorder(vec![1, 3]),
            Some(Rc::new(RefCell::new(t1)))
        );
    }
}
