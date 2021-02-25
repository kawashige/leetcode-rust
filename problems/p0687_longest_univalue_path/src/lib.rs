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
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> (i32, i32) {
            if let Some(node) = root {
                let n = node.borrow();
                let left = recurse(&n.left, max);
                let right = recurse(&n.right, max);
                let mut count = 0;
                if n.val == left.0 {
                    count += left.1 + 1;
                }
                if n.val == right.0 {
                    count += right.1 + 1;
                }
                *max = std::cmp::max(*max, count);
                if n.val == right.0 && n.val == left.0 {
                    count -= std::cmp::min(right.1 + 1, left.1 + 1);
                }
                (n.val, count)
            } else {
                (-1001, 0)
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
    fn test_0687() {
        let mut t1_1 = TreeNode::new(5);
        let mut t1_2 = TreeNode::new(4);
        let mut t1_3 = TreeNode::new(5);
        let t1_4 = TreeNode::new(1);
        let t1_5 = TreeNode::new(1);
        let t1_6 = TreeNode::new(5);
        t1_3.right = Some(Rc::new(RefCell::new(t1_6)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert_eq!(
            Solution::longest_univalue_path(Some(Rc::new(RefCell::new(t1_1)))),
            2
        );

        let mut t2_1 = TreeNode::new(1);
        let mut t2_2 = TreeNode::new(4);
        let mut t2_3 = TreeNode::new(5);
        let t2_4 = TreeNode::new(4);
        let t2_5 = TreeNode::new(4);
        let t2_6 = TreeNode::new(5);
        t2_3.right = Some(Rc::new(RefCell::new(t2_6)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert_eq!(
            Solution::longest_univalue_path(Some(Rc::new(RefCell::new(t2_1)))),
            2
        );

        let mut t3_1 = TreeNode::new(1);
        let mut t3_2 = TreeNode::new(1);
        let mut t3_3 = TreeNode::new(1);
        let mut t3_4 = TreeNode::new(1);
        let t3_5 = TreeNode::new(1);
        let t3_6 = TreeNode::new(1);
        let t3_7 = TreeNode::new(1);
        t3_4.right = Some(Rc::new(RefCell::new(t3_7)));
        t3_3.left = Some(Rc::new(RefCell::new(t3_5)));
        t3_3.right = Some(Rc::new(RefCell::new(t3_6)));
        t3_2.left = Some(Rc::new(RefCell::new(t3_3)));
        t3_2.right = Some(Rc::new(RefCell::new(t3_4)));
        t3_1.right = Some(Rc::new(RefCell::new(t3_2)));

        assert_eq!(
            Solution::longest_univalue_path(Some(Rc::new(RefCell::new(t3_1)))),
            4
        );
    }
}
