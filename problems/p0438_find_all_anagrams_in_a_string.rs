pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let l = p.len();
        let p_map = p.as_bytes().into_iter().fold(HashMap::new(), |mut acc, c| {
            (*acc.entry(*c).or_insert(0)) += 1;
            acc
        });
        let mut s_map = HashMap::new();
        let s_bytes = s.as_bytes();
        let mut result = Vec::new();
        for i in 0..s_bytes.len() {
            (*s_map.entry(s_bytes[i]).or_insert(0)) += 1;
            if l - 1 < i {
                if s_map[&s_bytes[i - l]] == 1 {
                    s_map.remove(&s_bytes[i - l]);
                } else {
                    *s_map.get_mut(&s_bytes[i - l]).unwrap() -= 1;
                }
            }
            if l - 1 <= i && p_map == s_map {
                result.push((i - (l - 1)) as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0438() {
        assert_eq!(
            vec![0, 6],
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string())
        );
        assert_eq!(
            vec![0, 1, 2],
            Solution::find_anagrams("abab".to_string(), "ab".to_string())
        );
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::find_anagrams("".to_string(), "".to_string())
        );
    }
}
