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

    pub fn count_visible_people(n: i32, pos: i32, k: i32) -> i32 {
        if n - 1 < k {
            return 0;
        }
        if k == 0 {
            return 2;
        }

        let mut result = 0;
        let pos = pos as usize;
        let k = k as usize;
        let n = n as usize;

        let mut factorial = vec![1; n + 1];
        for i in 1..factorial.len() {
            factorial[i] *= factorial[i - 1] * i;
            factorial[i] %= Self::M;
        }
        for i in (k.saturating_sub(n - 1 - pos))..=pos.min(k) {
            result += (Self::combination(pos, i, &factorial)
                * Self::combination(n - 1 - pos, k - i, &factorial))
                % Self::M;
            result %= Self::M;
        }

        ((result * 2) % Self::M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3881() {
        assert_eq!(Solution::count_visible_people(3, 0, 1), 4);
        assert_eq!(Solution::count_visible_people(3, 1, 0), 2);
        assert_eq!(Solution::count_visible_people(3, 2, 1), 4);
        assert_eq!(Solution::count_visible_people(1, 0, 0), 2);
    }
}

fn main() {}
