pub struct Solution {}

impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        (l..=r).filter(|n| primes.contains(&n.count_ones())).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0762() {
        assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
        assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
    }
}
