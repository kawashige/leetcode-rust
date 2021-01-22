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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            if let Some(node) = root {
                let n = node.borrow();
                let left = recurse(&n.left, max);
                let right = recurse(&n.right, max);

                *max = std::cmp::max(*max, left + right);
                std::cmp::max(left, right) + 1
            } else {
                0
            }
        }

        let mut max = 0;
        recurse(&root, &mut max);
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0543() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            3,
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
