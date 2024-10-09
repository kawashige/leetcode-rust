pub struct Solution {}

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut result = -1;

        for i in 0..nums.len() {
            let mut d = 1;
            let mut j = i + 1;
            let mut len = 1;
            while j < nums.len() && nums[j] - nums[j - 1] == d {
                len += 1;
                d *= -1;
                j += 1;
            }
            if 1 < len {
                result = result.max(len);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2765() {
        assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
        assert_eq!(Solution::alternating_subarray(vec![4, 5, 6]), 2);
    }
}
