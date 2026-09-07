pub struct Solution {}

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const M: usize = 1_000_000_007;
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        let mut last_index = [s.len(); 26];

        for i in 0..s.len() {
            let j = (s.as_bytes()[i] - b'a') as usize;
            dp[i + 1] = ((dp[i] * 2) % M + M
                - if last_index[j] == s.len() {
                    0
                } else {
                    dp[last_index[j]]
                })
                % M;
            last_index[j] = i;
        }

        ((dp.into_iter().last().unwrap() + M - 1) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0940() {
        assert_eq!(Solution::distinct_subseq_ii("abc".to_string()), 7);
        assert_eq!(Solution::distinct_subseq_ii("aba".to_string()), 6);
        assert_eq!(Solution::distinct_subseq_ii("aaa".to_string()), 3);
    }
}

fn main() {}
