pub struct Solution {}

impl Solution {
    pub fn recurse(pattern: &mut Vec<i32>, m: usize, patterns: &mut Vec<i32>) {
        if pattern.len() == m {
            patterns.push(pattern.iter().fold(0, |acc, x| acc * 10 + x));
            return;
        }

        for i in 1..4 {
            if pattern.last() == Some(&i) {
                continue;
            }
            pattern.push(i);
            Self::recurse(pattern, m, patterns);
            pattern.pop();
        }
    }

    pub fn check(i: i32, j: i32) -> bool {
        let mut i = i;
        let mut j = j;
        while 0 < i && 0 < j {
            if i % 10 == j % 10 {
                return false;
            }
            i /= 10;
            j /= 10;
        }
        true
    }
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut patterns = Vec::new();
        Self::recurse(&mut Vec::new(), m as usize, &mut patterns);

        let mut dp = vec![vec![0; patterns.len()]; n as usize];
        for i in 0..dp[0].len() {
            dp[0][i] = 1;
        }

        let mut check = vec![vec![false; patterns.len()]; patterns.len()];
        for i in 0..patterns.len() {
            for j in 0..=i {
                if Self::check(patterns[i], patterns[j]) {
                    check[i][j] = true;
                    check[j][i] = true;
                }
            }
        }

        for i in 1..dp.len() {
            for j in 0..dp[i].len() {
                for k in 0..dp[i].len() {
                    if check[j][k] {
                        dp[i][j] += dp[i - 1][k];
                        dp[i][j] %= M;
                    }
                }
            }
        }

        dp.into_iter()
            .last()
            .unwrap()
            .into_iter()
            .fold(0, |acc, x| (acc + x) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1931() {
        assert_eq!(Solution::color_the_grid(1, 1), 3);
        assert_eq!(Solution::color_the_grid(1, 2), 6);
        assert_eq!(Solution::color_the_grid(5, 5), 580986);
    }
}
