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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
        d: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(root: &mut Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) {
            if let Some(node) = root {
                let mut n = node.borrow_mut();
                if d == 1 {
                    let mut left_node = TreeNode::new(v);
                    left_node.left = n.left.take();
                    let mut right_node = TreeNode::new(v);
                    right_node.right = n.right.take();
                    n.left = Some(Rc::new(RefCell::new(left_node)));
                    n.right = Some(Rc::new(RefCell::new(right_node)));
                } else {
                    recurse(&mut n.left, v, d - 1);
                    recurse(&mut n.right, v, d - 1);
                }
            }
        }

        if d == 1 {
            let mut node = TreeNode::new(v);
            node.left = root;
            Some(Rc::new(RefCell::new(node)))
        } else {
            let mut root = root;
            recurse(&mut root, v, d - 1);
            root
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0623() {
        let mut t1_1 = TreeNode::new(4);
        let mut t1_2 = TreeNode::new(2);
        let mut t1_3 = TreeNode::new(6);
        let t1_4 = TreeNode::new(3);
        let t1_5 = TreeNode::new(1);
        let t1_6 = TreeNode::new(5);
        t1_3.left = Some(Rc::new(RefCell::new(t1_6)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut r1_1 = TreeNode::new(4);
        let mut r1_2 = TreeNode::new(1);
        let mut r1_3 = TreeNode::new(1);
        let mut r1_4 = TreeNode::new(2);
        let mut r1_5 = TreeNode::new(6);
        let r1_6 = TreeNode::new(3);
        let r1_7 = TreeNode::new(1);
        let r1_8 = TreeNode::new(5);
        r1_5.left = Some(Rc::new(RefCell::new(r1_8)));
        r1_4.left = Some(Rc::new(RefCell::new(r1_6)));
        r1_4.right = Some(Rc::new(RefCell::new(r1_7)));
        r1_3.right = Some(Rc::new(RefCell::new(r1_5)));
        r1_2.left = Some(Rc::new(RefCell::new(r1_4)));
        r1_1.left = Some(Rc::new(RefCell::new(r1_2)));
        r1_1.right = Some(Rc::new(RefCell::new(r1_3)));

        assert_eq!(
            Solution::add_one_row(Some(Rc::new(RefCell::new(t1_1))), 1, 2),
            Some(Rc::new(RefCell::new(r1_1)))
        );

        let mut r2_1 = TreeNode::new(1);
        let r2_2 = TreeNode::new(4);
        r2_1.left = Some(Rc::new(RefCell::new(r2_2)));

        assert_eq!(
            Solution::add_one_row(Some(Rc::new(RefCell::new(TreeNode::new(4)))), 1, 1),
            Some(Rc::new(RefCell::new(r2_1)))
        );

        let mut t3_1 = TreeNode::new(4);
        let mut t3_2 = TreeNode::new(2);
        let t3_3 = TreeNode::new(3);
        let t3_4 = TreeNode::new(1);
        t3_2.left = Some(Rc::new(RefCell::new(t3_3)));
        t3_2.right = Some(Rc::new(RefCell::new(t3_4)));
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));

        let mut r3_1 = TreeNode::new(4);
        let mut r3_2 = TreeNode::new(2);
        let mut r3_3 = TreeNode::new(1);
        let mut r3_4 = TreeNode::new(1);
        let r3_5 = TreeNode::new(3);
        let r3_6 = TreeNode::new(1);
        r3_4.right = Some(Rc::new(RefCell::new(r3_6)));
        r3_3.left = Some(Rc::new(RefCell::new(r3_5)));
        r3_2.left = Some(Rc::new(RefCell::new(r3_3)));
        r3_2.right = Some(Rc::new(RefCell::new(r3_4)));
        r3_1.left = Some(Rc::new(RefCell::new(r3_2)));

        assert_eq!(
            Solution::add_one_row(Some(Rc::new(RefCell::new(t3_1))), 1, 3),
            Some(Rc::new(RefCell::new(r3_1)))
        );
    }
}
