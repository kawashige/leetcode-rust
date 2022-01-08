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
    pub fn count(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            1 + Self::count(&n.left) + Self::count(&n.right)
        } else {
            0
        }
    }

    pub fn check(root: &Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        if let Some(node) = root {
            let nd = node.borrow();
            if nd.val == x {
                let left = Self::count(&nd.left);
                let right = Self::count(&nd.right);
                left > n - left || right > n - right || n - left - right - 1 > left + right + 1
            } else {
                Self::check(&nd.left, n, x) || Self::check(&nd.right, n, x)
            }
        } else {
            false
        }
    }

    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        Self::check(&root, n, x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1145() {
        let mut t1 = TreeNode::new(5);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(6);
        let mut t4 = TreeNode::new(7);
        let mut t5 = TreeNode::new(8);
        let t6 = TreeNode::new(1);
        let mut t7 = TreeNode::new(4);
        let mut t8 = TreeNode::new(3);
        let t9 = TreeNode::new(2);
        t8.right = Some(Rc::new(RefCell::new(t9)));
        t7.right = Some(Rc::new(RefCell::new(t8)));
        t5.left = Some(Rc::new(RefCell::new(t6)));
        t5.right = Some(Rc::new(RefCell::new(t7)));
        t4.right = Some(Rc::new(RefCell::new(t5)));
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::btree_game_winning_move(
            Some(Rc::new(RefCell::new(t1))),
            9,
            8
        ));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let mut t4 = TreeNode::new(4);
        let mut t5 = TreeNode::new(5);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(8);
        let t9 = TreeNode::new(9);
        let t10 = TreeNode::new(10);
        let t11 = TreeNode::new(11);
        t5.left = Some(Rc::new(RefCell::new(t10)));
        t5.right = Some(Rc::new(RefCell::new(t11)));
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t4.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::btree_game_winning_move(
            Some(Rc::new(RefCell::new(t1))),
            11,
            3
        ));

        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::btree_game_winning_move(
            Some(Rc::new(RefCell::new(t1))),
            3,
            1
        ));
    }
}
