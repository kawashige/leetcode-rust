pub struct Solution {}

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        (left..=right).all(|i| ranges.iter().any(|range| range[0] <= i && i <= range[1]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1893() {
        assert!(Solution::is_covered(
            vec![vec![1, 2], vec![3, 4], vec![5, 6]],
            2,
            5
        ));
        assert!(!Solution::is_covered(
            vec![vec![1, 10], vec![10, 20]],
            21,
            32
        ));
    }
}
