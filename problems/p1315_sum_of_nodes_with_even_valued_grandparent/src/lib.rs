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
    pub fn traverse(
        root: &Option<Rc<RefCell<TreeNode>>>,
        parent: i32,
        grand_parent: i32,
        sum: &mut i32,
    ) {
        if let Some(node) = root {
            let n = node.borrow();
            if grand_parent % 2 == 0 {
                *sum += n.val;
            }
            Self::traverse(&n.left, n.val, parent, sum);
            Self::traverse(&n.right, n.val, parent, sum);
        }
    }

    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::traverse(&root, 1, 1, &mut sum);
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1315() {
        let mut t1 = TreeNode::new(6);
        let mut t2 = TreeNode::new(7);
        let mut t3 = TreeNode::new(8);
        let mut t4 = TreeNode::new(2);
        let mut t5 = TreeNode::new(7);
        let t6 = TreeNode::new(1);
        let mut t7 = TreeNode::new(3);
        let t8 = TreeNode::new(9);
        let t9 = TreeNode::new(1);
        let t10 = TreeNode::new(4);
        let t11 = TreeNode::new(5);
        t7.right = Some(Rc::new(RefCell::new(t11)));
        t5.left = Some(Rc::new(RefCell::new(t9)));
        t5.right = Some(Rc::new(RefCell::new(t10)));
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::sum_even_grandparent(Some(Rc::new(RefCell::new(t1)))),
            18
        );
        assert_eq!(
            Solution::sum_even_grandparent(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            0
        );
    }
}
