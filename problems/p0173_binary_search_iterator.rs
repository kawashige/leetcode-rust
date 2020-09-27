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
pub struct BSTIterator {
    values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn collect(root: &Option<Rc<RefCell<TreeNode>>>, results: &mut Vec<i32>) {
        match root {
            Some(n) => {
                let n = n.borrow();
                Self::collect(&n.right, results);
                results.push(n.val);
                Self::collect(&n.left, results);
            }
            None => {}
        }
    }

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = Vec::new();
        Self::collect(&root, &mut values);
        Self { values }
    }

    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        self.values.pop().unwrap()
    }

    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        !self.values.is_empty()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0173() {
        let mut t1 = TreeNode::new(7);
        let t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(15);
        let t4 = TreeNode::new(9);
        let t5 = TreeNode::new(20);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut obj = BSTIterator::new(Some(Rc::new(RefCell::new(t1))));
        assert_eq!(3, obj.next());
        assert_eq!(7, obj.next());
        assert!(obj.has_next());
        assert_eq!(9, obj.next());
        assert!(obj.has_next());
        assert_eq!(15, obj.next());
        assert!(obj.has_next());
        assert_eq!(20, obj.next());
        assert!(!obj.has_next());
    }
}
