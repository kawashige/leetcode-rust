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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, results: &mut Vec<i32>) {
            if let Some(node) = root {
                let n = node.borrow();
                if results.len() < depth {
                    results.push(n.val);
                } else if results[depth - 1] < n.val {
                    results[depth - 1] = n.val;
                }
                recurse(&n.left, depth + 1, results);
                recurse(&n.right, depth + 1, results);
            }
        }

        let mut results = Vec::new();
        recurse(&root, 1, &mut results);
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0515() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(2);
        let t4 = TreeNode::new(5);
        let t5 = TreeNode::new(3);
        let t6 = TreeNode::new(9);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec![1, 3, 9],
            Solution::largest_values(Some(Rc::new(RefCell::new(t1))))
        );

        let mut t1 = TreeNode::new(1);
        let t2 = TreeNode::new(3);
        let t3 = TreeNode::new(2);
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec![1, 3],
            Solution::largest_values(Some(Rc::new(RefCell::new(t1))))
        );

        assert_eq!(
            vec![1],
            Solution::largest_values(Some(Rc::new(RefCell::new(TreeNode::new(1)))))
        );

        assert_eq!(vec![] as Vec<i32>, Solution::largest_values(None));
    }
}
