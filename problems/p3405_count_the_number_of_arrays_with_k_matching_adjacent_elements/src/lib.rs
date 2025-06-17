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

    pub fn pow(x: usize, n: usize) -> usize {
        let mut result = 1;
        let mut n = n;
        let mut x = x;
        while 0 < n {
            if n & 1 == 1 {
                result = (result * x) % Self::M;
            }
            x = (x * x) % Self::M;
            n >>= 1;
        }
        result
    }

    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut result = Self::combination(n as usize - 1, k as usize);
        result = (result * m as usize) % Self::M;
        result = (result * Self::pow(m as usize - 1, (n - k) as usize - 1)) % Self::M;
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3405() {
        assert_eq!(Solution::count_good_arrays(3, 2, 1), 4);
        assert_eq!(Solution::count_good_arrays(4, 2, 2), 6);
        assert_eq!(Solution::count_good_arrays(5, 2, 0), 2);
    }
}
