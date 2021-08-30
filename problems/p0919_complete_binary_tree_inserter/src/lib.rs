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

struct CBTInserter {
    nodes: usize,
    root: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let nodes = Self::count(&root);
        Self { nodes, root }
    }

    fn count(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(node) = root {
            let n = node.borrow();
            1 + Self::count(&n.left) + Self::count(&n.right)
        } else {
            0
        }
    }

    fn insert(&mut self, val: i32) -> i32 {
        self.nodes += 1;

        let mut routes = vec![self.nodes];
        while routes.last().unwrap() > &1 {
            routes.push(routes.last().unwrap() / 2);
        }
        routes.pop();

        Self::insert_recurse(&mut self.root, val, &mut routes)
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }

    fn insert_recurse(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        routes: &mut Vec<usize>,
    ) -> i32 {
        if let Some(node) = root {
            let mut n = node.borrow_mut();
            if let Some(route) = routes.pop() {
                if routes.is_empty() {
                    if route % 2 == 0 {
                        n.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    } else {
                        n.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    }
                    n.val
                } else {
                    if route % 2 == 0 {
                        Self::insert_recurse(&mut n.left, val, routes)
                    } else {
                        Self::insert_recurse(&mut n.right, val, routes)
                    }
                }
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0919() {
        let mut t1_1 = TreeNode::new(1);
        let t1_2 = TreeNode::new(2);
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));

        let mut obj = CBTInserter::new(Some(Rc::new(RefCell::new(t1_1))));
        assert_eq!(obj.insert(3), 1);
        assert_eq!(obj.insert(4), 2);

        let mut r1_1 = TreeNode::new(1);
        let mut r1_2 = TreeNode::new(2);
        let r1_3 = TreeNode::new(3);
        let r1_4 = TreeNode::new(4);
        r1_2.left = Some(Rc::new(RefCell::new(r1_4)));
        r1_1.left = Some(Rc::new(RefCell::new(r1_2)));
        r1_1.right = Some(Rc::new(RefCell::new(r1_3)));

        assert_eq!(obj.get_root(), Some(Rc::new(RefCell::new(r1_1))));
    }
}
