pub struct Solution {}

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut nums = nums.into_iter().zip(0..).collect::<Vec<_>>();
        nums.sort_unstable();

        let mut values = Vec::new();
        let mut indices = Vec::new();
        for i in 0..nums.len() {
            if 0 < i && limit < (nums[i].0 - nums[i - 1].0).abs() {
                indices.sort_unstable();
                for j in 0..indices.len() {
                    result[indices[j]] = values[j];
                }
                values.clear();
                indices.clear();
            }
            values.push(nums[i].0);
            indices.push(nums[i].1);
        }
        indices.sort_unstable();
        for j in 0..indices.len() {
            result[indices[j]] = values[j];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2948() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
            vec![1, 6, 7, 18, 1, 2]
        );
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
            vec![1, 7, 28, 19, 10]
        );
    }
}
