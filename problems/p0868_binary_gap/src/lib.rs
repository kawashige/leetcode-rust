pub struct Solution {}

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        (0..32)
            .filter(|i| n & 1 << i > 0)
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|v| v[1] - v[0])
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0868() {
        assert_eq!(Solution::binary_gap(22), 2);
        assert_eq!(Solution::binary_gap(5), 2);
        assert_eq!(Solution::binary_gap(6), 1);
        assert_eq!(Solution::binary_gap(8), 0);
        assert_eq!(Solution::binary_gap(1), 0);
    }
}
