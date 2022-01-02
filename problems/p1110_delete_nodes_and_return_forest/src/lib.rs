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
        to_delete: &Vec<bool>,
        is_top: bool,
        forests: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> bool {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            if to_delete[n.val as usize] {
                Self::recurse(&n.left, to_delete, true, forests);
                Self::recurse(&n.right, to_delete, true, forests);
                true
            } else {
                if Self::recurse(&n.left, to_delete, false, forests) {
                    n.left = None;
                }
                if Self::recurse(&n.right, to_delete, false, forests) {
                    n.right = None;
                };
                if is_top {
                    forests.push(Some(node.clone()))
                }
                false
            }
        } else {
            false
        }
    }

    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut forests = Vec::new();
        let to_delete = to_delete
            .into_iter()
            .fold(vec![false; 1001], |mut to_delete, val| {
                to_delete[val as usize] = true;
                to_delete
            });
        Self::recurse(&root, &to_delete, true, &mut forests);

        forests
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1110() {
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

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(2);
        let r4 = TreeNode::new(4);
        let r6 = TreeNode::new(6);
        let r7 = TreeNode::new(7);
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Solution::del_nodes(Some(Rc::new(RefCell::new(t1))), vec![3, 5]),
            vec![
                Some(Rc::new(RefCell::new(r6))),
                Some(Rc::new(RefCell::new(r7))),
                Some(Rc::new(RefCell::new(r1))),
            ]
        )
    }
}
