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
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn recurse(t: &Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(n) = t {
                let n = n.borrow();
                let left = recurse(&n.left);
                let right = recurse(&n.right);
                match (left.len(), right.len()) {
                    (0, 0) => n.val.to_string(),
                    (_, 0) => format!("{}({})", n.val, left),
                    _ => format!("{}({})({})", n.val, left, right),
                }
            } else {
                "".to_string()
            }
        }
        recurse(&t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0606() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::tree2str(Some(Rc::new(RefCell::new(t1)))),
            "1(2(4))(3)".to_string()
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::tree2str(Some(Rc::new(RefCell::new(t1)))),
            "1(2()(4))(3)".to_string()
        );
    }
}
