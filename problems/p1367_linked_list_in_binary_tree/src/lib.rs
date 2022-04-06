// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn check(
        head: &Option<Box<ListNode>>,
        current_head: &Option<Box<ListNode>>,
        root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            if let Some(h) = current_head {
                if h.val == n.val
                    && (Self::check(&h.next, &h.next, &n.left)
                        || Self::check(&h.next, &h.next, &n.right))
                {
                    return true;
                }
            } else {
                return true;
            }
            Self::check(head, head, &n.left) || Self::check(head, head, &n.right)
        } else {
            current_head.is_none()
        }
    }

    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check(&head, &head, &root)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1367() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(4);
        let mut t4 = TreeNode::new(2);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(1);
        let t7 = TreeNode::new(6);
        let mut t8 = TreeNode::new(8);
        let t9 = TreeNode::new(1);
        let t10 = TreeNode::new(3);
        t8.left = Some(Rc::new(RefCell::new(t9)));
        t8.right = Some(Rc::new(RefCell::new(t10)));
        t5.left = Some(Rc::new(RefCell::new(t7)));
        t5.right = Some(Rc::new(RefCell::new(t8)));
        t4.left = Some(Rc::new(RefCell::new(t6)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut t1_1 = ListNode::new(4);
        let mut t1_2 = ListNode::new(2);
        let t1_3 = ListNode::new(8);
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert!(Solution::is_sub_path(
            Some(Box::new(t1_1)),
            Some(Rc::new(RefCell::new(t1)))
        ));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(4);
        let mut t4 = TreeNode::new(2);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(1);
        let t7 = TreeNode::new(6);
        let mut t8 = TreeNode::new(8);
        let t9 = TreeNode::new(1);
        let t10 = TreeNode::new(3);
        t8.left = Some(Rc::new(RefCell::new(t9)));
        t8.right = Some(Rc::new(RefCell::new(t10)));
        t5.left = Some(Rc::new(RefCell::new(t7)));
        t5.right = Some(Rc::new(RefCell::new(t8)));
        t4.left = Some(Rc::new(RefCell::new(t6)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(4);
        let mut t1_3 = ListNode::new(2);
        let t1_4 = ListNode::new(6);
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert!(Solution::is_sub_path(
            Some(Box::new(t1_1)),
            Some(Rc::new(RefCell::new(t1)))
        ));

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(4);
        let mut t3 = TreeNode::new(4);
        let mut t4 = TreeNode::new(2);
        let mut t5 = TreeNode::new(2);
        let t6 = TreeNode::new(1);
        let t7 = TreeNode::new(6);
        let mut t8 = TreeNode::new(8);
        let t9 = TreeNode::new(1);
        let t10 = TreeNode::new(3);
        t8.left = Some(Rc::new(RefCell::new(t9)));
        t8.right = Some(Rc::new(RefCell::new(t10)));
        t5.left = Some(Rc::new(RefCell::new(t7)));
        t5.right = Some(Rc::new(RefCell::new(t8)));
        t4.left = Some(Rc::new(RefCell::new(t6)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut t1_1 = ListNode::new(1);
        let mut t1_2 = ListNode::new(4);
        let mut t1_3 = ListNode::new(2);
        let mut t1_4 = ListNode::new(6);
        let t1_5 = ListNode::new(8);
        t1_4.next = Some(Box::new(t1_5));
        t1_3.next = Some(Box::new(t1_4));
        t1_2.next = Some(Box::new(t1_3));
        t1_1.next = Some(Box::new(t1_2));

        assert!(!Solution::is_sub_path(
            Some(Box::new(t1_1)),
            Some(Rc::new(RefCell::new(t1)))
        ));
    }
}
