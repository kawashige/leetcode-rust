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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            Default::default()
        } else if n == 1 {
            vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]
        } else {
            let mut result = Vec::new();
            for i in (1..=(n - 2)).step_by(2) {
                for left in &Self::all_possible_fbt(i) {
                    for right in &Self::all_possible_fbt(n - 1 - i) {
                        let mut node = TreeNode::new(0);
                        node.left = left.clone();
                        node.right = right.clone();
                        result.push(Some(Rc::new(RefCell::new(node))));
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0894() {
        assert_eq!(Solution::all_possible_fbt(2), vec![]);
    }
}
