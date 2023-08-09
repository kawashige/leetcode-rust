pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i32, nums: &[i32], p: i32) -> bool {
        let mut i = 1;
        let mut remains = p;

        while i < nums.len() {
            if nums[i] - nums[i - 1] <= mid {
                remains -= 1;
                if remains == 0 {
                    return true;
                }
                i += 2;
            } else {
                i += 1;
            }
        }

        false
    }

    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        if p == 0 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();

        let mut ng = -1;
        let mut ok = nums.last().unwrap() - nums[0];

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if Self::is_ok(mid, &nums, p) {
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
    fn test_2616() {
        assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
        assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
    }
}
