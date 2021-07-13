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
    pub fn leafs(root: &Option<Rc<RefCell<TreeNode>>>, leafs: &mut Vec<i32>) {
        if let Some(node) = root {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                leafs.push(n.val);
            } else {
                Self::leafs(&n.left, leafs);
                Self::leafs(&n.right, leafs);
            }
        }
    }

    pub fn check(root: &Option<Rc<RefCell<TreeNode>>>, leafs: &mut Vec<i32>) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                leafs.pop() == Some(n.val)
            } else {
                Self::check(&n.right, leafs) && Self::check(&n.left, leafs)
            }
        } else {
            true
        }
    }

    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leafs = Vec::new();
        Self::leafs(&root1, &mut leafs);
        Self::check(&root2, &mut leafs) && leafs.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0872() {
        let mut t1_1 = TreeNode::new(3);
        let mut t1_2 = TreeNode::new(5);
        let mut t1_3 = TreeNode::new(1);
        let t1_4 = TreeNode::new(6);
        let mut t1_5 = TreeNode::new(2);
        let t1_6 = TreeNode::new(9);
        let t1_7 = TreeNode::new(8);
        let t1_8 = TreeNode::new(7);
        let t1_9 = TreeNode::new(4);
        t1_5.left = Some(Rc::new(RefCell::new(t1_8)));
        t1_5.right = Some(Rc::new(RefCell::new(t1_9)));
        t1_3.left = Some(Rc::new(RefCell::new(t1_6)));
        t1_3.right = Some(Rc::new(RefCell::new(t1_7)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut t2_1 = TreeNode::new(3);
        let mut t2_2 = TreeNode::new(5);
        let mut t2_3 = TreeNode::new(1);
        let t2_4 = TreeNode::new(6);
        let t2_5 = TreeNode::new(7);
        let t2_6 = TreeNode::new(4);
        let mut t2_7 = TreeNode::new(2);
        let t2_8 = TreeNode::new(9);
        let t2_9 = TreeNode::new(8);
        t2_7.left = Some(Rc::new(RefCell::new(t2_8)));
        t2_7.right = Some(Rc::new(RefCell::new(t2_9)));
        t2_3.left = Some(Rc::new(RefCell::new(t2_6)));
        t2_3.right = Some(Rc::new(RefCell::new(t2_7)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert!(Solution::leaf_similar(
            Some(Rc::new(RefCell::new(t1_1))),
            Some(Rc::new(RefCell::new(t2_1)))
        ));

        assert!(Solution::leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        ));

        assert!(!Solution::leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        ));
    }
}
