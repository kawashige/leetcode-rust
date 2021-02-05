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
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        pub fn recurse(
            t1: &Option<Rc<RefCell<TreeNode>>>,
            t2: &Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match (t1, t2) {
                (Some(n1), Some(n2)) => {
                    let n1 = n1.borrow();
                    let n2 = n2.borrow();
                    let mut node = TreeNode::new(n1.val + n2.val);
                    node.right = recurse(&n1.right, &n2.right);
                    node.left = recurse(&n1.left, &n2.left);
                    Some(Rc::new(RefCell::new(node)))
                }
                (Some(n1), None) => Some(n1.clone()),
                (None, Some(n2)) => Some(n2.clone()),
                _ => None,
            }
        }
        recurse(&t1, &t2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0617() {
        let mut t1_1 = TreeNode::new(1);
        let mut t1_2 = TreeNode::new(3);
        let t1_3 = TreeNode::new(2);
        let t1_4 = TreeNode::new(5);
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut t2_1 = TreeNode::new(2);
        let mut t2_2 = TreeNode::new(1);
        let mut t2_3 = TreeNode::new(3);
        let t2_4 = TreeNode::new(4);
        let t2_5 = TreeNode::new(7);
        t2_3.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_4)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        let mut r_1 = TreeNode::new(3);
        let mut r_2 = TreeNode::new(4);
        let mut r_3 = TreeNode::new(5);
        let r_4 = TreeNode::new(5);
        let r_5 = TreeNode::new(4);
        let r_6 = TreeNode::new(7);
        r_3.right = Some(Rc::new(RefCell::new(r_6)));
        r_2.left = Some(Rc::new(RefCell::new(r_4)));
        r_2.right = Some(Rc::new(RefCell::new(r_5)));
        r_1.left = Some(Rc::new(RefCell::new(r_2)));
        r_1.right = Some(Rc::new(RefCell::new(r_3)));

        assert_eq!(
            Solution::merge_trees(
                Some(Rc::new(RefCell::new(t1_1))),
                Some(Rc::new(RefCell::new(t2_1)))
            ),
            Some(Rc::new(RefCell::new(r_1)))
        );
    }
}
