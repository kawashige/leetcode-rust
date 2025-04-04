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
    pub fn find_max_depth(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize) -> usize {
        if let Some(node) = root {
            let n = node.borrow();
            Self::find_max_depth(&n.left, depth + 1).max(Self::find_max_depth(&n.right, depth + 1))
        } else {
            depth - 1
        }
    }

    pub fn find_lca(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        target_depth: usize,
    ) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if let Some(node) = root {
            let node = node.borrow();
            let (left, left_max_depth) = Self::find_lca(&node.left, depth + 1, target_depth);
            let (right, right_max_depth) = Self::find_lca(&node.right, depth + 1, target_depth);
            if left_max_depth != target_depth || right_max_depth != target_depth {
                return (
                    if left.is_none() { right } else { left },
                    left_max_depth.max(right_max_depth),
                );
            }

            let mut new_node = TreeNode::new(node.val);
            if left_max_depth == target_depth {
                new_node.left = node.left.clone();
            }
            if right_max_depth == target_depth {
                new_node.right = node.right.clone();
            }
            (Some(Rc::new(RefCell::new(new_node))), left_max_depth)
        } else {
            (None, depth - 1)
        }
    }

    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let max_depth = Self::find_max_depth(&root, 0);
        Self::find_lca(&root, 0, max_depth).0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1123() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(6);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(0);
        let t7 = TreeNode::new(8);
        let t8 = TreeNode::new(7);
        let t9 = TreeNode::new(4);
        t5.left = Some(Rc::new(RefCell::new(t8)));
        t5.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(2);
        let r2 = TreeNode::new(7);
        let r3 = TreeNode::new(4);
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::lca_deepest_leaves(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1))),
        );

        let t1 = TreeNode::new(1);
        let r1 = TreeNode::new(1);

        assert_eq!(
            Solution::lca_deepest_leaves(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1))),
        );

        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(1);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(2);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let r1 = TreeNode::new(2);

        assert_eq!(
            Solution::lca_deepest_leaves(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1))),
        );
    }
}
