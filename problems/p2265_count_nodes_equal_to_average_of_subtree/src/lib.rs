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
    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(node) = root {
            let n = node.borrow();
            let mut count = 1;
            let mut sum = n.val;
            let mut nodes = 0;
            if n.left.is_some() {
                let left = Self::recurse(&n.left);
                count += left.0;
                sum += left.1;
                nodes += left.2;
            }
            if n.right.is_some() {
                let right = Self::recurse(&n.right);
                count += right.0;
                sum += right.1;
                nodes += right.2;
            }
            (count, sum, nodes + if sum / count == n.val { 1 } else { 0 })
        } else {
            (0, 0, 0)
        }
    }
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::recurse(&root).2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2265() {
        let mut t1 = TreeNode::new(4);
        let mut t2 = TreeNode::new(8);
        let mut t3 = TreeNode::new(5);
        let t4 = TreeNode::new(0);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(6);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::average_of_subtree(Some(Rc::new(RefCell::new(t1)))),
            5
        );

        assert_eq!(
            Solution::average_of_subtree(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            1
        );
    }
}
