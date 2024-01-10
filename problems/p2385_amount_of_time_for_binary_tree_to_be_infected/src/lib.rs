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
use std::collections::VecDeque;
use std::rc::Rc;
pub struct Solution {}

impl Solution {
    pub fn recurse(
        node: &Option<Rc<RefCell<TreeNode>>>,
        parent: usize,
        edge: &mut Vec<Vec<usize>>,
    ) {
        if let Some(node) = node {
            let n = node.borrow();
            if parent != 0 {
                edge[parent].push(n.val as usize);
                edge[n.val as usize].push(parent);
            }
            Self::recurse(&n.left, n.val as usize, edge);
            Self::recurse(&n.right, n.val as usize, edge);
        }
    }

    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut edge = vec![vec![]; 100_001];
        Self::recurse(&root, 0, &mut edge);

        let mut queue = VecDeque::new();
        queue.push_back((start as usize, 0));
        let mut seen = vec![false; 100_001];
        let mut max = 0;

        while let Some((node, time)) = queue.pop_front() {
            if seen[node] {
                continue;
            }
            seen[node] = true;
            max = max.max(time);

            for next in &edge[node] {
                queue.push_back((*next, time + 1));
            }
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2385() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(3);
        let mut t4 = TreeNode::new(4);
        let t5 = TreeNode::new(10);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(9);
        let t8 = TreeNode::new(2);
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::amount_of_time(Some(Rc::new(RefCell::new(t1))), 3),
            4
        );

        let t1 = TreeNode::new(1);

        assert_eq!(
            Solution::amount_of_time(Some(Rc::new(RefCell::new(t1))), 3),
            0
        );
    }
}
