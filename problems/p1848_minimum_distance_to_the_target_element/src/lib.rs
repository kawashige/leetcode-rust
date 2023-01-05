pub struct Solution {}

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        (0..nums.len())
            .filter(|i| nums[*i] == target)
            .map(|i| (i as i32 - start).abs())
            .min()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1848() {
        assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
        assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
        assert_eq!(
            Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0),
            0
        );
    }
}
