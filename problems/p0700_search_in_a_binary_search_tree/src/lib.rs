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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let n = Rc::try_unwrap(node).unwrap().into_inner();
            if n.val == val {
                Some(Rc::new(RefCell::new(n)))
            } else if val < n.val {
                Self::search_bst(n.left, val)
            } else {
                Self::search_bst(n.right, val)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0700() {
        let mut t1_1 = TreeNode::new(4);
        let mut t1_2 = TreeNode::new(2);
        let t1_3 = TreeNode::new(7);
        let t1_4 = TreeNode::new(1);
        let t1_5 = TreeNode::new(3);
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut r1_1 = TreeNode::new(2);
        let r1_2 = TreeNode::new(1);
        let r1_3 = TreeNode::new(3);
        r1_1.left = Some(Rc::new(RefCell::new(r1_2)));
        r1_1.right = Some(Rc::new(RefCell::new(r1_3)));

        assert_eq!(
            Solution::search_bst(Some(Rc::new(RefCell::new(t1_1))), 2),
            Some(Rc::new(RefCell::new(r1_1)))
        );

        let mut t2_1 = TreeNode::new(4);
        let mut t2_2 = TreeNode::new(2);
        let t2_3 = TreeNode::new(7);
        let t2_4 = TreeNode::new(1);
        let t2_5 = TreeNode::new(3);
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert_eq!(
            Solution::search_bst(Some(Rc::new(RefCell::new(t2_1))), 5),
            None
        );
    }
}
