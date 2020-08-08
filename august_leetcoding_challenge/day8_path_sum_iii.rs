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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        fn count(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, sums: &Vec<i32>) -> i32 {
            match root {
                Some(r) => {
                    let r_inner = Rc::try_unwrap(r).unwrap().into_inner();
                    let result = sums.iter().filter(|s| **s == r_inner.val).count();
                    let mut new_sums = sums
                        .clone()
                        .iter()
                        .map(|s| *s - r_inner.val)
                        .collect::<Vec<i32>>();
                    new_sums.push(sum);
                    result as i32
                        + count(r_inner.left, sum, &new_sums)
                        + count(r_inner.right, sum, &new_sums)
                }
                None => 0,
            }
        }

        count(root, sum, &vec![sum])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day8() {
        let mut t1 = TreeNode::new(10);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(-3);
        let mut t4 = TreeNode::new(3);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(11);
        let t7 = TreeNode::new(3);
        let t8 = TreeNode::new(-2);
        let t9 = TreeNode::new(1);
        t5.right = Some(Rc::new(RefCell::new(t9)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(3, Solution::path_sum(Some(Rc::new(RefCell::new(t1))), 8))
    }
}
