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
    pub fn recurse(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, values: &mut Vec<Vec<i32>>) {
        if let Some(node) = node {
            let node = node.borrow();
            if values.len() < level + 1 {
                values.push(Vec::new());
            }
            values[level].push(node.val);
            Self::recurse(&node.left, level + 1, values);
            Self::recurse(&node.right, level + 1, values);
        }
    }
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values = Vec::new();
        Self::recurse(&root, 0, &mut values);

        let mut count = 0;
        for i in 0..values.len() {
            for j in 0..values[i].len() {
                let mut min_index = values[i].len() - 1;
                let mut min = values[i][min_index];
                for k in (j..values[i].len() - 1).rev() {
                    if values[i][k] < min {
                        min = values[i][k];
                        min_index = k;
                    }
                }
                if j != min_index {
                    count += 1;
                    values[i][min_index] = values[i][j];
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2471() {
        let mut t1_1 = TreeNode::new(1);
        let mut t1_2 = TreeNode::new(4);
        let mut t1_3 = TreeNode::new(3);
        let t1_4 = TreeNode::new(7);
        let t1_5 = TreeNode::new(6);
        let mut t1_6 = TreeNode::new(8);
        let mut t1_7 = TreeNode::new(5);
        let t1_8 = TreeNode::new(9);
        let t1_9 = TreeNode::new(10);
        t1_7.left = Some(Rc::new(RefCell::new(t1_9)));
        t1_6.left = Some(Rc::new(RefCell::new(t1_8)));
        t1_3.left = Some(Rc::new(RefCell::new(t1_6)));
        t1_3.right = Some(Rc::new(RefCell::new(t1_7)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert_eq!(
            Solution::minimum_operations(Some(Rc::new(RefCell::new(t1_1)))),
            3
        );

        let mut t2_1 = TreeNode::new(1);
        let mut t2_2 = TreeNode::new(3);
        let mut t2_3 = TreeNode::new(2);
        let t2_4 = TreeNode::new(7);
        let t2_5 = TreeNode::new(6);
        let t2_6 = TreeNode::new(5);
        let t2_7 = TreeNode::new(4);
        t2_3.left = Some(Rc::new(RefCell::new(t2_6)));
        t2_3.right = Some(Rc::new(RefCell::new(t2_7)));
        t2_2.left = Some(Rc::new(RefCell::new(t2_4)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_5)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert_eq!(
            Solution::minimum_operations(Some(Rc::new(RefCell::new(t2_1)))),
            3
        );

        let mut t3_1 = TreeNode::new(1);
        let mut t3_2 = TreeNode::new(2);
        let mut t3_3 = TreeNode::new(3);
        let t3_4 = TreeNode::new(4);
        let t3_5 = TreeNode::new(5);
        let t3_6 = TreeNode::new(6);
        t3_3.left = Some(Rc::new(RefCell::new(t3_6)));
        t3_2.left = Some(Rc::new(RefCell::new(t3_4)));
        t3_2.right = Some(Rc::new(RefCell::new(t3_5)));
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));
        t3_1.right = Some(Rc::new(RefCell::new(t3_3)));

        assert_eq!(
            Solution::minimum_operations(Some(Rc::new(RefCell::new(t3_1)))),
            0
        );
    }
}
