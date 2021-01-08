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

use std::collections::HashMap;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, count: &mut HashMap<i32, usize>) {
            if let Some(node) = root {
                let n = node.borrow();
                (*count.entry(n.val).or_insert(0)) += 1;
                recurse(&n.left, count);
                recurse(&n.right, count);
            }
        }
        let mut count = HashMap::new();
        recurse(&root, &mut count);
        if count.is_empty() {
            return Vec::new();
        }
        let max = *count.values().max().unwrap();
        count
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
    fn test_0501() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(2);
        t2.right = Some(Rc::new(RefCell::new(t3)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            vec![2],
            Solution::find_mode(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(2);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec![3, 2],
            Solution::find_mode(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(2);
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            vec![2, 1],
            Solution::find_mode(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
