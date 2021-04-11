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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize) -> usize {
            if let Some(node) = root {
                let n = node.borrow();
                std::cmp::max(
                    max_depth(&n.left, depth + 1),
                    max_depth(&n.right, depth + 1),
                )
            } else {
                depth - 1
            }
        }

        fn sum(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, max_depth: usize) -> i32 {
            if let Some(node) = root {
                let n = node.borrow();
                if depth == max_depth {
                    n.val
                } else {
                    sum(&n.left, depth + 1, max_depth) + sum(&n.right, depth + 1, max_depth)
                }
            } else {
                0
            }
        }

        sum(&&root, 0, max_depth(&root, 0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day11() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let mut t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        let mut t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(8);
        t6.right = Some(Rc::new(RefCell::new(t8)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::deepest_leaves_sum(Some(Rc::new(RefCell::new(t1)))),
            15
        );
    }
}
