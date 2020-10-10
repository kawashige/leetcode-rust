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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, depth: i32, depths: &mut Vec<i32>) {
            if let Some(n) = root {
                let n = n.borrow();
                if n.left.is_none()
                    && n.right.is_none()
                    && (depths.is_empty() || depths.contains(&depth))
                {
                    depths.push(depth);
                    return;
                }
                if n.left.is_some() {
                    recurse(&n.left, depth + 1, depths);
                }
                if n.right.is_some() {
                    recurse(&n.right, depth + 1, depths);
                }
            }
        }

        let mut depths = Vec::new();
        recurse(&root, 0, &mut depths);
        if depths.is_empty() {
            0
        } else {
            (0..depths[0]).fold(0 as i32, |sum, i| sum + (2 as i32).pow(i as u32))
                + depths.len() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0222() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(4, Solution::count_nodes(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        let t6 = TreeNode::new(6);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(6, Solution::count_nodes(Some(Rc::new(RefCell::new(t1)))));
    }
}
