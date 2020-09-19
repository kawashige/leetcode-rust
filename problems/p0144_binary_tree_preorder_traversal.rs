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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = Vec::new();
        let mut nodes = vec![root];
        while let Some(node) = nodes.pop() {
            if let Some(n) = node {
                let n_inner = Rc::try_unwrap(n).unwrap().into_inner();
                results.push(n_inner.val);
                nodes.push(n_inner.right);
                nodes.push(n_inner.left);
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0144() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec![1, 2, 4, 5, 3, 6, 7],
            Solution::preorder_traversal(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
