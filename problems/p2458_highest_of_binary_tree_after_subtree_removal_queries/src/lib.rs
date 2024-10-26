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
pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recurse(
        node: &Option<Rc<RefCell<TreeNode>>>,
        d: usize,
        depth: &mut Vec<usize>,
        level: &mut Vec<Vec<(usize, i32)>>,
    ) -> usize {
        if let Some(node) = node {
            let node = node.borrow();
            if level.len() < d + 1 {
                level.push(vec![]);
            }
            depth[node.val as usize] = d;
            let val = Self::recurse(&node.left, d + 1, depth, level).max(Self::recurse(
                &node.right,
                d + 1,
                depth,
                level,
            ));
            level[d].push((val, node.val));
            level[d].sort_unstable_by(|a, b| b.0.cmp(&a.0));
            if level[d].len() == 3 {
                level[d].pop();
            }

            val
        } else {
            d - 1
        }
    }

    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut depth = vec![0; 100_001];
        let mut level = vec![];

        Self::recurse(&root, 0, &mut depth, &mut level);

        queries
            .into_iter()
            .map(|q| {
                (if level[depth[q as usize]].len() == 1 {
                    depth[q as usize] - 1
                } else if level[depth[q as usize]][0].1 == q {
                    level[depth[q as usize]][1].0
                } else {
                    level[depth[q as usize]][0].0
                }) as i32
            })
            .collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2458() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(2);
        let t5 = TreeNode::new(4);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::tree_queries(Some(Rc::new(RefCell::new(t1))), vec![3, 5, 4, 2, 4]),
            vec![1, 0, 3, 3, 3]
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(4);
        let t4 = TreeNode::new(2);
        let t5 = TreeNode::new(6);
        let mut t6 = TreeNode::new(5);
        let t7 = TreeNode::new(7);
        t6.right = Some(Rc::new(RefCell::new(t7)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::tree_queries(Some(Rc::new(RefCell::new(t1))), vec![4]),
            vec![2]
        );

        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(8);
        let mut t3 = TreeNode::new(9);
        let mut t4 = TreeNode::new(2);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(3);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(4);
        let t9 = TreeNode::new(6);
        t4.left = Some(Rc::new(RefCell::new(t8)));
        t4.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::tree_queries(Some(Rc::new(RefCell::new(t1))), vec![3, 2, 4, 8]),
            vec![3, 2, 3, 2]
        );
    }
}
