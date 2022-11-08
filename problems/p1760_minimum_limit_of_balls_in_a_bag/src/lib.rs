pub struct Solution {}

impl Solution {
    pub fn is_ok(nums: &[i32], max_operations: i32, target: i32) -> bool {
        let mut remains = max_operations;

        for i in (0..nums.len()).rev() {
            remains -= nums[i] / target + if nums[i] % target == 0 { -1 } else { 0 };
            if remains < 0 {
                return false;
            }
        }
        true
    }

    pub fn minimum_size(mut nums: Vec<i32>, max_operations: i32) -> i32 {
        nums.sort_unstable();

        let mut ok = nums[nums.len() - 1];
        let mut ng = 0;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(&nums, max_operations, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1760() {
        assert_eq!(Solution::minimum_size(vec![7, 17], 2), 7);
        assert_eq!(Solution::minimum_size(vec![9], 2), 3);
        assert_eq!(Solution::minimum_size(vec![2, 4, 8, 2], 4), 2);
    }
}
