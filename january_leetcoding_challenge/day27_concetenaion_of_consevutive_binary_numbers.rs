pub struct Solution {}

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (1..=n)
            .rev()
            .fold((1_i64, 0_i64), |(mul, sum), i| {
                (
                    mul * 2_i64.pow(32 - i.leading_zeros()) % 1_000_000_007,
                    (sum + (mul * i as i64)) % 1_000_000_007,
                )
            })
            .1 as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day27() {
        assert_eq!(Solution::concatenated_binary(1), 1);
        assert_eq!(Solution::concatenated_binary(3), 27);
        assert_eq!(Solution::concatenated_binary(12), 505379714);
    }
}
