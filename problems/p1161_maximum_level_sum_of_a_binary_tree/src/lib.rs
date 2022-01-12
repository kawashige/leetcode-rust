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
    pub fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, sum: &mut Vec<i32>) {
        if let Some(node) = root {
            let n = node.borrow();
            if sum.len() < level {
                sum.push(n.val);
            } else {
                sum[level - 1] += n.val;
            }
            Self::recurse(&n.left, level + 1, sum);
            Self::recurse(&n.right, level + 1, sum);
        }
    }
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = Vec::new();
        Self::recurse(&root, 1, &mut sum);
        (0..sum.len())
            .max_by_key(|i| (sum[*i], -(*i as i32)))
            .unwrap() as i32
            + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1161() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(7);
        let t3 = TreeNode::new(0);
        let t4 = TreeNode::new(7);
        let t5 = TreeNode::new(-8);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(Solution::max_level_sum(Some(Rc::new(RefCell::new(t1)))), 2);

        let mut t1 = TreeNode::new(989);
        let mut t2 = TreeNode::new(10250);
        let t3 = TreeNode::new(98693);
        let mut t4 = TreeNode::new(-89388);
        let t5 = TreeNode::new(-32127);
        t4.right = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(Solution::max_level_sum(Some(Rc::new(RefCell::new(t1)))), 2);

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(0);
        let t4 = TreeNode::new(7);
        let t5 = TreeNode::new(-8);
        let t6 = TreeNode::new(-7);
        let t7 = TreeNode::new(9);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(Solution::max_level_sum(Some(Rc::new(RefCell::new(t1)))), 1);
    }
}
