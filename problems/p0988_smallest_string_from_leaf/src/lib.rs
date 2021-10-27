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
    pub fn recurse(
        root: &Option<Rc<RefCell<TreeNode>>>,
        chars: &mut Vec<char>,
        smallest: &mut Option<String>,
    ) {
        if let Some(node) = root {
            let n = node.borrow();
            chars.push((n.val as u8 + 0x61) as char);

            match (&n.left, &n.right) {
                (None, None) => {
                    let s = chars.iter().rev().collect();
                    *smallest = Some(smallest.as_ref().unwrap_or(&s).clone().min(s))
                }
                (None, _) => Self::recurse(&n.right, chars, smallest),
                (_, None) => Self::recurse(&n.left, chars, smallest),
                _ => {
                    Self::recurse(&n.left, chars, smallest);
                    Self::recurse(&n.right, chars, smallest);
                }
            }

            chars.pop();
        }
    }

    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut smallest = None;
        Self::recurse(&root, &mut Vec::new(), &mut smallest);
        smallest.unwrap().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0988() {
        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(2);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(4);
        let t6 = TreeNode::new(3);
        let t7 = TreeNode::new(4);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(t1)))),
            "dba".to_string()
        );

        let mut t1 = TreeNode::new(25);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(1);
        let t5 = TreeNode::new(3);
        let t6 = TreeNode::new(0);
        let t7 = TreeNode::new(2);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(t1)))),
            "adz".to_string()
        );

        let mut t1 = TreeNode::new(2);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(1);
        let mut t4 = TreeNode::new(1);
        let t5 = TreeNode::new(0);
        let t6 = TreeNode::new(0);
        t4.left = Some(Rc::new(RefCell::new(t6)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(t1)))),
            "abc".to_string()
        );

        let mut t1 = TreeNode::new(4);
        let mut t2 = TreeNode::new(0);
        let t3 = TreeNode::new(1);
        let t4 = TreeNode::new(1);
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t3)));
        t1.right = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(t1)))),
            "bae".to_string()
        );

        let mut t1 = TreeNode::new(25);
        let mut t2 = TreeNode::new(1);
        let t3 = TreeNode::new(0);
        let mut t4 = TreeNode::new(0);
        let mut t5 = TreeNode::new(1);
        let t6 = TreeNode::new(0);
        t5.left = Some(Rc::new(RefCell::new(t6)));
        t4.left = Some(Rc::new(RefCell::new(t5)));
        t2.left = Some(Rc::new(RefCell::new(t3)));
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::smallest_from_leaf(Some(Rc::new(RefCell::new(t1)))),
            "ababz".to_string()
        );
    }
}
