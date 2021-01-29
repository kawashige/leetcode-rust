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
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recurse(s: &Option<Rc<RefCell<TreeNode>>>, t: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if s == t {
                return true;
            }
            if let Some(n) = s {
                let n = n.borrow();
                recurse(&n.left, t) || recurse(&n.right, t)
            } else {
                false
            }
        }

        recurse(&s, &t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0572() {
        let mut t1_1 = TreeNode::new(3);
        let mut t1_2 = TreeNode::new(4);
        let t1_3 = TreeNode::new(5);
        let t1_4 = TreeNode::new(1);
        let t1_5 = TreeNode::new(2);
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut t2_1 = TreeNode::new(4);
        let t2_2 = TreeNode::new(1);
        let t2_3 = TreeNode::new(2);
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert!(Solution::is_subtree(
            Some(Rc::new(RefCell::new(t1_1))),
            Some(Rc::new(RefCell::new(t2_1)))
        ));

        let mut t3_1 = TreeNode::new(3);
        let mut t3_2 = TreeNode::new(4);
        let t3_3 = TreeNode::new(5);
        let t3_4 = TreeNode::new(1);
        let mut t3_5 = TreeNode::new(2);
        let t3_6 = TreeNode::new(6);
        t3_5.left = Some(Rc::new(RefCell::new(t3_6)));
        t3_2.left = Some(Rc::new(RefCell::new(t3_4)));
        t3_2.right = Some(Rc::new(RefCell::new(t3_5)));
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));
        t3_1.right = Some(Rc::new(RefCell::new(t3_3)));

        let mut t4_1 = TreeNode::new(4);
        let t4_2 = TreeNode::new(1);
        let t4_3 = TreeNode::new(2);
        t4_1.left = Some(Rc::new(RefCell::new(t4_2)));
        t4_1.right = Some(Rc::new(RefCell::new(t4_3)));

        assert!(!Solution::is_subtree(
            Some(Rc::new(RefCell::new(t3_1))),
            Some(Rc::new(RefCell::new(t4_1)))
        ));

        let mut t5_1 = TreeNode::new(3);
        let mut t5_2 = TreeNode::new(4);
        let mut t5_3 = TreeNode::new(5);
        let t5_4 = TreeNode::new(1);
        let t5_5 = TreeNode::new(2);
        t5_3.left = Some(Rc::new(RefCell::new(t5_5)));
        t5_2.left = Some(Rc::new(RefCell::new(t5_4)));
        t5_1.left = Some(Rc::new(RefCell::new(t5_2)));
        t5_1.right = Some(Rc::new(RefCell::new(t5_3)));

        let mut t6_1 = TreeNode::new(3);
        let t6_2 = TreeNode::new(1);
        let t6_3 = TreeNode::new(2);
        t6_1.left = Some(Rc::new(RefCell::new(t6_2)));
        t6_1.right = Some(Rc::new(RefCell::new(t6_3)));

        assert!(!Solution::is_subtree(
            Some(Rc::new(RefCell::new(t5_1))),
            Some(Rc::new(RefCell::new(t6_1)))
        ));
    }
}
