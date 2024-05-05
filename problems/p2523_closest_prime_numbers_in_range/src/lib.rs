pub struct Solution {}

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut is_prime = vec![true; right as usize + 1];
        let mut result = vec![-1, -1];
        let mut prev = 0;

        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }

            if left <= prev as i32
                && (result[0] == -1 || (((i - prev) as i32) < result[1] - result[0]))
            {
                result = vec![prev as i32, i as i32];
            }
            prev = i;

            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2523() {
        assert_eq!(Solution::closest_primes(10, 19), vec![11, 13]);
        assert_eq!(Solution::closest_primes(4, 6), vec![-1, -1]);
    }
}
