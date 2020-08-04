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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn traverse(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>, d: usize) {
            match node {
                Some(n) => {
                    if d < result.len() {
                        result[d].push(n.borrow().val);
                    } else {
                        result.push(vec![n.borrow().val]);
                    }
                    let n_inner = Rc::try_unwrap(n).ok().unwrap().into_inner();
                    traverse(n_inner.left, result, d + 1);
                    traverse(n_inner.right, result, d + 1);
                }
                None => {
                    // nothing to do
                }
            }
        }

        let mut result = Vec::new();
        traverse(root, &mut result, 0);
        result.reverse();
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0107() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec![vec![15, 7], vec![9, 20], vec![3]],
            Solution::level_order_bottom(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
