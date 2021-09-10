use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut dp = vec![HashMap::new(); nums.len()];
        let mut r = 0;

        for i in 1..nums.len() {
            for j in (0..i).rev() {
                let d = nums[i] as i64 - nums[j] as i64;
                let c = *dp[j].get(&d).unwrap_or(&0);
                r += c;
                *dp[i].entry(d).or_insert(0) += c + 1;
            }
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day10() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
            0
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
    }
}
