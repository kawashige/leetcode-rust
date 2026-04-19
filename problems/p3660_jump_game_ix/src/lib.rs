pub struct Solution {}

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_max = vec![0; nums.len()];
        prefix_max[0] = nums[0];
        for i in 1..nums.len() {
            prefix_max[i] = prefix_max[i - 1].max(nums[i]);
        }
        let mut suffix_min = vec![0; nums.len()];
        suffix_min[nums.len() - 1] = nums[nums.len() - 1];
        for i in (0..nums.len() - 1).rev() {
            suffix_min[i] = suffix_min[i + 1].min(nums[i]);
        }

        let mut result = vec![0; nums.len()];
        result[prefix_max.len() - 1] = prefix_max[prefix_max.len() - 1];

        for i in (0..result.len() - 1).rev() {
            result[i] = prefix_max[i];
            if prefix_max[i] > suffix_min[i + 1] {
                result[i] = result[i + 1];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3660() {
        assert_eq!(Solution::max_value(vec![2, 1, 3]), vec![2, 2, 3]);
        assert_eq!(Solution::max_value(vec![2, 3, 1]), vec![3, 3, 3]);
    }
}
