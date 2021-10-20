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
    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            let l = Self::recurse(&n.left, moves);
            let r = Self::recurse(&n.right, moves);
            let x = n.val + l + r - 1;
            *moves += x.abs();
            x
        } else {
            0
        }
    }

    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut moves = 0;
        Self::recurse(&root, &mut moves);
        moves
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0979() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(0);
        let t3 = TreeNode::new(0);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::distribute_coins(Some(Rc::new(RefCell::new(t1)))),
            2
        );

        let mut t1 = TreeNode::new(0);
        let t2 = TreeNode::new(3);
        let t3 = TreeNode::new(0);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::distribute_coins(Some(Rc::new(RefCell::new(t1)))),
            3
        );

        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(0);
        let t3 = TreeNode::new(2);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::distribute_coins(Some(Rc::new(RefCell::new(t1)))),
            2
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(0);
        let t3 = TreeNode::new(0);
        let t4 = TreeNode::new(3);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::distribute_coins(Some(Rc::new(RefCell::new(t1)))),
            4
        );
    }
}
