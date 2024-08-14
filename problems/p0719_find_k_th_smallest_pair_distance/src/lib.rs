pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i32, nums: &[i32], k: i32) -> bool {
        let mut j = 0;
        let mut count = 0;
        for i in 0..nums.len() {
            while j + 1 < nums.len() && nums[j + 1] - nums[i] <= mid {
                j += 1;
            }
            count += j - i;
        }
        k <= count as i32
    }

    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut ng = -1;
        let mut ok = nums[nums.len() - 1] - nums[0] + 1;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &nums, k) {
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
    fn test_0719() {
        assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
        assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
        assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
    }
}
