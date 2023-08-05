pub struct Solution {}

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let mut min = nums[0];
        let mut min_i = 0;
        let mut max = nums[0];
        let mut max_i = 0;

        for i in 0..nums.len() {
            if nums[i] < min {
                min = nums[i];
                min_i = i;
            }
            if max < nums[i] {
                max = nums[i];
                max_i = i;
            }
        }

        (min_i.min(max_i) + 1 + nums.len() - min_i.max(max_i))
            .min(min_i.max(max_i) + 1)
            .min(nums.len() - min_i.min(max_i)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2091() {
        assert_eq!(
            Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]),
            5
        );
        assert_eq!(
            Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]),
            3
        );
        assert_eq!(Solution::minimum_deletions(vec![101]), 1);
    }
}
