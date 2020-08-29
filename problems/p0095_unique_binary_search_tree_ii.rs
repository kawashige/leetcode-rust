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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn generate(nums: &[i32]) -> Vec<Rc<RefCell<TreeNode>>> {
            let mut results = Vec::new();
            for i in 0..nums.len() {
                let lefts = generate(&nums[..i]);
                let rights = generate(&nums[(i + 1)..]);

                if lefts.len() == 0 && rights.len() == 0 {
                    results.push(Rc::new(RefCell::new(TreeNode::new(nums[i]))));
                } else if lefts.len() == 0 {
                    for r in rights {
                        let mut t = TreeNode::new(nums[i]);
                        t.right = Some(r.clone());
                        results.push(Rc::new(RefCell::new(t)));
                    }
                } else if rights.len() == 0 {
                    for l in lefts {
                        let mut t = TreeNode::new(nums[i]);
                        t.left = Some(l.clone());
                        results.push(Rc::new(RefCell::new(t)));
                    }
                } else {
                    for l in lefts {
                        for r in rights.clone() {
                            let mut t = TreeNode::new(nums[i]);
                            t.left = Some(l.clone());
                            t.right = Some(r.clone());
                            results.push(Rc::new(RefCell::new(t)));
                        }
                    }
                }
            }
            results
        }

        generate(&(1..=n).collect::<Vec<i32>>())
            .into_iter()
            .map(|x| Some(x))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0095() {
        let mut t1_1 = TreeNode::new(1);
        let mut t1_2 = TreeNode::new(3);
        let t1_3 = TreeNode::new(2);
        t1_2.left = Some(Rc::new(RefCell::new(t1_3)));
        t1_1.right = Some(Rc::new(RefCell::new(t1_2)));

        let mut t2_1 = TreeNode::new(3);
        let mut t2_2 = TreeNode::new(2);
        let t2_3 = TreeNode::new(1);
        t2_2.left = Some(Rc::new(RefCell::new(t2_3)));
        t2_1.left = Some(Rc::new(RefCell::new(t2_2)));

        let mut t3_1 = TreeNode::new(3);
        let mut t3_2 = TreeNode::new(1);
        let t3_3 = TreeNode::new(2);
        t3_2.right = Some(Rc::new(RefCell::new(t3_3)));
        t3_1.left = Some(Rc::new(RefCell::new(t3_2)));

        let mut t4_1 = TreeNode::new(2);
        let t4_2 = TreeNode::new(1);
        let t4_3 = TreeNode::new(3);
        t4_1.left = Some(Rc::new(RefCell::new(t4_2)));
        t4_1.right = Some(Rc::new(RefCell::new(t4_3)));

        let mut t5_1 = TreeNode::new(1);
        let mut t5_2 = TreeNode::new(2);
        let t5_3 = TreeNode::new(3);
        t5_2.right = Some(Rc::new(RefCell::new(t5_3)));
        t5_1.right = Some(Rc::new(RefCell::new(t5_2)));

        assert_eq!(
            vec![
                Some(Rc::new(RefCell::new(t5_1))),
                Some(Rc::new(RefCell::new(t1_1))),
                Some(Rc::new(RefCell::new(t4_1))),
                Some(Rc::new(RefCell::new(t3_1))),
                Some(Rc::new(RefCell::new(t2_1))),
            ],
            Solution::generate_trees(3)
        );
    }
}
