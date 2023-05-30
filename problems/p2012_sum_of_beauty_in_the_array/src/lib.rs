pub struct Solution {}

impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let mut left_max = vec![0; nums.len()];
        let mut right_min = vec![0; nums.len()];
        left_max[0] = nums[0];
        right_min[nums.len() - 1] = nums[nums.len() - 1];

        for i in 1..nums.len() {
            left_max[i] = left_max[i - 1].max(nums[i]);
        }
        for i in (0..nums.len() - 1).rev() {
            right_min[i] = right_min[i + 1].min(nums[i]);
        }

        (1..nums.len() - 1)
            .map(|i| {
                if left_max[i - 1] < nums[i] && nums[i] < right_min[i + 1] {
                    2
                } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2012() {
        assert_eq!(Solution::sum_of_beauties(vec![1, 2, 3]), 2);
        assert_eq!(Solution::sum_of_beauties(vec![2, 4, 6, 4]), 1);
        assert_eq!(Solution::sum_of_beauties(vec![3, 2, 1]), 0);
    }
}
