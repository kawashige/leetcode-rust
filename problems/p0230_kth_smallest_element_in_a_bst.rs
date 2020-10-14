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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, values: &mut Vec<i32>) {
            if let Some(n) = root {
                let n = n.borrow();
                if n.left.is_some() {
                    recurse(&n.left, k, values);
                    if values.len() as i32 == k {
                        return;
                    }
                }
                values.push(n.val);
                if values.len() as i32 == k {
                    return;
                }
                if n.right.is_some() {
                    recurse(&n.right, k, values);
                }
            }
        }

        let mut values = Vec::new();
        recurse(&root, k, &mut values);
        *values.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0230() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(1);
        let t3 = TreeNode::new(4);
        let t4 = TreeNode::new(2);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            1,
            Solution::kth_smallest(Some(Rc::new(RefCell::new(t1))), 1)
        );

        let mut t2_1 = TreeNode::new(5);
        let mut t2_2 = TreeNode::new(3);
        let t2_3 = TreeNode::new(6);
        let mut t2_4 = TreeNode::new(2);
        let t2_5 = TreeNode::new(4);
        let t2_6 = TreeNode::new(1);
        t2_4.left = Some(Rc::new(RefCell::new(t2_6)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert_eq!(
            3,
            Solution::kth_smallest(Some(Rc::new(RefCell::new(t2_1))), 3)
        );
    }
}
