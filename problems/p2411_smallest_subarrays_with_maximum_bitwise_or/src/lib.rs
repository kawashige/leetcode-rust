pub struct Solution {}

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let mut min_index = vec![std::usize::MAX; 32];
        let mut result = vec![0; nums.len()];

        for i in (0..nums.len()).rev() {
            let mut max_index = i;
            for j in 0..min_index.len() {
                if nums[i] & 1 << j != 0 {
                    min_index[j] = i;
                }
                if min_index[j] != std::usize::MAX {
                    max_index = max_index.max(min_index[j]);
                }
            }
            result[i] = (max_index - i + 1) as i32;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2411() {
        assert_eq!(
            Solution::smallest_subarrays(vec![1, 0, 2, 1, 3]),
            vec![3, 3, 2, 2, 1]
        );
        assert_eq!(Solution::smallest_subarrays(vec![1, 2]), vec![2, 1]);
    }
}
