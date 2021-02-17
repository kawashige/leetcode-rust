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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        fn find_depth(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize) -> usize {
            if let Some(node) = root {
                let n = node.borrow();
                std::cmp::max(
                    find_depth(&n.left, depth + 1),
                    find_depth(&n.right, depth + 1),
                )
            } else {
                depth
            }
        }

        fn print_tree(
            root: &Option<Rc<RefCell<TreeNode>>>,
            results: &mut Vec<Vec<String>>,
            row: usize,
            s: usize,
            e: usize,
        ) {
            if let Some(node) = root {
                let n = node.borrow();
                let column = s + (e - s) / 2;
                results[row][column] = n.val.to_string();
                if n.left.is_some() {
                    print_tree(&n.left, results, row + 1, s, column - 1);
                }
                if n.right.is_some() {
                    print_tree(&n.right, results, row + 1, column + 1, e);
                }
            }
        }

        let m = find_depth(&root, 0);
        let n = 2_usize.pow(m as u32) - 1;
        let mut results = vec![vec![String::new(); n]; m];
        print_tree(&root, &mut results, 0, 0, n);
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0655() {
        let mut t1_1 = TreeNode::new(1);
        let t1_2 = TreeNode::new(2);
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));

        assert_eq!(
            Solution::print_tree(Some(Rc::new(RefCell::new(t1_1)))),
            vec![
                vec!["".to_string(), "1".to_string(), "".to_string()],
                vec!["2".to_string(), "".to_string(), "".to_string()]
            ]
        );

        let mut t2_1 = TreeNode::new(1);
        let mut t2_2 = TreeNode::new(2);
        let t2_3 = TreeNode::new(3);
        let t2_4 = TreeNode::new(4);
        t2_2.right = Some(Rc::new(RefCell::new(t2_4)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert_eq!(
            Solution::print_tree(Some(Rc::new(RefCell::new(t2_1)))),
            vec![
                vec![
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "1".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                ],
                vec![
                    "".to_string(),
                    "2".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "3".to_string(),
                    "".to_string()
                ],
                vec![
                    "".to_string(),
                    "".to_string(),
                    "4".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                ]
            ]
        );

        let mut t3_1 = TreeNode::new(1);
        let mut t3_2 = TreeNode::new(2);
        let t3_3 = TreeNode::new(5);
        let mut t3_4 = TreeNode::new(3);
        let t3_5 = TreeNode::new(4);
        t3_4.left = Some(Rc::new(RefCell::new(t3_5)));
        t3_2.left = Some(Rc::new(RefCell::new(t3_4)));
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));
        t3_1.right = Some(Rc::new(RefCell::new(t3_3)));

        assert_eq!(
            Solution::print_tree(Some(Rc::new(RefCell::new(t3_1)))),
            vec![
                vec![
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "1".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                ],
                vec![
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "2".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "5".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                ],
                vec![
                    "".to_string(),
                    "3".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                ],
                vec![
                    "4".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                ],
            ]
        );
    }
}
