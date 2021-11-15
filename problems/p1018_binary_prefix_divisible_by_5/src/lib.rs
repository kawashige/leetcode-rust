pub struct Solution {}

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut m = 0;
        nums.into_iter()
            .map(|d| {
                m = (m * 2 + d) % 5;
                m % 5 == 0
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1018() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1]),
            vec![true, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1]),
            vec![false, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]),
            vec![true, false, false, false, true, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1, 0, 1]),
            vec![false, false, false, false, false]
        );
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 0, 0, 0, 1, 0, 0, 1]),
            vec![false, false, false, false, false, false, false, false, false]
        );
    }
}
