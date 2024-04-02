pub struct Solution {}

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = Vec::new();
        let mut zero_count = 0;

        for i in 0..nums.len() - 1 {
            if nums[i] == 0 {
                zero_count += 1;
            } else if nums[i] == nums[i + 1] {
                result.push(nums[i] * 2);
                nums[i + 1] = 0;
            } else {
                result.push(nums[i]);
            }
        }
        result.push(nums[nums.len() - 1]);

        for _ in 0..zero_count {
            result.push(0);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2460() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
        assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
    }
}
