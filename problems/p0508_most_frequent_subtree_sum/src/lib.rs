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
use std::collections::HashMap;
use std::rc::Rc;
pub struct Solution {}

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, counts: &mut HashMap<i32, i32>) -> i32 {
            if let Some(node) = root {
                let n = node.borrow();
                let num = n.val + recurse(&n.left, counts) + recurse(&n.right, counts);
                (*counts.entry(num).or_insert(0)) += 1;
                num
            } else {
                0
            }
        }

        if root.is_none() {
            return Vec::new();
        }

        let mut counts = HashMap::new();
        recurse(&root, &mut counts);
        let max = *counts.values().max().unwrap();
        counts
            .into_iter()
            .filter(|(_, v)| v == &max)
            .map(|(k, _)| k)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0508() {
        let mut t1 = TreeNode::new(5);
        let t2 = TreeNode::new(2);
        let t3 = TreeNode::new(-3);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        assert_eq!(
            vec![2, -3, 4],
            Solution::find_frequent_tree_sum(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(5);
        let t2 = TreeNode::new(2);
        let t3 = TreeNode::new(-5);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));
        assert_eq!(
            vec![2],
            Solution::find_frequent_tree_sum(Some(Rc::new(RefCell::new(t1))))
        );

        assert_eq!(vec![] as Vec<i32>, Solution::find_frequent_tree_sum(None));
    }
}
