pub struct Solution {}

impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let mut is_prime = vec![true; 1001];
        let mut count = 0;

        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }
            for num in &nums {
                if num % i as i32 == 0 {
                    count += 1;
                    break;
                }
            }
            for k in (i + i..is_prime.len()).step_by(i) {
                is_prime[k] = false;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2521() {
        assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 3, 7, 10, 6]), 4);
        assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 8, 16]), 1);
    }
}
