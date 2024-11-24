pub struct Solution {}

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let n = n as usize;
        let target = target as usize;

        if n < target / 2 {
            return ((n * (n + 1) / 2) % M) as i32;
        }

        ((((target / 2 * (target / 2 + 1) / 2) % M
            + (target + (target + n - target / 2 - 1)) * (n - target / 2) / 2)
            % M)
            % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2384() {
        assert_eq!(
            Solution::minimum_possible_sum(100000000, 1000000000),
            15000000
        );
        assert_eq!(Solution::minimum_possible_sum(2, 3), 4);
        assert_eq!(Solution::minimum_possible_sum(3, 3), 8);
        assert_eq!(Solution::minimum_possible_sum(1, 1), 1);
    }
}
