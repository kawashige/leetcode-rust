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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_symmetric_nodes(
            node1: Option<Rc<RefCell<TreeNode>>>,
            node2: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (node1, node2) {
                (Some(n1), Some(n2)) => {
                    let inner_n1 = Rc::try_unwrap(n1).unwrap().into_inner();
                    let inner_n2 = Rc::try_unwrap(n2).unwrap().into_inner();

                    inner_n1.val == inner_n2.val
                        && is_symmetric_nodes(inner_n1.left, inner_n2.right)
                        && is_symmetric_nodes(inner_n1.right, inner_n2.left)
                }
                (None, None) => true,
                (_, _) => false,
            }
        }

        match root {
            Some(r) => {
                let inner_r = Rc::try_unwrap(r).unwrap().into_inner();
                is_symmetric_nodes(inner_r.left, inner_r.right)
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0101() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(2);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(4);
        let t6 = TreeNode::new(4);
        let t7 = TreeNode::new(3);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(t1)))));
    }
}
