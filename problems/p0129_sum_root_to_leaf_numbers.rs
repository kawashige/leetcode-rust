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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurese(root: &Option<Rc<RefCell<TreeNode>>>, path: Vec<i32>, results: &mut Vec<i32>) {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let mut new_path = path.clone();
                    new_path.push(n.val);
                    if n.left.is_none() && n.right.is_none() {
                        results.push(
                            new_path
                                .into_iter()
                                .map(|i| i.to_string())
                                .collect::<String>()
                                .parse()
                                .unwrap(),
                        );
                    } else if n.left.is_none() {
                        recurese(&n.right, new_path, results);
                    } else if n.right.is_none() {
                        recurese(&n.left, new_path, results);
                    } else {
                        recurese(&n.right, new_path.clone(), results);
                        recurese(&n.left, new_path, results);
                    }
                }
                None => {}
            }
        }

        let mut results = vec![0];
        recurese(&root, Vec::new(), &mut results);
        results.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0129() {
        let mut t1 = TreeNode::new(4);
        let mut t2 = TreeNode::new(9);
        let t3 = TreeNode::new(0);
        let t4 = TreeNode::new(5);
        let t5 = TreeNode::new(1);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(1026, Solution::sum_numbers(Some(Rc::new(RefCell::new(t1)))));
        assert_eq!(0, Solution::sum_numbers(None));
    }
}
