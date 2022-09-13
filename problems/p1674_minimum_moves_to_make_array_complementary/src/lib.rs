pub struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let mut modifications = vec![0; limit as usize * 2 + 2];

        for i in 0..nums.len() / 2 {
            let small = nums[i].min(nums[nums.len() - 1 - i]) as usize;
            let big = nums[i].max(nums[nums.len() - 1 - i]) as usize;
            let sum = big + small;

            modifications[0] += 2;
            modifications[sum - (big - 1)] -= 1;
            modifications[sum] -= 1;
            modifications[sum + 1] += 1;
            modifications[big + limit + 1] += 1;
        }

        let mut min = modifications[0];
        for i in 1..modifications.len() {
            modifications[i] += modifications[i - 1];
            min = min.min(modifications[i]);
        }

        min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1674() {
        assert_eq!(Solution::min_moves(vec![1, 2, 4, 3], 4), 1);
        assert_eq!(Solution::min_moves(vec![1, 2, 2, 1], 2), 2);
        assert_eq!(Solution::min_moves(vec![1, 2, 1, 2], 2), 0);
    }
}
