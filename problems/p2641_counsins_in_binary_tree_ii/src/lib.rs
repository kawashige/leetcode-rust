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
    pub fn depth_sum(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, sum: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            if sum.len() == depth {
                sum.push(0);
            }
            sum[depth] += node.val;
            Self::depth_sum(&node.left, depth + 1, sum);
            Self::depth_sum(&node.right, depth + 1, sum);
        }
    }

    pub fn recurse(
        node: &mut Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        brother_sum: i32,
        sum: &Vec<i32>,
    ) {
        if let Some(node) = node {
            let mut node = node.borrow_mut();
            node.val = sum[depth] - node.val - brother_sum;

            let right = node.right.as_ref().map_or(0, |n| n.borrow().val);
            let left = node.left.as_ref().map_or(0, |n| n.borrow().val);

            Self::recurse(&mut node.left, depth + 1, right, sum);
            Self::recurse(&mut node.right, depth + 1, left, sum);
        }
    }

    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = vec![];
        Self::depth_sum(&root, 0, &mut sum);

        let mut root = root;
        Self::recurse(&mut root, 0, 0, &sum);
        root
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2641() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(9);
        let t4 = TreeNode::new(1);
        let t5 = TreeNode::new(10);
        let t6 = TreeNode::new(7);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(0);
        let mut r2 = TreeNode::new(0);
        let mut r3 = TreeNode::new(0);
        let r4 = TreeNode::new(7);
        let r5 = TreeNode::new(7);
        let r6 = TreeNode::new(11);
        r3.right = Some(Rc::new(RefCell::new(r6)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::replace_value_in_tree(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );

        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(1);
        let t3 = TreeNode::new(2);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(0);
        let r2 = TreeNode::new(0);
        let r3 = TreeNode::new(0);
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Solution::replace_value_in_tree(Some(Rc::new(RefCell::new(t1)))),
            Some(Rc::new(RefCell::new(r1)))
        );
    }
}
