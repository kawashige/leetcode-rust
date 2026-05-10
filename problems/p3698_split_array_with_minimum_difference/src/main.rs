pub struct Solution {}

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        let mut sum = vec![0; nums.len()];
        sum[0] = nums[0] as i64;
        let mut increasing = vec![true; nums.len()];
        for i in 1..nums.len() {
            sum[i] = sum[i - 1] + nums[i] as i64;
            if !increasing[i - 1] || nums[i] <= nums[i - 1] {
                increasing[i] = false;
            }
        }
        let mut decreasing = vec![true; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            if !decreasing[i + 1] || nums[i] <= nums[i + 1] {
                decreasing[i] = false;
            }
        }

        println!("increasing: {:?}", increasing);
        println!("decreasing: {:?}", decreasing);
        println!("sum: {:?}", sum);

        let mut result = std::i64::MAX;
        for i in 0..nums.len() - 1 {
            if increasing[i] && decreasing[i + 1] {
                result = result.min((sum[sum.len() - 1] - sum[i] * 2).abs());
            }
        }

        if result == std::i64::MAX { -1 } else { result }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3698() {
        assert_eq!(Solution::split_array(vec![1, 3, 2]), 2);
        assert_eq!(Solution::split_array(vec![1, 2, 4, 3]), 4);
        assert_eq!(Solution::split_array(vec![3, 1, 2]), -1);
    }
}

fn main() {}
