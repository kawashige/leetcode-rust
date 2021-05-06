pub struct Solution {}

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        fn m(a: usize, b: usize) -> usize {
            if a >= b {
                a - b
            } else {
                0
            }
        }

        let n = ((n + 24) / 25) as usize;
        if n >= 500 {
            return 1.0;
        }

        let mut memo = vec![vec![0.0; n + 1]; n + 1];

        for s in 0..=(2 * n) {
            for i in 0..=n {
                if s < i || s - i > n {
                    continue;
                }
                let j = s - i;
                let r = if i == 0 && j == 0 {
                    0.5
                } else if i == 0 {
                    1.0
                } else if i > 0 && j > 0 {
                    0.25 * (memo[m(i, 4)][j]
                        + memo[m(i, 3)][m(j, 1)]
                        + memo[m(i, 2)][m(j, 2)]
                        + memo[m(i, 1)][m(j, 3)])
                } else {
                    0.0
                };
                memo[i][j] = r;
            }
        }
        memo[n][n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0808() {
        assert_eq!(Solution::soup_servings(50), 0.625);
        assert_eq!(Solution::soup_servings(660295675), 1.0);
    }
}
