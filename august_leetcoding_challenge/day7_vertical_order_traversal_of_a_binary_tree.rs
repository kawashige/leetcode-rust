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

use std::collections::BTreeMap;
impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn traverse(
            node: Option<Rc<RefCell<TreeNode>>>,
            x: i32,
            y: i32,
            result: &mut BTreeMap<i32, BTreeMap<i32, Vec<i32>>>,
        ) {
            match node {
                Some(n) => {
                    let n_inner = Rc::try_unwrap(n).unwrap().into_inner();
                    let current_x = result.entry(x).or_insert(BTreeMap::new());
                    let current_x_y = current_x.entry(y).or_insert(Vec::new());
                    let pos = current_x_y
                        .binary_search(&n_inner.val)
                        .unwrap_or_else(|e| e);
                    current_x_y.insert(pos, n_inner.val);
                    traverse(n_inner.left, x - 1, y + 1, result);
                    traverse(n_inner.right, x + 1, y + 1, result);
                }
                None => {}
            }
        }

        let mut result = BTreeMap::new();
        traverse(root, 0, 0, &mut result);
        result
            .values()
            .map(|v| v.values().cloned().collect::<Vec<Vec<i32>>>().concat())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day7() {
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
            vec![vec![9], vec![3, 15], vec![20], vec![7]],
            Solution::vertical_traversal(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
