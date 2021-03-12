pub struct Solution {}

impl Solution {
    pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut dp1 = 0;
        let mut dp2 = 0;

        for i in 0..nums.len() {
            if 0 < i && nums[i - 1] == nums[i] {
                dp1 = dp1 + nums[i];
            } else if 0 < i && nums[i - 1] + 1 == nums[i] {
                let max = std::cmp::max(dp1, dp2);
                dp1 = dp2 + nums[i];
                dp2 = max;
            } else {
                let max = std::cmp::max(dp1, dp2);
                dp1 = max + nums[i];
                dp2 = max;
            }
        }

        std::cmp::max(dp1, dp2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0740() {
        assert_eq!(
            Solution::delete_and_earn(vec![
                10, 8, 4, 2, 1, 3, 4, 8, 2, 9, 10, 4, 8, 5, 9, 1, 5, 1, 6, 8, 1, 1, 6, 7, 8, 9, 1,
                7, 6, 8, 4, 5, 4, 1, 5, 9, 8, 6, 10, 6, 4, 3, 8, 4, 10, 8, 8, 10, 6, 4, 4, 4, 9, 6,
                9, 10, 7, 1, 5, 3, 4, 4, 8, 1, 1, 2, 1, 4, 1, 1, 4, 9, 4, 7, 1, 5, 1, 10, 3, 5, 10,
                3, 10, 2, 1, 10, 4, 1, 1, 4, 1, 2, 10, 9, 7, 10, 1, 2, 7, 5
            ]),
            338
        );
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
