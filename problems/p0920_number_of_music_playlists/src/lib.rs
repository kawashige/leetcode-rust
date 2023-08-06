use std::collections::HashMap;

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

    pub fn calc(n: usize, goal: usize, k: usize) -> usize {
        let mut count = 1;

        for i in 0..goal {
            count *= n - i.min(k);
            count %= Self::M;
        }

        count
    }

    pub fn recurse(n: usize, goal: usize, k: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(count) = memo.get(&n) {
            return *count;
        }

        let mut count = Self::calc(n, goal, k);
        for i in (k + 1..n).rev() {
            count = count + Self::M
                - (Self::combination(n as usize, n - i) * Self::recurse(i, goal, k, memo))
                    % Self::M;
            count %= Self::M;
        }

        memo.insert(n, count);
        count
    }

    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        Self::recurse(n as usize, goal as usize, k as usize, &mut HashMap::new()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0920() {
        assert_eq!(Solution::num_music_playlists(30, 34, 4), 6);
        assert_eq!(Solution::num_music_playlists(3, 3, 0), 6);
        assert_eq!(Solution::num_music_playlists(3, 3, 1), 6);
        assert_eq!(Solution::num_music_playlists(2, 3, 0), 6);
        assert_eq!(Solution::num_music_playlists(2, 3, 1), 2);
    }
}
