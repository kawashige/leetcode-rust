pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut result = 0;
        for i in 0..nums.len() - 2 {
            if nums[i] == 0 {
                nums[i + 1] = (nums[i + 1] + 1) % 2;
                nums[i + 2] = (nums[i + 2] + 1) % 2;
                result += 1;
            }
        }
        if nums[nums.len() - 1] == 1 && nums[nums.len() - 2] == 1 {
            result
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3191() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1]), -1);
    }
}
