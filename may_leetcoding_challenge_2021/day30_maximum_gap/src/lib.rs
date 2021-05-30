pub struct Solution {}

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        nums.sort_unstable();
        (1..nums.len())
            .map(|i| nums[i] - nums[i - 1])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day30() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }
}
