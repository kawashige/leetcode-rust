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
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        fn gets(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            match root {
                Some(n) => {
                    let n_inner = Rc::try_unwrap(n).unwrap().into_inner();
                    vec![gets(n_inner.left), vec![n_inner.val], gets(n_inner.right)]
                        .into_iter()
                        .filter(|v| v.len() > 0)
                        .flatten()
                        .collect::<Vec<_>>()
                }
                None => vec![],
            }
        }

        let mut results = gets(root1);
        for n in gets(root2) {
            let pos = results.binary_search(&n).unwrap_or_else(|e| e);
            results.insert(pos, n);
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day5() {
        let mut t1_1 = TreeNode::new(2);
        let t1_2 = TreeNode::new(1);
        let t1_3 = TreeNode::new(4);
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        let mut t2_1 = TreeNode::new(1);
        let t2_2 = TreeNode::new(0);
        let t2_3 = TreeNode::new(3);
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));
        t2_1.right = Some(Rc::new(RefCell::new(t2_3)));

        assert_eq!(
            vec![0, 1, 1, 2, 3, 4],
            Solution::get_all_elements(
                Some(Rc::new(RefCell::new(t1_1))),
                Some(Rc::new(RefCell::new(t2_1)))
            )
        );

        let mut t3_1 = TreeNode::new(1);
        let t3_2 = TreeNode::new(8);
        t3_1.right = Some(Rc::new(RefCell::new(t3_2)));

        let mut t4_1 = TreeNode::new(8);
        let t4_2 = TreeNode::new(1);
        t4_1.left = Some(Rc::new(RefCell::new(t4_2)));

        assert_eq!(
            vec![1, 1, 8, 8],
            Solution::get_all_elements(
                Some(Rc::new(RefCell::new(t3_1))),
                Some(Rc::new(RefCell::new(t4_1)))
            )
        );
    }
}
