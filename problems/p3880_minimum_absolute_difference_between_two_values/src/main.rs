pub struct Solution {}

impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
        let mut result = std::i32::MAX;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if nums[i] == 1 && nums[j] == 2 {
                    result = result.min((i as i32 - j as i32).abs());
                }
            }
        }

        if result == std::i32::MAX { -1 } else { result }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3880() {
        assert_eq!(Solution::min_absolute_difference(vec![1, 0, 0, 2, 0, 1]), 2);
        assert_eq!(Solution::min_absolute_difference(vec![1, 0, 1, 0]), -1);
    }
}

fn main() {}
