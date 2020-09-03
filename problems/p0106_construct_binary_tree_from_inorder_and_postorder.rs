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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match inorder.len() {
                0 => None,
                _ => {
                    let mut t = TreeNode::new(postorder[postorder.len() - 1]);
                    for i in 0..inorder.len() {
                        if t.val == inorder[i] {
                            t.left = build(&inorder[..i], &postorder[..i]);
                            t.right =
                                build(&inorder[(i + 1)..], &postorder[i..(postorder.len() - 1)]);
                        }
                    }
                    Some(Rc::new(RefCell::new(t)))
                }
            }
        }

        build(&inorder, &postorder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0106() {
        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(9);
        let mut t3 = TreeNode::new(20);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t3.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let result = Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]).unwrap();
        assert_eq!(Rc::new(RefCell::new(t1)), result);

        let result2 = Solution::build_tree(vec![3], vec![3]).unwrap();
        assert_eq!(Rc::new(RefCell::new(TreeNode::new(3))), result2);
    }
}
