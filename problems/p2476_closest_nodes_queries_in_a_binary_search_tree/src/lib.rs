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
    pub fn recurse(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::recurse(&node.left, values);
            values.push(node.val);
            Self::recurse(&node.right, values);
        }
    }
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut values = Vec::new();
        Self::recurse(&root, &mut values);

        queries
            .into_iter()
            .map(|q| match values.binary_search(&q) {
                Ok(_) => vec![q, q],
                Err(i) => {
                    vec![
                        if i == 0 { -1 } else { values[i - 1] },
                        if i == values.len() { -1 } else { values[i] },
                    ]
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2476() {
        let mut t1_1 = TreeNode::new(6);
        let mut t1_2 = TreeNode::new(2);
        let mut t1_3 = TreeNode::new(13);
        let t1_4 = TreeNode::new(1);
        let t1_5 = TreeNode::new(4);
        let t1_6 = TreeNode::new(9);
        let mut t1_7 = TreeNode::new(15);
        let t1_8 = TreeNode::new(14);
        t1_7.left = Some(Rc::new(RefCell::new(t1_8)));
        t1_3.left = Some(Rc::new(RefCell::new(t1_6)));
        t1_3.right = Some(Rc::new(RefCell::new(t1_7)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert_eq!(
            Solution::closest_nodes(Some(Rc::new(RefCell::new(t1_1))), vec![2, 5, 16]),
            vec![vec![2, 2], vec![4, 6], vec![15, -1]]
        );

        let mut t2_1 = TreeNode::new(4);
        let t2_2 = TreeNode::new(9);
        t2_1.right = Some(Rc::new(RefCell::new(t2_2)));

        assert_eq!(
            Solution::closest_nodes(Some(Rc::new(RefCell::new(t2_1))), vec![3]),
            vec![vec![-1, 4]]
        );
    }
}
