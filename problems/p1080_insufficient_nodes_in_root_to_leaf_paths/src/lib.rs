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
    pub fn delete(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, limit: i32) -> bool {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            match (&n.left, &n.right) {
                (None, None) => sum + n.val < limit,
                (_, _) => {
                    let left = Self::delete(&n.left, sum + n.val, limit);
                    if left {
                        n.left = None;
                    }
                    let right = Self::delete(&n.right, sum + n.val, limit);
                    if right {
                        n.right = None;
                    }
                    left && right
                }
            }
        } else {
            true
        }
    }

    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::delete(&root, 0, limit) {
            None
        } else {
            root
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1080() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let mut t4 = TreeNode::new(4);
        let mut t5 = TreeNode::new(-99);
        let mut t6 = TreeNode::new(-99);
        let mut t7 = TreeNode::new(7);
        let t8 = TreeNode::new(8);
        let t9 = TreeNode::new(9);
        let t10 = TreeNode::new(-99);
        let t11 = TreeNode::new(-99);
        let t12 = TreeNode::new(12);
        let t13 = TreeNode::new(13);
        let t14 = TreeNode::new(-99);
        let t15 = TreeNode::new(14);
        t7.left = Some(Rc::new(RefCell::new(t14)));
        t7.right = Some(Rc::new(RefCell::new(t15)));
        t6.left = Some(Rc::new(RefCell::new(t12)));
        t6.right = Some(Rc::new(RefCell::new(t13)));
        t5.left = Some(Rc::new(RefCell::new(t10)));
        t5.right = Some(Rc::new(RefCell::new(t11)));
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t4.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r2 = TreeNode::new(2);
        let mut r3 = TreeNode::new(3);
        let mut r4 = TreeNode::new(4);
        let mut r7 = TreeNode::new(7);
        let r8 = TreeNode::new(8);
        let r9 = TreeNode::new(9);
        let r15 = TreeNode::new(14);
        r7.right = Some(Rc::new(RefCell::new(r15)));
        r4.left = Some(Rc::new(RefCell::new(r8)));
        r4.right = Some(Rc::new(RefCell::new(r9)));
        r3.right = Some(Rc::new(RefCell::new(r7)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::sufficient_subset(Some(Rc::new(RefCell::new(t1))), 1),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(8);
        let mut t4 = TreeNode::new(11);
        let t5 = TreeNode::new(17);
        let mut t6 = TreeNode::new(4);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(1);
        let t9 = TreeNode::new(5);
        let t10 = TreeNode::new(3);
        t6.left = Some(Rc::new(RefCell::new(t9)));
        t6.right = Some(Rc::new(RefCell::new(t10)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(5);
        let mut r2 = TreeNode::new(4);
        let mut r3 = TreeNode::new(8);
        let mut r4 = TreeNode::new(11);
        let r5 = TreeNode::new(17);
        let mut r6 = TreeNode::new(4);
        let r7 = TreeNode::new(7);
        let r9 = TreeNode::new(5);
        r6.left = Some(Rc::new(RefCell::new(r9)));
        r4.left = Some(Rc::new(RefCell::new(r7)));
        r3.left = Some(Rc::new(RefCell::new(r5)));
        r3.right = Some(Rc::new(RefCell::new(r6)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::sufficient_subset(Some(Rc::new(RefCell::new(t1))), 22),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(-3);
        let t4 = TreeNode::new(-5);
        let t5 = TreeNode::new(4);
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(1);
        let mut r3 = TreeNode::new(-3);
        let r5 = TreeNode::new(4);
        r3.left = Some(Rc::new(RefCell::new(r5)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::sufficient_subset(Some(Rc::new(RefCell::new(t1))), -1),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(10);
        let t2 = TreeNode::new(5);
        let t3 = TreeNode::new(10);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::sufficient_subset(Some(Rc::new(RefCell::new(t1))), 21),
            None
        );
    }
}
