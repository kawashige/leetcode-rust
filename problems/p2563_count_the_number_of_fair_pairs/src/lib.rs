pub struct Solution {}

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut l = nums.len() - 1;
        let mut r = nums.len() - 1;
        let mut result = 0;

        for i in 0..nums.len() {
            while 0 < l as i32 && lower <= nums[l - 1] + nums[i] {
                l -= 1;
            }
            while 0 < r && upper < nums[r as usize] + nums[i] {
                r -= 1;
            }
            if r < l || nums[l] + nums[i] < lower || upper < nums[r] + nums[i] {
                continue;
            }

            result += (r - l) as i64 + 1
                - if (l as usize..=r as usize).contains(&i) {
                    1
                } else {
                    0
                };
        }

        result / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2563() {
        assert_eq!(Solution::count_fair_pairs(vec![0], 3, 6), 0);
        assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
        assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
}
