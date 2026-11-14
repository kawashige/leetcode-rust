pub struct Solution {}

impl Solution {
    pub fn min_increase(nums: Vec<i32>) -> i64 {
        let mut dp = vec![0; nums.len() + 1];
        for i in (1..nums.len() - 1).rev() {
            if i % 2 != nums.len() % 2 {
                continue;
            }
            dp[i] += dp[i + 2];
            dp[i] +=
                ((nums[i + 1] - nums[i] + 1).max(0)).max((nums[i - 1] - nums[i] + 1).max(0)) as i64;
        }

        if nums.len() % 2 == 1 {
            return dp[1];
        }

        let mut cur = 0;
        let mut min = std::i64::MAX;
        for i in (1..nums.len() - 1).step_by(2) {
            min = min.min(cur + dp[i + 1]);
            cur +=
                ((nums[i + 1] - nums[i] + 1).max(0)).max((nums[i - 1] - nums[i] + 1).max(0)) as i64;
        }

        min.min(cur)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3891() {
        assert_eq!(Solution::min_increase(vec![1, 2, 2]), 1);
        assert_eq!(Solution::min_increase(vec![2, 1, 1, 3]), 2);

        assert_eq!(Solution::min_increase(vec![5, 2, 1, 4, 3]), 4);
    }
}

fn main() {}
