pub struct Solution {}

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let tmp = s
            .chars()
            .chain(std::iter::once('*'))
            .chain(s.chars().rev())
            .collect::<Vec<_>>();

        let n = tmp.len();
        let mut lps = vec![0; n];

        for i in 1..n {
            let mut len = lps[i - 1];
            while 0 < len && tmp[len] != tmp[i] {
                len = lps[len - 1];
            }
            if tmp[i] == tmp[len] {
                len += 1;
            }
            lps[i] = len
        }

        s[lps[n - 1]..].chars().rev().collect::<String>() + &s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0214() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_string()),
            "aaacecaaa".to_string()
        );
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_string()),
            "dcbabcd".to_string()
        );
    }
}
