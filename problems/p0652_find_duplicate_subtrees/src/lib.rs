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
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};
pub struct Solution {}

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn recurse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            occurence: &mut HashMap<String, usize>,
            results: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> String {
            if let Some(node) = root {
                let n = node.borrow();
                let key = format!(
                    "{},{},{}",
                    n.val,
                    recurse(&n.left, occurence, results),
                    recurse(&n.right, occurence, results)
                );
                *occurence.entry(key.clone()).or_insert(0) += 1;
                if occurence[&key] == 2 {
                    results.push(node.clone().into());
                }
                key
            } else {
                "".to_string()
            }
        }

        let mut occurence = HashMap::new();
        let mut results = Vec::new();
        recurse(&root, &mut occurence, &mut results);
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0652() {
        let mut t1_1 = TreeNode::new(1);
        let mut t1_2 = TreeNode::new(2);
        let mut t1_3 = TreeNode::new(3);
        let t1_4 = TreeNode::new(4);
        let mut t1_5 = TreeNode::new(2);
        let t1_6 = TreeNode::new(4);
        let t1_7 = TreeNode::new(4);
        t1_5.left = Some(Rc::new(RefCell::new(t1_7)));
        t1_3.left = Some(Rc::new(RefCell::new(t1_5)));
        t1_3.right = Some(Rc::new(RefCell::new(t1_6)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut r1_1 = TreeNode::new(2);
        let r1_2 = TreeNode::new(4);
        r1_1.left = Some(Rc::new(RefCell::new(r1_2)));
        let r1_3 = TreeNode::new(4);

        assert_eq!(
            Solution::find_duplicate_subtrees(Some(Rc::new(RefCell::new(t1_1)))),
            vec![
                Some(Rc::new(RefCell::new(r1_3))),
                Some(Rc::new(RefCell::new(r1_1)))
            ]
        );
    }
}
