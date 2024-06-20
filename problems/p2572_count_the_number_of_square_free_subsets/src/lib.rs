pub struct Solution {}

impl Solution {
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_007;
        const PRIMES: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        let mut factors = vec![];
        for i in 0..nums.len() {
            let mut factor = 0;
            let mut num = nums[i];
            let mut is_square_free = true;
            while 1 < num {
                for j in 0..PRIMES.len() {
                    if num % PRIMES[j] == 0 {
                        factor |= 1 << (j + 1);
                        num /= PRIMES[j];
                        if num % PRIMES[j] == 0 {
                            is_square_free = false;
                            break;
                        }
                    }
                }
                if !is_square_free {
                    break;
                }
            }
            if is_square_free {
                factors.push(factor);
            }
        }

        let mut dp = vec![vec![0; 2_usize.pow(PRIMES.len() as u32 + 1)]; factors.len() + 1];
        dp[0][0] = 1;

        for i in 0..factors.len() {
            for j in 0..dp[0].len() {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= M;
                if factors[i] & j == 0 {
                    dp[i + 1][factors[i] | j] += dp[i][j];
                    dp[i + 1][factors[i] | j] %= M;
                }
            }
        }

        ((M + dp
            .last()
            .unwrap()
            .into_iter()
            .fold(0, |acc, dp| (acc + dp) % M)
            - 1)
            % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2572() {
        assert_eq!(Solution::square_free_subsets(vec![11, 2, 19, 7, 9, 27]), 15);
        assert_eq!(Solution::square_free_subsets(vec![3, 4, 4, 5]), 3);
        assert_eq!(Solution::square_free_subsets(vec![1]), 1);
    }
}
