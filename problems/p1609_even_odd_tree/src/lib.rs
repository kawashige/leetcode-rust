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
    pub fn recurse(
        root: &Option<Rc<RefCell<TreeNode>>>,
        level: usize,
        nodes: &mut Vec<i32>,
    ) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            if nodes.len() <= level {
                if n.val as usize % 2 != level % 2 {
                    nodes.push(n.val);
                    Self::recurse(&n.left, level + 1, nodes)
                        && Self::recurse(&n.right, level + 1, nodes)
                } else {
                    false
                }
            } else if (level % 2 == 0 && n.val % 2 == 1 && nodes[level] < n.val)
                || (level % 2 == 1 && n.val % 2 == 0 && nodes[level] > n.val)
            {
                nodes[level] = n.val;
                Self::recurse(&n.left, level + 1, nodes)
                    && Self::recurse(&n.right, level + 1, nodes)
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::recurse(&root, 0, &mut vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1609() {
        let mut t1 = TreeNode::new(2);
        let t2 = TreeNode::new(12);
        let t3 = TreeNode::new(8);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(10);
        let mut t3 = TreeNode::new(4);
        let mut t4 = TreeNode::new(3);
        let mut t5 = TreeNode::new(7);
        let mut t6 = TreeNode::new(9);
        let t7 = TreeNode::new(12);
        let t8 = TreeNode::new(8);
        let t9 = TreeNode::new(6);
        let t10 = TreeNode::new(2);
        t6.right = Some(Rc::new(RefCell::new(t10)));
        t5.left = Some(Rc::new(RefCell::new(t9)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(2);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(3);
        let t6 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(t1)))));

        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(5);
        let t6 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::is_even_odd_tree(Some(Rc::new(RefCell::new(t1)))));
    }
}
