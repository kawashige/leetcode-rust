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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, max: i32, min: i32) -> i32 {
            if let Some(n) = root {
                let n = n.borrow();
                let mut diff = if max == -1 {
                    0
                } else {
                    std::cmp::max((n.val - min).abs(), (n.val - max).abs())
                };
                diff = std::cmp::max(
                    diff,
                    recurse(
                        &n.left,
                        std::cmp::max(max, n.val),
                        std::cmp::min(min, n.val),
                    ),
                );
                std::cmp::max(
                    diff,
                    recurse(
                        &n.right,
                        std::cmp::max(max, n.val),
                        std::cmp::min(min, n.val),
                    ),
                )
            } else {
                0
            }
        }
        recurse(&root, -1, 100001)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day9() {
        let mut t1 = TreeNode::new(8);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(10);
        let t4 = TreeNode::new(1);
        let mut t5 = TreeNode::new(6);
        let mut t6 = TreeNode::new(14);
        let t7 = TreeNode::new(4);
        let t8 = TreeNode::new(7);
        let t9 = TreeNode::new(13);
        t6.right = Some(Rc::new(RefCell::new(t9)));
        t5.left = Some(Rc::new(RefCell::new(t7)));
        t5.right = Some(Rc::new(RefCell::new(t8)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            7,
            Solution::max_ancestor_diff(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(0);
        let t4 = TreeNode::new(3);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t3)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            3,
            Solution::max_ancestor_diff(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
