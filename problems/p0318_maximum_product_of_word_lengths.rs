pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut bits = Vec::new();
        let mut lengths = Vec::new();

        let alphabets = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
        for w in words {
            let chars: HashSet<char> = w.chars().collect();
            let mut i = 1;
            let mut n = 0;
            for a in &alphabets {
                if chars.contains(a) {
                    n |= 1 << i;
                }
                i += 1;
            }
            bits.push(n);
            lengths.push(w.len() as i32);
        }

        let mut result = 0;
        for i in 0..bits.len() {
            for j in (i + 1)..bits.len() {
                if bits[i] & bits[j] == 0 {
                    result = std::cmp::max(result, lengths[i] * lengths[j]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0318() {
        assert_eq!(
            16,
            Solution::max_product(vec![
                "abcw".to_string(),
                "baz".to_string(),
                "foo".to_string(),
                "bar".to_string(),
                "xtfn".to_string(),
                "abcdef".to_string()
            ])
        );
        assert_eq!(
            4,
            Solution::max_product(vec![
                "a".to_string(),
                "ab".to_string(),
                "abc".to_string(),
                "d".to_string(),
                "cd".to_string(),
                "bcd".to_string(),
                "abcd".to_string()
            ])
        );
        assert_eq!(
            0,
            Solution::max_product(vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string()
            ])
        );
    }
}
