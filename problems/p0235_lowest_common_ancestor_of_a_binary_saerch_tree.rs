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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(
            root: Option<Rc<RefCell<TreeNode>>>,
            a: i32,
            b: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = root {
                let n = Rc::try_unwrap(n).unwrap().into_inner();
                if a <= n.val && n.val <= b {
                    Some(Rc::new(RefCell::new(n)))
                } else if b < n.val {
                    recurse(n.left, a, b)
                } else {
                    recurse(n.right, a, b)
                }
            } else {
                None
            }
        }
        let mut a = p.unwrap().borrow().val;
        let mut b = q.unwrap().borrow().val;
        if b < a {
            std::mem::swap(&mut a, &mut b);
        }
        recurse(root, a, b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0235() {
        let mut t1 = TreeNode::new(6);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(8);
        let t4 = TreeNode::new(0);
        let mut t5 = TreeNode::new(4);
        let t6 = TreeNode::new(7);
        let t7 = TreeNode::new(9);
        let t8 = TreeNode::new(3);
        let t9 = TreeNode::new(5);
        t5.left = Some(Rc::new(RefCell::new(t8)));
        t5.right = Some(Rc::new(RefCell::new(t9)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r2 = TreeNode::new(2);
        let r4 = TreeNode::new(0);
        let mut r5 = TreeNode::new(4);
        let r8 = TreeNode::new(3);
        let r9 = TreeNode::new(5);
        r5.left = Some(Rc::new(RefCell::new(r8)));
        r5.right = Some(Rc::new(RefCell::new(r9)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));

        assert_eq!(
            Some(Rc::new(RefCell::new(r2))),
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(t1))),
                Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                Some(Rc::new(RefCell::new(TreeNode::new(3))))
            )
        );
    }
}
