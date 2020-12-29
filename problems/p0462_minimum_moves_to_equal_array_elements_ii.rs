pub struct Solution {}

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let m = nums[nums.len() / 2];
        nums.into_iter().map(|n| (n - m).abs()).sum()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0462() {
        assert_eq!(2, Solution::min_moves2(vec![1, 2, 3]));
        assert_eq!(0, Solution::min_moves2(vec![1]));
        assert_eq!(13, Solution::min_moves2(vec![1, 1, 1, 5, 10]));
    }
}
