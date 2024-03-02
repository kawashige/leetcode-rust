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
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn recurse(
        node: &Option<Rc<RefCell<TreeNode>>>,
        level: usize,
        values: &mut Vec<VecDeque<i32>>,
    ) {
        if let Some(node) = node {
            let node = node.borrow();
            if values.len() < level + 1 {
                values.push(VecDeque::new());
            }
            values[level].push_back(node.val);
            Self::recurse(&node.left, level + 1, values);
            Self::recurse(&node.right, level + 1, values);
        }
    }

    pub fn recurse2(
        level: usize,
        values: &mut Vec<VecDeque<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if values.len() <= level {
            return None;
        }
        let val = if level % 2 == 0 {
            values[level].pop_front().unwrap()
        } else {
            values[level].pop_back().unwrap()
        };
        let mut node = TreeNode::new(val);
        node.left = Self::recurse2(level + 1, values);
        node.right = Self::recurse2(level + 1, values);
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = Vec::new();
        Self::recurse(&root, 0, &mut values);
        Self::recurse2(0, &mut values)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2415() {
        let mut t1 = TreeNode::new(2);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(5);
        let t4 = TreeNode::new(8);
        let t5 = TreeNode::new(13);
        let t6 = TreeNode::new(21);
        let t7 = TreeNode::new(34);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(2);
        let mut r2 = TreeNode::new(5);
        let mut r3 = TreeNode::new(3);
        let r4 = TreeNode::new(8);
        let r5 = TreeNode::new(13);
        let r6 = TreeNode::new(21);
        let r7 = TreeNode::new(34);
        r3.left = Some(Rc::new(RefCell::new(r6)));
        r3.right = Some(Rc::new(RefCell::new(r7)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::reverse_odd_levels(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );
    }
}
