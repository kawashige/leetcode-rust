pub struct Solution {}

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|n| {
                n.to_string()
                    .as_bytes()
                    .iter()
                    .map(|b| (b - b'0') as i32)
                    .sum()
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3300() {
        assert_eq!(Solution::min_element(vec![10, 12, 13, 14]), 1);
        assert_eq!(Solution::min_element(vec![1, 2, 3, 4]), 1);
        assert_eq!(Solution::min_element(vec![999, 19, 199]), 10);
    }
}
