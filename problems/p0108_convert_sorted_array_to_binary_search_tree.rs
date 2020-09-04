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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match nums.len() {
                0 => None,
                _ => {
                    let i = nums.len() / 2;
                    let mut t = TreeNode::new(nums[i]);
                    t.left = build(&nums[..i]);
                    t.right = build(&nums[(i + 1)..]);
                    Some(Rc::new(RefCell::new(t)))
                }
            }
        }

        build(&nums)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0108() {
        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(-3);
        let mut t3 = TreeNode::new(9);
        let t4 = TreeNode::new(-10);
        let t5 = TreeNode::new(5);
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let result = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]).unwrap();
        assert_eq!(Rc::new(RefCell::new(t1)), result);

        let result2 = Solution::sorted_array_to_bst(vec![-1]).unwrap();
        assert_eq!(Rc::new(RefCell::new(TreeNode::new(-1))), result2);
    }
}
