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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut depth = 1;
        let mut nodes = vec![root];
        while nodes.len() > 0 {
            let mut new_nodes = Vec::new();
            for node in nodes {
                match node {
                    Some(n) => {
                        let n_inner = Rc::try_unwrap(n).unwrap().into_inner();
                        if n_inner.left.is_none() && n_inner.right.is_none() {
                            return depth;
                        }
                        new_nodes.push(n_inner.left);
                        new_nodes.push(n_inner.right);
                    }
                    None => {}
                }
            }
            depth += 1;
            nodes = new_nodes;
        }
        depth
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0111() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(2, Solution::min_depth(Some(Rc::new(RefCell::new(t1)))));
    }
}
