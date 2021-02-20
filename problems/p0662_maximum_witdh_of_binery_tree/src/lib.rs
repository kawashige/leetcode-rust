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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(
            root: &Option<Rc<RefCell<TreeNode>>>,
            depth: usize,
            width: u64,
            widths: &mut Vec<(u64, u64)>,
        ) {
            if let Some(node) = root {
                let n = node.borrow();
                if widths.len() <= depth {
                    widths.push((width, width));
                } else {
                    widths[depth] = (
                        std::cmp::min(widths[depth].0, width),
                        std::cmp::max(widths[depth].1, width),
                    )
                }
                recurse(&n.left, depth + 1, width * 2, widths);
                recurse(&n.right, depth + 1, width * 2 + 1, widths);
            }
        }

        let mut widths = Vec::new();
        recurse(&root, 0, 0, &mut widths);
        widths
            .into_iter()
            .map(|(l, r)| (r - l + 1) as i32)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0662() {
        let mut t1_1 = TreeNode::new(1);
        let mut t1_2 = TreeNode::new(3);
        let mut t1_3 = TreeNode::new(2);
        let t1_4 = TreeNode::new(5);
        let t1_5 = TreeNode::new(3);
        let t1_6 = TreeNode::new(9);
        t1_3.right = Some(Rc::new(RefCell::new(t1_6)));
        t1_2.left = Some(Rc::new(RefCell::new(t1_4)));
        t1_2.right = Some(Rc::new(RefCell::new(t1_5)));
        t1_1.left = Some(Rc::new(RefCell::new(t1_2)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_3)));

        assert_eq!(
            Solution::width_of_binary_tree(Some(Rc::new(RefCell::new(t1_1)))),
            4
        );

        let mut t2_1 = TreeNode::new(1);
        let mut t2_2 = TreeNode::new(3);
        let t2_3 = TreeNode::new(2);
        let t2_4 = TreeNode::new(5);
        t2_2.left = Some(Rc::new(RefCell::new(t2_3)));
        t2_2.right = Some(Rc::new(RefCell::new(t2_4)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));

        assert_eq!(
            Solution::width_of_binary_tree(Some(Rc::new(RefCell::new(t2_1)))),
            2
        );

        let mut t3_1 = TreeNode::new(1);
        let mut t3_2 = TreeNode::new(3);
        let mut t3_3 = TreeNode::new(2);
        let mut t3_4 = TreeNode::new(5);
        let mut t3_5 = TreeNode::new(3);
        let t3_6 = TreeNode::new(9);
        let t3_7 = TreeNode::new(9);
        t3_5.right = Some(Rc::new(RefCell::new(t3_7)));
        t3_4.left = Some(Rc::new(RefCell::new(t3_6)));
        t3_3.right = Some(Rc::new(RefCell::new(t3_5)));
        t3_2.left = Some(Rc::new(RefCell::new(t3_4)));
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));
        t3_1.right = Some(Rc::new(RefCell::new(t3_3)));

        assert_eq!(
            Solution::width_of_binary_tree(Some(Rc::new(RefCell::new(t3_1)))),
            8
        );
    }
}
