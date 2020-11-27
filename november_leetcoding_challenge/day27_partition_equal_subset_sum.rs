pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let half = sum / 2;
        let mut dp = vec![false; half as usize + 1];
        dp[0] = true;
        for n in nums {
            if half < n {
                return false;
            }
            if dp[(half - n) as usize] {
                return true;
            }
            for i in (n..half).rev() {
                if dp[(i - n) as usize] {
                    dp[i as usize] = true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day27() {
        assert!(!Solution::can_partition(vec![1, 2, 5]));
        assert!(Solution::can_partition(vec![3, 3, 3, 4, 5]));
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }
}
