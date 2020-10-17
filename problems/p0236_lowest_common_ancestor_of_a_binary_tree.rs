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
            root: &Option<Rc<RefCell<TreeNode>>>,
            a: i32,
            b: i32,
            found: &mut usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                Some(n) => {
                    let n = n.borrow();
                    let mut count = 0;
                    let left = recurse(&n.left, a, b, &mut count);
                    if left.is_some() {
                        return left;
                    }
                    let right = recurse(&n.right, a, b, &mut count);
                    if right.is_some() {
                        return right;
                    }
                    if n.val == a || n.val == b {
                        count += 1;
                    }
                    if count == 2 {
                        return Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
                    }
                    *found += count;
                }
                None => {}
            }
            None
        }

        let a = p.unwrap().borrow().val;
        let b = q.unwrap().borrow().val;
        recurse(&root, a, b, &mut 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0236() {
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

        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(t1))),
                Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                Some(Rc::new(RefCell::new(TreeNode::new(4))))
            )
        );
    }
}
