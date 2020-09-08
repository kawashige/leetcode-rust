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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn collect(
            root: &Option<Rc<RefCell<TreeNode>>>,
            route: Vec<i32>,
            results: &mut Vec<Vec<i32>>,
        ) {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let mut new_route = route.clone();
                    new_route.push(n.val);
                    if n.left.is_none() && n.right.is_none() {
                        results.push(new_route);
                    } else if n.left.is_none() {
                        collect(&n.right, new_route, results);
                    } else if n.right.is_none() {
                        collect(&n.left, new_route, results);
                    } else {
                        collect(&n.left, new_route.clone(), results);
                        collect(&n.right, new_route, results);
                    }
                }
                None => {}
            }
        }

        let mut results = Vec::new();
        collect(&root, Vec::new(), &mut results);
        results
            .into_iter()
            .map(|v| {
                i32::from_str_radix(&v.into_iter().map(|n| n.to_string()).collect::<String>(), 2)
                    .unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day8() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(0);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(0);
        let t5 = TreeNode::new(1);
        let t6 = TreeNode::new(0);
        let t7 = TreeNode::new(1);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            22,
            Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
