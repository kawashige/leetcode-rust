pub struct Solution {}

impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_007;
        if let Some(i) = (0..nums.len()).find(|i| nums[*i] == 1) {
            let mut result = 1;
            let mut prev = i;

            for i in i + 1..nums.len() {
                if nums[i] == 1 {
                    result *= i - prev;
                    result %= M;
                    prev = i;
                }
            }

            result as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2750() {
        assert_eq!(
            Solution::number_of_good_subarray_splits(vec![0, 1, 0, 0, 1]),
            3
        );
        assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0]), 1);
    }
}
