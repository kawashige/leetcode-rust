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
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        fn recurse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            voyage: &mut Vec<i32>,
            results: &mut Vec<i32>,
        ) -> bool {
            let n = root.as_ref().unwrap().borrow();

            if n.val != voyage.pop().unwrap() {
                return false;
            }

            match (&n.left, &n.right) {
                (Some(l), Some(_)) if &l.borrow().val == voyage.last().unwrap() => {
                    recurse(&n.left, voyage, results) && recurse(&n.right, voyage, results)
                }
                (Some(_), Some(_)) => {
                    results.push(n.val);
                    recurse(&n.right, voyage, results) && recurse(&n.left, voyage, results)
                }
                (Some(_), None) => recurse(&n.left, voyage, results),
                (None, Some(_)) => recurse(&n.right, voyage, results),
                (None, None) => true,
            }
        }

        let mut results = Vec::with_capacity(voyage.len());
        let mut voyage = voyage.into_iter().rev().collect();
        if recurse(&root, &mut voyage, &mut results) {
            results
        } else {
            vec![-1]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day29() {
        let mut t1_1 = TreeNode::new(1);
        let t1_2 = TreeNode::new(2);
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));

        assert_eq!(
            Solution::flip_match_voyage(Some(Rc::new(RefCell::new(t1_1))), vec![2, 1]),
            vec![-1]
        );

        let mut t2_1 = TreeNode::new(1);
        let t2_2 = TreeNode::new(2);
        let t2_3 = TreeNode::new(3);
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));

        assert_eq!(
            Solution::flip_match_voyage(Some(Rc::new(RefCell::new(t2_1))), vec![1, 3, 2]),
            vec![1]
        );

        let mut t3_1 = TreeNode::new(1);
        let t3_2 = TreeNode::new(2);
        let t3_3 = TreeNode::new(3);
        t3_1.right = Some(Rc::new(RefCell::new(t3_3)));
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));

        assert_eq!(
            Solution::flip_match_voyage(Some(Rc::new(RefCell::new(t3_1))), vec![1, 2, 3]),
            vec![]
        );
    }
}
