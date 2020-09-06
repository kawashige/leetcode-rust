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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        fn sums(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<i32> {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    if n.left.is_none() && n.right.is_none() {
                        vec![sum + n.val]
                    } else if n.left.is_none() {
                        sums(&n.right, sum + n.val)
                    } else if n.right.is_none() {
                        sums(&n.left, sum + n.val)
                    } else {
                        sums(&n.left, sum + n.val)
                            .into_iter()
                            .chain(sums(&n.right, sum + n.val))
                            .collect::<Vec<_>>()
                    }
                }
                None => Vec::new(),
            }
        }
        if root.is_none() {
            return false;
        }
        sums(&root, 0).contains(&sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0112() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(8);
        let mut t4 = TreeNode::new(11);
        let t5 = TreeNode::new(13);
        let mut t6 = TreeNode::new(4);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(2);
        let t9 = TreeNode::new(1);
        t6.right = Some(Rc::new(RefCell::new(t9)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::has_path_sum(Some(Rc::new(RefCell::new(t1))), 22));

        let mut t2_1 = TreeNode::new(1);
        let t2_2 = TreeNode::new(2);
        t2_1.right = Some(Rc::new(RefCell::new(t2_2)));
        assert!(!Solution::has_path_sum(
            Some(Rc::new(RefCell::new(t2_1))),
            1
        ));
    }
}
