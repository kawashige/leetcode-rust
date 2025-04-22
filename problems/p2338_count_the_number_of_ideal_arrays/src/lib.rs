pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;

    pub fn modinv(a: usize) -> usize {
        let mut a = a as i64;
        let m = 1_000_000_007;
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= m;
        if u < 0 {
            u += m;
        }
        u as usize
    }

    pub fn combination(n: usize, k: usize) -> usize {
        let mut factorial = vec![1; n + 1];
        for i in 1..factorial.len() {
            factorial[i] *= factorial[i - 1] * i;
            factorial[i] %= Self::M;
        }

        ((factorial[n] * Self::modinv(factorial[k])) % Self::M) * Self::modinv(factorial[n - k])
            % Self::M
    }

    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let mut dp = vec![1; max_value as usize + 1];
        let mut result = max_value as usize;
        let mut min = 1;
        let mut max = dp.len() - 1;

        for i in 2..=n as usize {
            let mut new_dp = vec![0; max_value as usize + 1];
            let mul = Self::combination(n as usize - 1, n as usize - i);
            let mut next_min = std::usize::MAX;
            let mut next_max = std::usize::MIN;
            for j in min..max {
                if dp[j] == 0 {
                    continue;
                }
                for k in (j + j..dp.len()).step_by(j) {
                    new_dp[k] += dp[j];
                    new_dp[k] %= Self::M;
                    result += (dp[j] * mul) % Self::M;
                    result %= Self::M;
                    next_min = next_min.min(k);
                    next_max = next_max.max(k);
                }
            }
            dp = new_dp;
            min = next_min;
            max = next_max;
            if min == std::usize::MAX {
                break;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2338() {
        assert_eq!(Solution::ideal_arrays(10000, 10000), 10);
        assert_eq!(Solution::ideal_arrays(2, 5), 10);
        assert_eq!(Solution::ideal_arrays(5, 3), 11);
    }
}
