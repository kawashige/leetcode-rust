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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn traverse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            depth: usize,
            depths: &mut [[usize; 2]; 501],
        ) -> usize {
            if let Some(n) = root {
                let n = n.borrow();
                depths[n.val as usize][0] = traverse(&n.left, depth + 1, depths);
                depths[n.val as usize][1] = traverse(&n.right, depth + 1, depths);
                std::cmp::max(depths[n.val as usize][0], depths[n.val as usize][1])
            } else {
                depth
            }
        }

        fn search(
            root: Option<Rc<RefCell<TreeNode>>>,
            depths: &[[usize; 2]; 501],
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let n = Rc::try_unwrap(node).unwrap().into_inner();
                if depths[n.val as usize][0] == depths[n.val as usize][1] {
                    Some(Rc::new(RefCell::new(n)))
                } else if depths[n.val as usize][0] < depths[n.val as usize][1] {
                    search(n.right, depths)
                } else {
                    search(n.left, depths)
                }
            } else {
                None
            }
        }

        let mut depths = [[0, 0]; 501];
        traverse(&root, 0, &mut depths);
        search(root, &depths)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day12() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(5);
        let mut t3 = TreeNode::new(1);
        let t4 = TreeNode::new(6);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(0);
        let t7 = TreeNode::new(8);
        let t8 = TreeNode::new(7);
        let t9 = TreeNode::new(4);
        t5.left = Some(Rc::new(RefCell::new(t8)));
        t5.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(2);
        let r2 = TreeNode::new(7);
        let r3 = TreeNode::new(4);
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        assert_eq!(
            Some(Rc::new(RefCell::new(r1))),
            Solution::subtree_with_all_deepest(Some(Rc::new(RefCell::new(t1))))
        );

        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Solution::subtree_with_all_deepest(Some(Rc::new(RefCell::new(TreeNode::new(1)))))
        );

        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(5);
        let t3 = TreeNode::new(1);
        let t4 = TreeNode::new(6);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            Solution::subtree_with_all_deepest(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
