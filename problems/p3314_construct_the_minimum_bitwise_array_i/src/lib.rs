pub struct Solution {}

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|n| (0..n).find(|x| (x | x + 1) == n).unwrap_or(-1))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3314() {
        assert_eq!(
            Solution::min_bitwise_array(vec![2, 3, 5, 7]),
            vec![-1, 1, 4, 3]
        );
        assert_eq!(
            Solution::min_bitwise_array(vec![11, 13, 31]),
            vec![9, 12, 15]
        );
    }
}
