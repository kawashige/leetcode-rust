pub struct Solution {}

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut result = std::i32::MAX;

        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if (l..=r).contains(&((j - i + 1) as i32)) && 0 < sum {
                    result = result.min(sum);
                }
            }
        }

        if result == std::i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3364() {
        assert_eq!(Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
        assert_eq!(Solution::minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3), -1);
        assert_eq!(Solution::minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4), 3);
    }
}
