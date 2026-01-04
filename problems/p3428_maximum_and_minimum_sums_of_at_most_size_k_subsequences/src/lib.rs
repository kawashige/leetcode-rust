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

    pub fn combination(n: usize, k: usize, factorial: &[usize]) -> usize {
        ((factorial[n] * Self::modinv(factorial[k])) % Self::M) * Self::modinv(factorial[n - k])
            % Self::M
    }
    pub fn min_max_sums(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut result = 0;
        let k = k as usize;

        let mut factorial = vec![1; nums.len()];
        for i in 1..factorial.len() {
            factorial[i] *= factorial[i - 1] * i;
            factorial[i] %= Self::M;
        }

        let mut combi = vec![vec![0; k as usize]; nums.len()];

        for i in 0..nums.len() {
            result += nums[i] as usize * 2;
            result %= Self::M;

            let small = i;
            for j in 1..=i.min(k - 1) {
                if combi[small][j] == 0 {
                    combi[small][j] = Self::combination(small, j, &factorial);
                }
                result += nums[i] as usize * combi[small][j];
                result %= Self::M;
            }
            let big = nums.len() - 1 - i;
            for j in 1..=(nums.len() - 1 - i).min(k - 1) {
                if combi[big][j] == 0 {
                    combi[big][j] = Self::combination(big, j, &factorial);
                }
                result += nums[i] as usize * combi[big][j];
                result %= Self::M;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3428() {
        assert_eq!(Solution::min_max_sums(vec![0, 1, 1], 3), 9);
        assert_eq!(Solution::min_max_sums(vec![1, 2, 3], 2), 24);
        assert_eq!(Solution::min_max_sums(vec![5, 0, 6], 1), 22);
        assert_eq!(Solution::min_max_sums(vec![1, 1, 1], 2), 12);
    }
}
