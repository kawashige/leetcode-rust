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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
            let mut result = 0;
            if let Some(n) = root {
                let n = n.borrow();
                if low <= n.val && n.val <= high {
                    result += n.val;
                }
                if low < n.val {
                    result += recurse(&n.left, low, high);
                }
                if n.val < high {
                    result += recurse(&n.right, low, high);
                }
            }
            result
        }

        recurse(&root, low, high)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day15() {
        let mut t1 = TreeNode::new(10);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(15);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(7);
        let t6 = TreeNode::new(18);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            32,
            Solution::range_sum_bst(Some(Rc::new(RefCell::new(t1))), 7, 15)
        );

        let mut t1 = TreeNode::new(10);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(15);
        let mut t4 = TreeNode::new(3);
        let mut t5 = TreeNode::new(7);
        let t6 = TreeNode::new(13);
        let t7 = TreeNode::new(18);
        let t8 = TreeNode::new(1);
        let t9 = TreeNode::new(6);
        t5.left = Some(Rc::new(RefCell::new(t9)));
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            23,
            Solution::range_sum_bst(Some(Rc::new(RefCell::new(t1))), 6, 10)
        );
    }
}
