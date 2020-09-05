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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match nums.len() {
                0 => None,
                _ => {
                    let i = nums.len() / 2;
                    let mut t = TreeNode::new(nums[i]);
                    t.left = build(&nums[..i]);
                    t.right = build(&nums[(i + 1)..]);
                    Some(Rc::new(RefCell::new(t)))
                }
            }
        }
        let mut nums = Vec::new();
        let mut node = head;
        while let Some(n) = node {
            nums.push(n.val);
            node = n.next;
        }
        build(&nums)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0109() {
        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(-3);
        let mut t3 = TreeNode::new(9);
        let t4 = TreeNode::new(-10);
        let t5 = TreeNode::new(5);
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut l1_1 = ListNode::new(-10);
        let mut l1_2 = ListNode::new(-3);
        let mut l1_3 = ListNode::new(0);
        let mut l1_4 = ListNode::new(5);
        let l1_5 = ListNode::new(9);
        l1_4.next = Some(Box::new(l1_5));
        l1_3.next = Some(Box::new(l1_4));
        l1_2.next = Some(Box::new(l1_3));
        l1_1.next = Some(Box::new(l1_2));

        let result = Solution::sorted_list_to_bst(Some(Box::new(l1_1))).unwrap();
        assert_eq!(Rc::new(RefCell::new(t1)), result);
    }
}
