pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut primes = Vec::new();
        for i in (2..n).filter(|n| n == &2 || n % 2 != 0) {
            if primes
                .iter()
                .take_while(|p| (**p as f64) < (n as f64).sqrt())
                .all(|p| i % p != 0)
            {
                primes.push(i);
            }
        }
        primes.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0204() {
        assert_eq!(4, Solution::count_primes(10));
        assert_eq!(0, Solution::count_primes(0));
        assert_eq!(0, Solution::count_primes(1));
        assert_eq!(25, Solution::count_primes(100));
        assert_eq!(78497, Solution::count_primes(999983));
    }
}
