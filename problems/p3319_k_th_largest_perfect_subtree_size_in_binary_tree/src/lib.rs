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
    pub fn recurse(tree: &Option<Rc<RefCell<TreeNode>>>, size: &mut Vec<i32>) -> Option<i32> {
        if let Some(node) = tree {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                size.push(1);
                return Some(1);
            }
            match (
                Self::recurse(&node.left, size),
                Self::recurse(&node.right, size),
            ) {
                (Some(l), Some(r)) => {
                    if l == r {
                        size.push(l + r + 1);
                        Some(l + r + 1)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let k = k as usize;
        let mut size = Vec::new();
        Self::recurse(&root, &mut size);
        size.sort_unstable_by_key(|k| -k);
        if size.len() < k {
            -1
        } else {
            size[k - 1]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3319() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(6);
        let mut t4 = TreeNode::new(5);
        let t5 = TreeNode::new(2);
        let mut t6 = TreeNode::new(5);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(1);
        let t9 = TreeNode::new(8);
        let t10 = TreeNode::new(6);
        let t11 = TreeNode::new(8);
        t6.left = Some(Rc::new(RefCell::new(t10)));
        t6.right = Some(Rc::new(RefCell::new(t11)));
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t4.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::kth_largest_perfect_subtree(Some(Rc::new(RefCell::new(t1))), 2),
            3
        );

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
            Solution::kth_largest_perfect_subtree(Some(Rc::new(RefCell::new(t1))), 1),
            7
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::kth_largest_perfect_subtree(Some(Rc::new(RefCell::new(t1))), 3),
            -1
        );
    }
}
