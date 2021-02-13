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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, values: &mut Vec<Vec<i64>>) {
            if let Some(node) = root {
                let n = node.borrow();
                if values.len() <= level {
                    values.push(Vec::new());
                }
                values[level].push(n.val as i64);
                recurse(&n.left, level + 1, values);
                recurse(&n.right, level + 1, values);
            }
        }
        let mut values = Vec::new();
        recurse(&root, 0, &mut values);
        values
            .into_iter()
            .map(|v| v.iter().sum::<i64>() as f64 / v.len() as f64)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0637() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::average_of_levels(Some(Rc::new(RefCell::new(t1)))),
            vec![3.0, 14.5, 11.0]
        );
        assert_eq!(Solution::average_of_levels(None), vec![] as Vec<f64>);
    }
}
