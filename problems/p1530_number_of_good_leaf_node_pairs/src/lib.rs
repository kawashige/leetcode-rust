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
    pub fn recurse(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        distance: i32,
        count: &mut i32,
    ) -> Option<Vec<i32>> {
        if let Some(node) = root {
            let n = node.borrow();
            match (
                Self::recurse(&n.left, depth + 1, distance, count),
                Self::recurse(&n.right, depth + 1, distance, count),
            ) {
                (Some(left), Some(right)) => {
                    let mut result = Vec::with_capacity(left.len() + right.len());
                    for l in &left {
                        result.push(*l);
                        for r in &right {
                            if l + r - depth * 2 <= distance {
                                *count += 1;
                            }
                        }
                    }
                    for r in right {
                        result.push(r);
                    }
                    result.into()
                }
                (Some(left), None) => left.into(),
                (None, Some(right)) => right.into(),
                (None, None) => vec![depth].into(),
            }
        } else {
            None
        }
    }

    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut count = 0;
        Self::recurse(&root, 0, distance, &mut count);
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1530() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(Solution::count_pairs(Some(Rc::new(RefCell::new(t1))), 3), 1);

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(Solution::count_pairs(Some(Rc::new(RefCell::new(t1))), 3), 2);

        let mut t1 = TreeNode::new(7);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(4);
        let t4 = TreeNode::new(6);
        let t5 = TreeNode::new(5);
        let mut t6 = TreeNode::new(3);
        let t7 = TreeNode::new(2);
        t6.right = Some(Rc::new(RefCell::new(t7)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(Solution::count_pairs(Some(Rc::new(RefCell::new(t1))), 3), 1);
    }
}
