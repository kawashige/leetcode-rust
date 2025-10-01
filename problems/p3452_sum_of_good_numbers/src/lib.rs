pub struct Solution {}

impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        (0..nums.len())
            .filter_map(|i| {
                if (i < k || nums[i - k] < nums[i])
                    && (i + k >= nums.len() || nums[i + k] < nums[i])
                {
                    Some(nums[i])
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3452() {
        assert_eq!(Solution::sum_of_good_numbers(vec![1, 3, 2, 1, 5, 4], 2), 12);
        assert_eq!(Solution::sum_of_good_numbers(vec![2, 1], 1), 2);
    }
}
