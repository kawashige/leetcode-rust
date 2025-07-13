pub struct Solution {}

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut counts = vec![vec![0; 10]; nums[0].to_string().len()];
        for i in 0..nums.len() {
            for (i, d) in nums[i].to_string().as_bytes().iter().rev().enumerate() {
                counts[i][(d - b'0') as usize] += 1;
            }
        }

        let mut result = 0;
        for i in 0..counts.len() {
            for j in 0..counts[i].len() {
                result += counts[i][j] as i64 * (nums.len() as i64 - counts[i][j] as i64)
            }
        }

        result / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3153() {
        assert_eq!(Solution::sum_digit_differences(vec![13, 23, 12]), 4);
        assert_eq!(Solution::sum_digit_differences(vec![10, 10, 10, 10]), 0);
    }
}
