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
    pub fn collect(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<Vec<usize>>) -> i32 {
        if let Some(n) = root {
            let n = n.borrow();
            let left = Self::collect(&n.left, list);
            if left > -1 {
                list[n.val as usize].push(left as usize);
                list[left as usize].push(n.val as usize);
            }

            let right = Self::collect(&n.right, list);
            if right > -1 {
                list[n.val as usize].push(right as usize);
                list[right as usize].push(n.val as usize);
            }

            n.val
        } else {
            -1
        }
    }

    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut list = vec![vec![]; 501];
        Self::collect(&root, &mut list);

        let target = target.unwrap().borrow().val;
        let mut result = Vec::new();
        let mut stack = vec![(target as usize, 0)];
        let mut seen = vec![false; 501];

        while let Some((n, d)) = stack.pop() {
            seen[n] = true;

            if d == k {
                result.push(n as i32);
            }

            if d < k {
                for next in &list[n] {
                    if seen[*next] {
                        continue;
                    }
                    stack.push((*next, d + 1));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0863() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(6);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(0);
        let t7 = TreeNode::new(8);
        let t8 = TreeNode::new(7);
        let t9 = TreeNode::new(4);
        t5.left = Some(Rc::new(RefCell::new(t8)));
        t5.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::distance_k(
                Some(Rc::new(RefCell::new(t1))),
                Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                2
            ),
            vec![1, 4, 7]
        );
    }
}
