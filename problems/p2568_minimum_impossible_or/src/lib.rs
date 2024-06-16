pub struct Solution {}

impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 1;

        for i in 0..nums.len() {
            if result == nums[i] {
                result *= 2;
            } else if result < nums[i] {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2568() {
        assert_eq!(Solution::min_impossible_or(vec![2, 1]), 4);
        assert_eq!(Solution::min_impossible_or(vec![5, 3, 2]), 1);
    }
}
