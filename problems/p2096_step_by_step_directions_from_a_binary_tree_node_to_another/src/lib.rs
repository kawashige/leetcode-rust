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
    pub fn find_path(root: &Option<Rc<RefCell<TreeNode>>>, value: i32, path: &mut String) -> bool {
        if let Some(node) = root {
            let n = node.borrow();
            if n.val == value {
                return true;
            } else {
                path.push('L');
                if Self::find_path(&n.left, value, path) {
                    return true;
                }
                path.pop();
                path.push('R');
                if Self::find_path(&n.right, value, path) {
                    return true;
                }
                path.pop();
            }
        }
        false
    }

    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let mut start_path = String::new();
        let mut dest_path = String::new();
        Self::find_path(&root, start_value, &mut start_path);
        Self::find_path(&root, dest_value, &mut dest_path);

        let mut i = 0;
        while i < start_path.len()
            && i < dest_path.len()
            && start_path.as_bytes()[i] == dest_path.as_bytes()[i]
        {
            i += 1;
        }

        let mut result = String::new();
        for _ in i..start_path.len() {
            result.push('U');
        }
        result += &dest_path[i..];
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2096() {
        let mut t1 = TreeNode::new(5);
        let mut t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(2);
        let t4 = TreeNode::new(3);
        let t5 = TreeNode::new(6);
        let t6 = TreeNode::new(4);
        t3.left = Some(Rc::new(RefCell::new(t5)));
        t3.right = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::get_directions(Some(Rc::new(RefCell::new(t1))), 3, 6),
            "UURL".to_string()
        );

        let mut t1 = TreeNode::new(2);
        let t2 = TreeNode::new(1);
        t1.left = Some(Rc::new(RefCell::new(t2)));

        assert_eq!(
            Solution::get_directions(Some(Rc::new(RefCell::new(t1))), 2, 1),
            "L".to_string()
        );
    }
}
