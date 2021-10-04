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
        root1: &Option<Rc<RefCell<TreeNode>>>,
        root2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let n1 = node1.borrow();
                let n2 = node2.borrow();
                if n1.val == n2.val {
                    (Self::recurse(&n1.left, &n2.left) && Self::recurse(&n1.right, &n2.right))
                        || (Self::recurse(&n1.right, &n2.left)
                            && Self::recurse(&n1.left, &n2.right))
                } else {
                    false
                }
            }
            (None, None) => true,
            _ => false,
        }
    }

    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::recurse(&root1, &root2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0951() {
        let mut t1_1 = TreeNode::new(1);
        let mut t1_2 = TreeNode::new(2);
        let mut t1_3 = TreeNode::new(3);
        let t1_4 = TreeNode::new(4);
        let mut t1_5 = TreeNode::new(5);
        let t1_6 = TreeNode::new(6);
        let t1_7 = TreeNode::new(7);
        let t1_8 = TreeNode::new(8);
        t1_5.left = Some(Rc::new(RefCell::new(t1_7)));
        t1_5.right = Some(Rc::new(RefCell::new(t1_8)));
        t1_3.left = Some(Rc::new(RefCell::new(t1_6)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut t2_1 = TreeNode::new(1);
        let mut t2_2 = TreeNode::new(2);
        let mut t2_3 = TreeNode::new(3);
        let t2_4 = TreeNode::new(4);
        let mut t2_5 = TreeNode::new(5);
        let t2_6 = TreeNode::new(6);
        let t2_7 = TreeNode::new(7);
        let t2_8 = TreeNode::new(8);
        t2_5.left = Some(Rc::new(RefCell::new(t2_8)));
        t2_5.right = Some(Rc::new(RefCell::new(t2_7)));
        t2_3.right = Some(Rc::new(RefCell::new(t2_6)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_3)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_2)));

        assert!(Solution::flip_equiv(
            Some(Rc::new(RefCell::new(t1_1))),
            Some(Rc::new(RefCell::new(t2_1)))
        ));

        assert!(Solution::flip_equiv(None, None));

        assert!(!Solution::flip_equiv(
            None,
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        ));

        let mut t3_1 = TreeNode::new(0);
        let t3_2 = TreeNode::new(1);
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));

        assert!(!Solution::flip_equiv(
            Some(Rc::new(RefCell::new(t3_1))),
            None
        ));

        let mut t4_1 = TreeNode::new(0);
        let t4_2 = TreeNode::new(1);
        t4_1.left = Some(Rc::new(RefCell::new(t4_2)));

        let mut t5_1 = TreeNode::new(0);
        let t5_2 = TreeNode::new(1);
        t5_1.right = Some(Rc::new(RefCell::new(t5_2)));

        assert!(Solution::flip_equiv(
            Some(Rc::new(RefCell::new(t4_1))),
            Some(Rc::new(RefCell::new(t5_1))),
        ));
    }
}
