pub struct Solution {}

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| a.abs().cmp(&b.abs()).then((-1 * a).cmp(&(-1 * b))));
        nums[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2239() {
        assert_eq!(Solution::find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
        assert_eq!(Solution::find_closest_number(vec![2, -1, 1]), 1);
    }
}
