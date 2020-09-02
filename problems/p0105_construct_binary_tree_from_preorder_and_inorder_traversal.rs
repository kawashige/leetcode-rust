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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match preorder.len() {
                0 => None,
                _ => {
                    let mut t = TreeNode::new(preorder[0]);
                    for i in 0..inorder.len() {
                        if preorder[0] == inorder[i] {
                            t.left = build(&preorder[1..(i + 1)], &inorder[..i]);
                            t.right = build(&preorder[(i + 1)..], &inorder[(i + 1)..]);
                        }
                    }
                    Some(Rc::new(RefCell::new(t)))
                }
            }
        }

        build(&preorder, &inorder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0105() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let result = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]).unwrap();
        assert_eq!(Rc::new(RefCell::new(t1)), result);

        let result2 = Solution::build_tree(vec![3], vec![3]).unwrap();
        assert_eq!(Rc::new(RefCell::new(TreeNode::new(3))), result2);
    }
}
