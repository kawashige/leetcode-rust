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
        val: usize,
        children: &Vec<Vec<usize>>,
        descriptions: &Vec<Vec<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = TreeNode::new(val as i32);
        for child in &children[val] {
            if descriptions[*child][2] == 1 {
                node.left = Self::recurse(descriptions[*child][1] as usize, children, descriptions);
            } else {
                node.right =
                    Self::recurse(descriptions[*child][1] as usize, children, descriptions);
            }
        }
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parent = vec![0; 100_001];
        let mut children = vec![vec![]; 100_001];

        for i in 0..descriptions.len() {
            parent[descriptions[i][1] as usize] = descriptions[i][0] as usize;
            children[descriptions[i][0] as usize].push(i);
        }

        let mut root = descriptions[0][0] as usize;
        while parent[root] != 0 {
            root = parent[root];
        }

        Self::recurse(root, &children, &descriptions)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2196() {
        let mut t1 = TreeNode::new(50);
        let mut t2 = TreeNode::new(20);
        let mut t3 = TreeNode::new(80);
        let t4 = TreeNode::new(15);
        let t5 = TreeNode::new(17);
        let t6 = TreeNode::new(19);
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        assert_eq!(
            Solution::create_binary_tree(vec![
                vec![20, 15, 1],
                vec![20, 17, 0],
                vec![50, 20, 1],
                vec![50, 80, 0],
                vec![80, 19, 1]
            ]),
            Some(Rc::new(RefCell::new(t1)))
        );

        let mut t1 = TreeNode::new(1);
        let mut t2 = TreeNode::new(2);
        let mut t3 = TreeNode::new(3);
        let t4 = TreeNode::new(4);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t3)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        assert_eq!(
            Solution::create_binary_tree(vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]]),
            Some(Rc::new(RefCell::new(t1)))
        );
    }
}
