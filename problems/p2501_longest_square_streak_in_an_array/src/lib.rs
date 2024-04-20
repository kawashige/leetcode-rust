use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut map = HashMap::new();
        let mut result = -1;
        for num in nums {
            if let Some(len) = map.get(&num) {
                result = result.max(len + 1);
                map.insert(num * num, len + 1);
            } else {
                map.insert(num * num, 1);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2501() {
        assert_eq!(Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
        assert_eq!(Solution::longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
    }
}
