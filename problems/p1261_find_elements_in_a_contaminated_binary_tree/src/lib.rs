use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

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
pub struct FindElements {
    values: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut values = HashSet::new();
        FindElements::traverse(&root, 0, &mut values);
        Self { values }
    }

    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, values: &mut HashSet<i32>) {
        if let Some(node) = root {
            values.insert(val);
            let n = node.borrow();
            FindElements::traverse(&n.left, val * 2 + 1, values);
            FindElements::traverse(&n.right, val * 2 + 2, values);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1261() {
        let mut t1 = TreeNode::new(-1);
        let t2 = TreeNode::new(-1);
        t1.right = Some(Rc::new(RefCell::new(t2)));
        let obj = FindElements::new(Some(Rc::new(RefCell::new(t1))));
        assert!(!obj.find(1));
        assert!(obj.find(2));
    }
}
