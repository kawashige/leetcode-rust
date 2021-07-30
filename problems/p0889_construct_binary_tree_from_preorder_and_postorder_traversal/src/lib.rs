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
    pub fn recurse(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            None
        } else {
            let mut node = TreeNode::new(preorder[0]);
            if preorder.len() > 1 {
                let i = (0..postorder.len())
                    .find(|i| postorder[*i] == preorder[1])
                    .unwrap();
                node.left = Self::recurse(&preorder[1..=(i + 1)], &postorder[..=i]);
                node.right = Self::recurse(
                    &preorder[(i + 2)..],
                    &postorder[(i + 1)..(postorder.len() - 1)],
                );
            }
            Some(Rc::new(RefCell::new(node)))
        }
    }

    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recurse(&preorder, &postorder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0889() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]),
            Some(Rc::new(RefCell::new(t1))),
        );

        assert_eq!(
            Solution::construct_from_pre_post(vec![1], vec![1]),
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        );

        assert_eq!(
            Solution::construct_from_pre_post(vec![1, 2], vec![2, 1]),
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        );
    }
}
