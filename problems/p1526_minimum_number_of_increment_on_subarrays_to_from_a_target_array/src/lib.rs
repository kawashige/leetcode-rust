pub struct Solution {}

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let n = target.len();
        let mut ans = target[0];
        for i in 1..n {
            ans += std::cmp::max(target[i] - target[i - 1], 0);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1526() {
        assert_eq!(Solution::min_number_operations(vec![1, 2, 3, 2, 1]), 3);
        assert_eq!(Solution::min_number_operations(vec![3, 1, 1, 2]), 4);
        assert_eq!(Solution::min_number_operations(vec![3, 1, 5, 4, 2]), 7);
    }
}
