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
    pub fn collect(root: &Option<Rc<RefCell<TreeNode>>>, a: &mut Vec<i32>) {
        if let Some(node) = root {
            let n = node.borrow();
            Self::collect(&n.left, a);
            a.push(n.val);
            Self::collect(&n.right, a);
        }
    }

    pub fn construct(a: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if a.is_empty() {
            return None;
        }

        let i = (0..a.len()).max_by_key(|i| a[*i]).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: a[i],
            left: Self::construct(&a[..i]),
            right: Self::construct(&a[(i + 1)..]),
        })))
    }

    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut a = Vec::new();
        Self::collect(&root, &mut a);
        a.push(val);
        Self::construct(&a)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0998() {
        let mut t1 = TreeNode::new(4);
        let t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(2);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(5);
        let mut r2 = TreeNode::new(4);
        let r3 = TreeNode::new(1);
        let mut r4 = TreeNode::new(3);
        let r5 = TreeNode::new(2);
        r4.left = Some(Rc::new(RefCell::new(r5)));
        r2.left = Some(Rc::new(RefCell::new(r3)));
        r2.right = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));

        assert_eq!(
            Solution::insert_into_max_tree(Some(Rc::new(RefCell::new(t1))), 5),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(4);
        let t4 = TreeNode::new(1);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(5);
        let mut r2 = TreeNode::new(2);
        let mut r3 = TreeNode::new(4);
        let r4 = TreeNode::new(1);
        let r5 = TreeNode::new(3);
        r3.right = Some(Rc::new(RefCell::new(r5)));
        r2.right = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::insert_into_max_tree(Some(Rc::new(RefCell::new(t1))), 3),
            Some(Rc::new(RefCell::new(r1)))
        );
    }
}
