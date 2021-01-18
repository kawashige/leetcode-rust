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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(n) = root {
                let n = n.borrow();
                nums.push(n.val);
                recurse(&n.left, nums);
                recurse(&n.right, nums);
            }
        }

        let mut nums = Vec::new();
        recurse(&root, &mut nums);
        nums.sort();
        nums.windows(2).map(|w| w[1] - w[0]).min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0530() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(3);
        let t3 = TreeNode::new(2);
        t2.right = Some(Rc::new(RefCell::new(t3)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            1,
            Solution::get_minimum_difference(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
