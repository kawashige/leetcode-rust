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
    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Vec<Vec<i32>>> {
        if let Some(node) = root {
            let n = node.borrow();
            match (Self::recurse(&n.left), Self::recurse(&n.right)) {
                (Some(left), None) => Some(vec![
                    vec![left[0][1], left[1][0].min(left[1][1])],
                    vec![
                        left[0][0].min(left[0][1]) + 1,
                        left[1][0].min(left[1][1]) + 1,
                    ],
                ]),
                (None, Some(right)) => Some(vec![
                    vec![right[0][1], right[1][0].min(right[1][1])],
                    vec![
                        right[0][0].min(right[0][1]) + 1,
                        right[1][0].min(right[1][1]) + 1,
                    ],
                ]),
                (Some(left), Some(right)) => Some(vec![
                    vec![
                        left[0][1] + right[0][1],
                        (left[1][0].min(left[1][1]) + right[0][1])
                            .min(left[0][1] + right[1][0].min(right[1][1]))
                            .min(left[1][0].min(left[1][1]) + right[1][0].min(right[1][1])),
                    ],
                    vec![
                        left[0][0].min(left[0][1]) + right[0][0].min(right[0][1]) + 1,
                        (left[1][0].min(left[1][1]) + right[0][0].min(right[0][1]))
                            .min(left[0][0].min(left[0][1]) + right[1][0].min(right[1][1]))
                            .min(left[1][0].min(left[1][1]) + right[1][0].min(right[1][1]))
                            + 1,
                    ],
                ]),
                (None, None) => Some(vec![vec![0, 10000], vec![1, 10000]]),
            }
        } else {
            None
        }
    }

    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let dp = Self::recurse(&root).unwrap();
        dp[0][1].min(dp[1][0]).min(dp[1][1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day16() {
        let t1 = TreeNode::new(0);

        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(t1)))),
            1
        );

        let mut t1 = TreeNode::new(0);
        let t2 = TreeNode::new(0);
        let mut t3 = TreeNode::new(0);
        let t4 = TreeNode::new(0);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(t1)))),
            2
        );

        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(0);
        let t3 = TreeNode::new(0);
        let t4 = TreeNode::new(0);
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(t1)))),
            1
        );

        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(0);
        let mut t3 = TreeNode::new(0);
        let mut t4 = TreeNode::new(0);
        let t5 = TreeNode::new(0);
        t4.right = Some(Rc::new(RefCell::new(t5)));
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(t1)))),
            2
        );

        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(0);
        let mut t3 = TreeNode::new(0);
        let t4 = TreeNode::new(0);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(t1)))),
            2
        );
    }
}
