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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn found_mistakes(
            root: &Option<Rc<RefCell<TreeNode>>>,
            nums: &mut Vec<Vec<i32>>,
            min: i32,
            max: i32,
        ) {
            if let Some(n) = root {
                let n = n.borrow();
                if n.val < min {
                    nums.push(vec![min, n.val]);
                } else if max < n.val {
                    nums.push(vec![n.val, max]);
                }
                found_mistakes(&n.left, nums, min, n.val);
                found_mistakes(&n.right, nums, n.val, max);
            }
        }

        fn recover(root: &mut Option<Rc<RefCell<TreeNode>>>, nums: &Vec<i32>) {
            if let Some(n) = root {
                let mut n = n.borrow_mut();

                if n.val == nums[0] {
                    n.val = nums[1];
                } else if n.val == nums[1] {
                    n.val = nums[0];
                }

                recover(&mut n.left, &nums);
                recover(&mut n.right, &nums);
            }
        }

        fn detect_target(nums: &Vec<Vec<i32>>) -> Vec<i32> {
            if nums.len() > 1 {
                let mut targets = Vec::new();
                for i in 1..nums.len() {
                    for j in [0usize, 1].iter() {
                        if nums[i][*j] < nums[0][1] {
                            targets.push(vec![nums[0][0], nums[i][*j]]);
                        } else if nums[0][0] < nums[i][*j] {
                            targets.push(vec![nums[0][1], nums[i][*j]]);
                        }
                    }
                }

                for t in targets {
                    if nums.iter().all(|n| {
                        let a = if n[0] == t[0] {
                            t[1]
                        } else if n[0] == t[1] {
                            t[0]
                        } else {
                            n[0]
                        };
                        let b = if n[1] == t[0] {
                            t[1]
                        } else if n[1] == t[1] {
                            t[0]
                        } else {
                            n[1]
                        };
                        a < b
                    }) {
                        return t;
                    }
                }
            }
            vec![nums[0][0], nums[0][1]]
        }

        let mut nums = Vec::new();
        found_mistakes(&root, &mut nums, std::i32::MIN, std::i32::MAX);
        let swaps = detect_target(&nums);
        recover(root, &swaps);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day31() {
        let mut t1 = TreeNode::new(3);
        let mut t2 = TreeNode::new(4);
        let t3 = TreeNode::new(1);
        let t4 = TreeNode::new(2);
        t2.right = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(3);
        let mut r2 = TreeNode::new(1);
        let r3 = TreeNode::new(4);
        let r4 = TreeNode::new(2);
        r2.right = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        let mut input = Some(Rc::new(RefCell::new(t1)));
        Solution::recover_tree(&mut input);
        assert_eq!(Some(Rc::new(RefCell::new(r1))), input);

        let mut t1 = TreeNode::new(3);
        let t2 = TreeNode::new(1);
        let mut t3 = TreeNode::new(4);
        let t4 = TreeNode::new(2);
        t3.left = Some(Rc::new(RefCell::new(t4)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(2);
        let r2 = TreeNode::new(1);
        let mut r3 = TreeNode::new(4);
        let r4 = TreeNode::new(3);
        r3.left = Some(Rc::new(RefCell::new(r4)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        let mut input = Some(Rc::new(RefCell::new(t1)));
        Solution::recover_tree(&mut input);
        assert_eq!(Some(Rc::new(RefCell::new(r1))), input);

        let mut t1 = TreeNode::new(15);
        let mut t2 = TreeNode::new(3);
        let mut t3 = TreeNode::new(8);
        let t4 = TreeNode::new(2);
        let t5 = TreeNode::new(4);
        let t6 = TreeNode::new(7);
        let mut t7 = TreeNode::new(10);
        let t8 = TreeNode::new(5);
        t7.right = Some(Rc::new(RefCell::new(t8)));
        t3.left = Some(Rc::new(RefCell::new(t6)));
        t3.right = Some(Rc::new(RefCell::new(t7)));
        t2.left = Some(Rc::new(RefCell::new(t4)));
        t2.right = Some(Rc::new(RefCell::new(t5)));
        t1.left = Some(Rc::new(RefCell::new(t2)));
        t1.right = Some(Rc::new(RefCell::new(t3)));

        let mut r1 = TreeNode::new(5);
        let mut r2 = TreeNode::new(3);
        let mut r3 = TreeNode::new(8);
        let r4 = TreeNode::new(2);
        let r5 = TreeNode::new(4);
        let r6 = TreeNode::new(7);
        let mut r7 = TreeNode::new(10);
        let r8 = TreeNode::new(15);
        r7.right = Some(Rc::new(RefCell::new(r8)));
        r3.left = Some(Rc::new(RefCell::new(r6)));
        r3.right = Some(Rc::new(RefCell::new(r7)));
        r2.left = Some(Rc::new(RefCell::new(r4)));
        r2.right = Some(Rc::new(RefCell::new(r5)));
        r1.left = Some(Rc::new(RefCell::new(r2)));
        r1.right = Some(Rc::new(RefCell::new(r3)));

        let mut input = Some(Rc::new(RefCell::new(t1)));
        Solution::recover_tree(&mut input);
        assert_eq!(Some(Rc::new(RefCell::new(r1))), input);
    }
}
