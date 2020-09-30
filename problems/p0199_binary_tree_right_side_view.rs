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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, l: usize, values: &mut Vec<Vec<i32>>) {
            match root {
                Some(r) => {
                    let r = r.borrow();
                    if values.len() < l {
                        values.push(vec![r.val]);
                    } else {
                        values[l - 1].push(r.val);
                    }
                    recurse(&r.left, l + 1, values);
                    recurse(&r.right, l + 1, values);
                }
                None => {}
            }
        }

        let mut values = Vec::new();
        recurse(&root, 1, &mut values);
        values.into_iter().map(|mut v| v.pop().unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0199() {
        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(5);
        let t5 = TreeNode::new(4);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            vec![1, 3, 4],
            Solution::right_side_view(Some(Rc::new(RefCell::new(t1))))
        );
    }
}
