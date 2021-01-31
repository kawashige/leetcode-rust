pub struct Solution {}

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut right_min = vec![nums[nums.len() - 1]; nums.len()];
        for i in (0..(nums.len() - 1)).rev() {
            right_min[i] = std::cmp::min(nums[i + 1], right_min[i + 1]);
        }
        let mut left_max = vec![nums[0]; nums.len()];
        for i in 2..nums.len() {
            left_max[i] = std::cmp::max(nums[i - 1], left_max[i - 1]);
        }

        if let Some(min) =
            (0..nums.len()).find(|i| right_min[*i] < nums[*i] || nums[*i] < left_max[*i])
        {
            if let Some(max) = ((min + 1)..nums.len())
                .rev()
                .find(|i| right_min[*i] < nums[*i] || nums[*i] < left_max[*i])
            {
                return (max - min + 1) as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0581() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
            5
        );
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
    }
}
