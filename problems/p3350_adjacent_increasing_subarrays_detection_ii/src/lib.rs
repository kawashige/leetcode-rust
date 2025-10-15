pub struct Solution {}

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut lens = vec![1; nums.len()];
        let mut len = 0;
        let mut result = 1;
        for i in 0..nums.len() {
            if 0 < i && nums[i] <= nums[i - 1] {
                len = 1;
            } else {
                len += 1;
            }
            lens[i] = len;
            result = result.max(len / 2);
            if len <= i && len <= lens[i - len] {
                result = result.max(len);
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3350() {
        assert_eq!(
            Solution::max_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1]),
            3
        );
        assert_eq!(
            Solution::max_increasing_subarrays(vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7]),
            2
        );
    }
}
