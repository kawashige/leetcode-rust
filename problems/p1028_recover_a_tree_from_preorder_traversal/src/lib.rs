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
    pub fn recurse(v: &[String], d: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let sp = std::iter::repeat('-').take(d + 1).collect::<String>();
        if let Some(l) = (0..v.len()).find(|i| v[*i] == sp) {
            let mut node = TreeNode::new(v[0].parse::<i32>().unwrap());
            if let Some(r) = (l + 1..v.len()).find(|i| v[*i] == sp) {
                node.left = Self::recurse(&v[l + 1..r], d + 1);
                node.right = Self::recurse(&v[r + 1..], d + 1);
            } else {
                node.left = Self::recurse(&v[l + 1..], d + 1);
            }
            Some(Rc::new(RefCell::new(node)))
        } else {
            let node = TreeNode::new(v[0].parse::<i32>().unwrap());
            Some(Rc::new(RefCell::new(node)))
        }
    }

    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v: Vec<String> = Vec::new();
        for c in traversal.chars() {
            if (c == '-' && !v.is_empty() && v[v.len() - 1].contains(c))
                || (c.is_digit(10) && !v.is_empty() && !v[v.len() - 1].contains('-'))
            {
                (*v.last_mut().unwrap()).push(c);
            } else {
                v.push(c.to_string());
            }
        }
        Self::recurse(&v, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1028() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(5);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(4);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string()),
            Some(Rc::new(RefCell::new(t1)))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(5);
        let mut t4 = TreeNode::new(3);
        let mut t5 = TreeNode::new(4);
        let t6 = TreeNode::new(6);
        let t7 = TreeNode::new(7);
        t5.left = Some(Rc::new(RefCell::new(t7)));
        t4.left = Some(Rc::new(RefCell::new(t6)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::recover_from_preorder("1-2--3---4-5--6---7".to_string()),
            Some(Rc::new(RefCell::new(t1)))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(401);
        let mut t3 = TreeNode::new(349);
        let t4 = TreeNode::new(88);
        let t5 = TreeNode::new(90);
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::recover_from_preorder("1-401--349---90--88".to_string()),
            Some(Rc::new(RefCell::new(t1)))
        );
    }
}
