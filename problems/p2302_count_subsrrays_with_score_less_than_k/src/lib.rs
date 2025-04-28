pub struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut result = 0;
        let mut l = 0;
        let mut sum = 0;

        for r in 0..nums.len() {
            sum += nums[r] as i64;
            while l < r && k <= sum * (r - l + 1) as i64 {
                sum -= nums[l] as i64;
                l += 1;
            }
            if sum * ((r - l + 1) as i64) < k {
                result += (r - l + 1) as i64;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2302() {
        assert_eq!(Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 5), 5);
    }
}
