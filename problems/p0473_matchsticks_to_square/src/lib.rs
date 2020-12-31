pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn makesquare(mut nums: Vec<i32>) -> bool {
        fn recurse(nums: &Vec<i32>, selected: &mut HashSet<usize>, target: i32) -> bool {
            for i in 0..nums.len() {
                if selected.contains(&i) {
                    continue;
                }
                if target < nums[i] {
                    return false;
                }

                selected.insert(i);
                if nums[i] == target {
                    return true;
                } else {
                    if recurse(nums, selected, target - nums[i]) {
                        return true;
                    } else {
                        selected.remove(&i);
                    }
                }
            }
            false
        }

        if nums.len() < 4 {
            return false;
        }

        let sum_length: i32 = nums.iter().sum();
        if sum_length % 4 != 0 {
            return false;
        }

        let edge_length = sum_length / 4;
        nums.sort();
        nums.reverse();
        for _ in 0..4 {
            let mut selected = HashSet::new();
            if !recurse(&nums, &mut selected, edge_length) {
                return false;
            }
            nums = nums
                .into_iter()
                .enumerate()
                .filter(|(i, _)| !selected.contains(i))
                .map(|(_, v)| v)
                .collect();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0473() {
        assert!(Solution::makesquare(vec![
            5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3
        ]));
        assert!(Solution::makesquare(vec![1, 1, 2, 2, 2]));
        assert!(!Solution::makesquare(vec![3, 3, 3, 3, 4]));
    }
}
