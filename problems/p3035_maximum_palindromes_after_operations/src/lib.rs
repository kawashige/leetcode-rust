pub struct Solution {}

impl Solution {
    pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        let mut count = [0; 26];
        let mut lens = Vec::with_capacity(words.len());
        for i in 0..words.len() {
            for j in 0..words[i].len() {
                count[(words[i].as_bytes()[j] - b'a') as usize] += 1;
            }
            lens.push(words[i].len());
        }
        lens.sort_unstable();
        let mut ones = 0;
        for i in 0..count.len() {
            if count[i] % 2 == 1 {
                ones += 1;
                count[i] -= 1;
            }
        }
        let mut result = 0;
        for i in 0..lens.len() {
            let mut remains = lens[i];
            if remains % 2 == 1 && 0 < ones {
                ones -= 1;
                remains -= 1;
            }
            for i in 0..count.len() {
                let d = remains.min(count[i]);
                remains -= d;
                count[i] -= d;
                if remains == 0 {
                    break;
                }
            }
            if remains == 0 {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3035() {
        assert_eq!(
            Solution::max_palindromes_after_operations(vec![
                "abbb".to_string(),
                "ba".to_string(),
                "aa".to_string()
            ]),
            3
        );
        assert_eq!(
            Solution::max_palindromes_after_operations(vec!["abc".to_string(), "ab".to_string()]),
            2
        );
        assert_eq!(
            Solution::max_palindromes_after_operations(vec![
                "cd".to_string(),
                "ef".to_string(),
                "a".to_string()
            ]),
            1
        );
    }
}
