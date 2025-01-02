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
use std::collections::VecDeque;
use std::rc::Rc;
struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut nums = Vec::new();
        Self::recurse2(&root, &mut nums, 0);
        let mut result = Vec::new();
        for i in 0..nums.len() {
            if nums[i].iter().all(|n| !n.is_some()) {
                break;
            }
            result.append(
                &mut nums[i]
                    .iter()
                    .map(|n| {
                        if n.is_some() {
                            n.unwrap().to_string()
                        } else {
                            "null".to_string()
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }
        format!("[{}]", result.join(","))
    }

    fn recurse2(node: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<Vec<Option<i32>>>, l: usize) {
        if nums.len() < l + 1 {
            nums.push(Vec::new());
        }
        if let Some(node) = node {
            let n = node.borrow();
            nums[l].push(Some(n.val));
            Self::recurse2(&n.left, nums, l + 1);
            Self::recurse2(&n.right, nums, l + 1);
        } else {
            nums[l].push(None);
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if &data == "[]" {
            return None;
        }
        let nums = data[1..data.len() - 1].split(",").collect::<Vec<_>>();
        let mut deque = VecDeque::new();
        deque.push_back(nums[0]);
        let mut nums2 = vec![deque];
        let mut i = 1;
        while i < nums.len() {
            let mut v = VecDeque::new();
            let j = nums2.last().unwrap().len();
            for k in 0..j {
                if nums2[nums2.len() - 1][k] != "null" {
                    v.push_back(nums[i]);
                    v.push_back(nums[i + 1]);
                    i += 2;
                    if nums.len() <= i {
                        break;
                    }
                }
            }
            nums2.push(v);
        }
        Self::recurse(0, &mut nums2)
    }

    fn recurse(i: usize, nums: &mut Vec<VecDeque<&str>>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() <= i || nums[i].is_empty() {
            return None;
        }
        if nums[i][0] == "null" {
            nums[i].pop_front();
            return None;
        }
        let mut node = TreeNode::new(nums[i].pop_front().unwrap().parse::<i32>().unwrap());
        node.left = Self::recurse(i + 1, nums);
        node.right = Self::recurse(i + 1, nums);
        Some(Rc::new(RefCell::new(node)))
    }
}
