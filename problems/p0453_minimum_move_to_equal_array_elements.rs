pub struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        nums.iter().map(|n| n - min).sum()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0453() {
        assert_eq!(2147483646, Solution::min_moves(vec![1, 1, 2147483647]));
        assert_eq!(3, Solution::min_moves(vec![1, 2, 3]));
        assert_eq!(0, Solution::min_moves(vec![1]));
        assert_eq!(7, Solution::min_moves(vec![1, 4, 5]));
        assert_eq!(14, Solution::min_moves(vec![1, 2, 7, 8]));
    }
}
