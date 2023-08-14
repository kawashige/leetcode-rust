pub struct Solution {}

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut min = vec![vec![std::i32::MAX; nums.len()]; nums.len()];
        let mut max = vec![vec![std::i32::MAX; nums.len()]; nums.len()];

        for i in 0..nums.len() {
            min[i][i] = nums[i];
            max[i][i] = nums[i];
        }

        let mut result = 0;

        for l in 1..nums.len() {
            for i in 0..nums.len() - l {
                min[i][i + l] = min[i][i + l - 1].min(nums[i + l]);
                max[i][i + l] = max[i][i + l - 1].max(nums[i + l]);
                result += (max[i][i + l] - min[i][i + l]) as i64;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2104() {
        assert_eq!(Solution::sub_array_ranges(vec![1, 2, 3]), 4);
        assert_eq!(Solution::sub_array_ranges(vec![1, 3, 3]), 4);
        assert_eq!(Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]), 59);
    }
}
