pub struct Solution {}

impl Solution {
    pub fn is_ok(nums: &[i32], freq: usize, k: i32) -> bool {
        let mut sum = (0..freq - 1).map(|i| nums[i]).sum::<i32>();

        for i in freq - 1..nums.len() {
            if nums[i] * (freq as i32 - 1) - sum <= k {
                return true;
            }
            sum += nums[i] - nums[i + 1 - freq]
        }

        false
    }

    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut ok = 1;
        let mut ng = nums.len() + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(&nums, mid, k) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1838() {
        assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
        assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
        assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);
    }
}
