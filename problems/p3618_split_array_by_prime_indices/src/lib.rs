pub struct Solution {}

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        if nums.len() < 2 {
            return (nums.into_iter().sum::<i32>() as i64).abs();
        }
        let mut is_prime = vec![true; nums.len()];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..nums.len() {
            if nums.len() <= i * i {
                break;
            }
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..nums.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let mut sum = [0; 2];

        for i in 0..nums.len() {
            sum[is_prime[i] as usize] += nums[i] as i64;
        }

        (sum[0] - sum[1]).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3618() {
        assert_eq!(Solution::split_array(vec![2, 3, 4]), 1);
        assert_eq!(Solution::split_array(vec![-1, 5, 7, 0]), 3);
    }
}
