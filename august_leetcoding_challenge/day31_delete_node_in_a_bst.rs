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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn find_altenative(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let node = root.unwrap().clone();
            let val = match node.borrow().left {
                Some(ref l) => find_altenative(Some(l.clone())),
                None => node.as_ref().borrow().val,
            };
            val
        }

        fn delete(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                Some(n) => {
                    let n = Rc::try_unwrap(n).unwrap().into_inner();
                    if n.val == key {
                        match (n.left, n.right) {
                            (Some(l), Some(r)) => {
                                let mut new = TreeNode::new(find_altenative(Some(r.clone())));
                                new.right = delete(Some(r), new.val);
                                new.left = Some(l);
                                Some(Rc::new(RefCell::new(new)))
                            }
                            (None, Some(r)) => Some(r),
                            (Some(l), None) => Some(l),
                            (None, None) => None,
                        }
                    } else if key < n.val {
                        let mut new = TreeNode::new(n.val);
                        new.right = n.right;
                        new.left = delete(n.left, key);
                        Some(Rc::new(RefCell::new(new)))
                    } else {
                        let mut new = TreeNode::new(n.val);
                        new.right = delete(n.right, key);
                        new.left = n.left;
                        Some(Rc::new(RefCell::new(new)))
                    }
                }
                None => None,
            }
        }
        delete(root, key)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day31() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(6);
        let t4 = TreeNode::new(2);
        let t5 = TreeNode::new(4);
        let t6 = TreeNode::new(7);
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(5);
        let mut r2 = TreeNode::new(4);
        let mut r3 = TreeNode::new(6);
        let r4 = TreeNode::new(2);
        let r5 = TreeNode::new(7);
        r3.right = Some(Rc::new(RefCell::new(r5)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        let result = Solution::delete_node(Some(Rc::new(RefCell::new(t1))), 3).unwrap();
        assert_eq!(r1, Rc::try_unwrap(result).unwrap().into_inner());
    }
}
