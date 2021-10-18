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
    pub fn collect(
        root: &Option<Rc<RefCell<TreeNode>>>,
        parent: i32,
        depth: i32,
        parents: &mut Vec<i32>,
        depths: &mut Vec<i32>,
    ) {
        if let Some(node) = root {
            let n = node.borrow();
            parents[n.val as usize] = parent;
            depths[n.val as usize] = depth;
            Self::collect(&n.left, n.val, depth + 1, parents, depths);
            Self::collect(&n.right, n.val, depth + 1, parents, depths);
        }
    }

    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut parents = vec![0; 101];
        let mut depths = vec![0; 101];

        Self::collect(&root, -1, 0, &mut parents, &mut depths);

        parents[x as usize] != parents[y as usize] && depths[x as usize] == depths[y as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0993() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::is_cousins(Some(Rc::new(RefCell::new(t1))), 4, 3));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        let t5 = TreeNode::new(5);
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(Solution::is_cousins(Some(Rc::new(RefCell::new(t1))), 5, 4));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert!(!Solution::is_cousins(Some(Rc::new(RefCell::new(t1))), 2, 3));
    }
}
