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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = Vec::new();
        if root.is_none() {
            return results;
        }
        let mut nodes = vec![root.unwrap()];
        while let Some(node) = nodes.pop() {
            let n = node.borrow();
            results.push(n.val);
            if let Some(left) = n.left.clone() {
                nodes.push(left);
            }
            if let Some(right) = n.right.clone() {
                nodes.push(right);
            }
        }
        results.reverse();
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0145() {
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
            vec![4, 5, 2, 6, 7, 3, 1],
            Solution::postorder_traversal(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
