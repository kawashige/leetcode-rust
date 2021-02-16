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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let i = (0..nums.len()).max_by_key(|i| nums[*i]).unwrap();
            let mut node = TreeNode::new(nums[i]);
            if 0 < i {
                node.left = recurse(&nums[..i]);
            }
            if i < nums.len() - 1 {
                node.right = recurse(&nums[(i + 1)..]);
            }
            Some(Rc::new(RefCell::new(node)))
        }
        recurse(&nums)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0654() {
        let mut r1_1 = TreeNode::new(6);
        let mut r1_2 = TreeNode::new(3);
        let mut r1_3 = TreeNode::new(5);
        let mut r1_4 = TreeNode::new(2);
        let r1_5 = TreeNode::new(0);
        let r1_6 = TreeNode::new(1);
        r1_4.right = Some(Rc::new(RefCell::new(r1_6)));
        r1_3.left = Some(Rc::new(RefCell::new(r1_5)));
        r1_2.right = Some(Rc::new(RefCell::new(r1_4)));
        r1_1.left = Some(Rc::new(RefCell::new(r1_2)));
        r1_1.right = Some(Rc::new(RefCell::new(r1_3)));

        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]),
            Some(Rc::new(RefCell::new(r1_1)))
        );

        let mut r2_1 = TreeNode::new(3);
        let mut r2_2 = TreeNode::new(2);
        let r2_3 = TreeNode::new(1);
        r2_2.right = Some(Rc::new(RefCell::new(r2_3)));
        r2_1.right = Some(Rc::new(RefCell::new(r2_2)));

        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1]),
            Some(Rc::new(RefCell::new(r2_1)))
        );
    }
}
