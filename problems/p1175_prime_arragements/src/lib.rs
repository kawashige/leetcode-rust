pub struct Solution {}

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        const M: usize = 1_000_000_007;
        let n = n as usize;

        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=n {
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let mut prime_count = 0;
        let mut not_prime_count = 0;

        let mut result = 1;
        for i in 1..=n {
            if is_prime[i] {
                prime_count += 1;
                result = result * prime_count % M;
            } else {
                not_prime_count += 1;
                result = result * not_prime_count % M;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1175() {
        assert_eq!(Solution::num_prime_arrangements(1), 1);
        assert_eq!(Solution::num_prime_arrangements(5), 12);
        assert_eq!(Solution::num_prime_arrangements(100), 682289015);
    }
}
