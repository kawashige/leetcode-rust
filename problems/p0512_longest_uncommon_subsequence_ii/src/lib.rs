pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut counts = strs
            .into_iter()
            .fold(HashMap::new(), |mut acc, s| {
                *acc.entry(s).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .collect::<Vec<(String, i32)>>();
        counts.sort_by_key(|(k, _)| -(k.len() as i32));

        let mut longer_strings: Vec<Vec<char>> = Vec::new();
        for (s, c) in counts {
            let chars: Vec<char> = s.chars().collect();
            if c == 1
                && (longer_strings.is_empty()
                    || longer_strings.iter().all(|l| {
                        let mut i = 0;
                        let mut j = 0;
                        while i < chars.len() && j < l.len() {
                            if let Some(k) = (j..l.len()).find(|k| chars[i] == l[*k]) {
                                j = k + 1;
                                i += 1;
                            } else {
                                break;
                            }
                        }
                        i != chars.len()
                    }))
            {
                return chars.len() as i32;
            }
            longer_strings.push(chars);
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0512() {
        assert_eq!(
            3,
            Solution::find_lu_slength(vec![
                "abcabc".to_string(),
                "abcabc".to_string(),
                "abcabc".to_string(),
                "abc".to_string(),
                "abc".to_string(),
                "cca".to_string()
            ])
        );
        assert_eq!(
            3,
            Solution::find_lu_slength(vec![
                "aba".to_string(),
                "cdc".to_string(),
                "eae".to_string()
            ])
        );
        assert_eq!(
            -1,
            Solution::find_lu_slength(vec![
                "abcd".to_string(),
                "abcd".to_string(),
                "abc".to_string()
            ])
        );
        assert_eq!(
            1,
            Solution::find_lu_slength(vec![
                "abcd".to_string(),
                "abcd".to_string(),
                "x".to_string()
            ])
        );
        assert_eq!(
            -1,
            Solution::find_lu_slength(vec!["abcd".to_string(), "abcd".to_string(), "".to_string()])
        );
    }
}
