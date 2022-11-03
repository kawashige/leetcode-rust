pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut count = vec![vec![0; 26]; 26];

        for word in words {
            count[(word.as_bytes()[0] - b'a') as usize][(word.as_bytes()[1] - b'a') as usize] += 1;
        }

        let mut length = 0;
        let mut has_center = false;

        for i in 0..26 {
            for j in i..26 {
                if i == j {
                    length += count[i][j] / 2 * 4;
                    if count[i][j] % 2 == 1 {
                        has_center = true;
                    }
                } else {
                    length += count[i][j].min(count[j][i]) * 4;
                }
            }
        }

        length + if has_center { 2 } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2131() {
        assert_eq!(
            Solution::longest_palindrome(vec![
                "lc".to_string(),
                "cl".to_string(),
                "gg".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::longest_palindrome(vec![
                "ab".to_string(),
                "ty".to_string(),
                "yt".to_string(),
                "lc".to_string(),
                "cl".to_string(),
                "ab".to_string()
            ]),
            8
        );
        assert_eq!(
            Solution::longest_palindrome(vec![
                "cc".to_string(),
                "ll".to_string(),
                "xx".to_string()
            ]),
            2
        );
    }
}
