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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, l: usize, results: &mut Vec<Vec<i32>>) {
            match root {
                Some(n) => {
                    let n = Rc::try_unwrap(n).unwrap().into_inner();
                    if results.len() < l + 1 {
                        results.push(vec![n.val]);
                    } else {
                        results[l].push(n.val);
                    }
                    traverse(n.left, l + 1, results);
                    traverse(n.right, l + 1, results);
                }
                None => {}
            }
        }
        let mut results = Vec::new();
        traverse(root, 0, &mut results);
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0102() {
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
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
