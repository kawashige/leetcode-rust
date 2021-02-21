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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, num: Option<i32>) -> Option<i32> {
            if let Some(node) = root {
                let n = node.borrow();
                let num = num.or(n.val.into());
                if n.val != num.unwrap() {
                    n.val.into()
                } else {
                    match (recurse(&n.left, num), recurse(&n.right, num)) {
                        (Some(r), Some(l)) => std::cmp::min(r, l).into(),
                        (Some(r), None) => r.into(),
                        (None, Some(l)) => l.into(),
                        (None, None) => None,
                    }
                }
            } else {
                None
            }
        }

        recurse(&root, None).unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0671() {
        let mut t1_1 = TreeNode::new(2);
        let t1_2 = TreeNode::new(2);
        let mut t1_3 = TreeNode::new(5);
        let t1_4 = TreeNode::new(5);
        let t1_5 = TreeNode::new(7);
        t1_3.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_3.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert_eq!(
            Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(t1_1)))),
            5
        );

        let mut t2_1 = TreeNode::new(2);
        let t2_2 = TreeNode::new(2);
        let t2_3 = TreeNode::new(2);
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert_eq!(
            Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(t2_1)))),
            -1
        );
    }
}
