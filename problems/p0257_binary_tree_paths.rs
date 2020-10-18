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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn recurse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            path: Vec<String>,
            results: &mut Vec<Vec<String>>,
        ) {
            if let Some(n) = root {
                let n = n.borrow();
                let mut new_path = path.clone();
                new_path.push(n.val.to_string());
                if n.left.is_none() && n.right.is_none() {
                    results.push(new_path);
                } else {
                    if n.left.is_some() {
                        recurse(&n.left, new_path.clone(), results);
                    }
                    if n.right.is_some() {
                        recurse(&n.right, new_path.clone(), results);
                    }
                }
            }
        }

        let mut results = Vec::new();
        recurse(&root, Vec::new(), &mut results);
        results.into_iter().map(|r| r.join("->")).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0257() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        let t4 = TreeNode::new(5);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec!["1->2->5".to_string(), "1->3".to_string()],
            Solution::binary_tree_paths(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
