use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut max = 0;
        let mut current_max = vec![0; nums.len()];
        let mut max_index = HashMap::new();
        max_index.insert(0, 0);
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];
            if let Some(j) = max_index.get(&(sum - target)) {
                let mut tmp_max = 1;
                if &0 < j {
                    tmp_max += current_max[j - 1];
                }
                max = max.max(tmp_max);
            }
            current_max[i] = max;
            max_index.insert(sum, i + 1);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1546() {
        assert_eq!(Solution::max_non_overlapping(vec![1, 1, 1, 1, 1], 2), 2);
        assert_eq!(
            Solution::max_non_overlapping(vec![-1, 3, 5, 1, 4, 2, -9], 6),
            2
        );
    }
}
