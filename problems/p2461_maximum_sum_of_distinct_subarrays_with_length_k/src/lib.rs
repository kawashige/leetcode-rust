pub struct Solution {}

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = vec![0; 100_001];
        let mut duplicated = 0;
        let mut sum = 0;
        let mut result = 0;
        let k = k as usize;

        for i in 0..nums.len() {
            sum += nums[i] as i64;
            count[nums[i] as usize] += 1;
            if count[nums[i] as usize] == 2 {
                duplicated += 1;
            }
            if k <= i {
                sum -= nums[i - k] as i64;
                count[nums[i - k] as usize] -= 1;
                if count[nums[i - k] as usize] == 1 {
                    duplicated -= 1;
                }
            }
            if k - 1 <= i && duplicated == 0 {
                result = result.max(sum);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2461() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3),
            15
        );
        assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}
