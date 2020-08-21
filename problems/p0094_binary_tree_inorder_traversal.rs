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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
            match root {
                Some(node) => {
                    let n = Rc::try_unwrap(node).unwrap().into_inner();
                    vec![traverse(n.left), vec![n.val], traverse(n.right)]
                        .into_iter()
                        .filter(|x| !x.is_empty())
                        .flatten()
                        .collect::<Vec<i32>>()
                }
                None => vec![],
            }
        }
        traverse(root)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0094() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let t3 = TreeNode::new(3);
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            vec![1, 3, 2],
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
