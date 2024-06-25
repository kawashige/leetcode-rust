pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: usize, nums: &[i32]) -> bool {
        for i in 0..mid {
            if nums[mid - 1 - i] * 2 > nums[nums.len() - 1 - i] {
                return false;
            }
        }
        true
    }

    pub fn max_num_of_marked_indices(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut ok = 0;
        let mut ng = nums.len() / 2 + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &nums) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32 * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2576() {
        assert_eq!(Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]), 2);
        assert_eq!(Solution::max_num_of_marked_indices(vec![9, 2, 5, 4]), 4);
        assert_eq!(Solution::max_num_of_marked_indices(vec![7, 6, 8]), 0);
    }
}
