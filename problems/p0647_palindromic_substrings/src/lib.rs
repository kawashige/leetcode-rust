pub struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let l = s.len();
        let mut is_palindrome = vec![vec![false; l]; l];
        for i in 0..l {
            is_palindrome[i][i] = true;
        }
        let bytes = s.as_bytes();
        let mut count = l as i32;
        for j in 1..l {
            for i in 0..(l - j) {
                if bytes[i] == bytes[i + j] && (j == 1 || is_palindrome[i + 1][i + j - 1]) {
                    is_palindrome[i][i + j] = true;
                    count += 1
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0647() {
        assert_eq!(Solution::count_substrings("".to_string()), 0);
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
        assert_eq!(Solution::count_substrings("abccba".to_string()), 9);
    }
}
