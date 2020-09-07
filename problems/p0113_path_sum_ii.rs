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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        fn sums(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, route: Vec<i32>) -> Vec<Vec<i32>> {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let mut new_route = route.clone();
                    new_route.push(n.val);
                    if n.left.is_none() && n.right.is_none() {
                        if new_route.iter().sum::<i32>() == sum {
                            vec![new_route]
                        } else {
                            vec![]
                        }
                    } else if n.left.is_none() {
                        sums(&n.right, sum, new_route)
                    } else if n.right.is_none() {
                        sums(&n.left, sum, new_route)
                    } else {
                        sums(&n.left, sum, new_route.clone())
                            .into_iter()
                            .chain(sums(&n.right, sum, new_route))
                            .collect::<Vec<Vec<i32>>>()
                    }
                }
                None => Vec::new(),
            }
        }
        if root.is_none() {
            return vec![];
        }

        sums(&root, sum, vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0113() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(8);
        let mut t4 = TreeNode::new(11);
        let t5 = TreeNode::new(13);
        let mut t6 = TreeNode::new(4);
        let t7 = TreeNode::new(7);
        let t8 = TreeNode::new(2);
        let t9 = TreeNode::new(5);
        let t10 = TreeNode::new(1);
        t6.right = Some(Rc::new(RefCell::new(t10)));
        t6.left = Some(Rc::new(RefCell::new(t9)));
        t4.right = Some(Rc::new(RefCell::new(t8)));
        t4.left = Some(Rc::new(RefCell::new(t7)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]],
            Solution::path_sum(Some(Rc::new(RefCell::new(t1))), 22)
        );
    }
}
