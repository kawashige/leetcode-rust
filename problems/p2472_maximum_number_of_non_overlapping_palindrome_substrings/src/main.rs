pub struct Solution {}

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let mut is_palindrome = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
            is_palindrome[i][i] = true;
            // odd
            let mut l = 1;
            while l <= i && i + l < s.len() && s.as_bytes()[i - l] == s.as_bytes()[i + l] {
                is_palindrome[i - l][i + l] = true;
                l += 1;
            }

            // even
            if i + 1 < s.len() && s.as_bytes()[i] == s.as_bytes()[i + 1] {
                let mut l = 0;
                while l <= i
                    && i + 1 + l < s.len()
                    && s.as_bytes()[i - l] == s.as_bytes()[i + 1 + l]
                {
                    is_palindrome[i - l][i + 1 + l] = true;
                    l += 1;
                }
            }
        }
        println!("{:?}", is_palindrome);

        let mut dp = vec![0; s.len() + 1];
        let k = k as usize;
        for i in k - 1..s.len() {
            dp[i + 1] = dp[i];
            for j in (0..=i - (k - 1)).rev() {
                if is_palindrome[j][i] {
                    dp[i + 1] = dp[i + 1].max(1 + dp[j]);
                }
            }
        }

        *dp.last().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2472() {
        assert_eq!(Solution::max_palindromes("abaccdbbd".to_string(), 3), 2);
        assert_eq!(Solution::max_palindromes("adbcda".to_string(), 2), 0);
    }
}

fn main() {}
