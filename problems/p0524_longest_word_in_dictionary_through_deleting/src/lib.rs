pub struct Solution {}

impl Solution {
    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let mut char_indexes = vec![Vec::new() as Vec<usize>; 26];
        for (i, c) in s.chars().enumerate() {
            char_indexes[c as usize - 0x61].push(i);
        }

        let mut d = d;
        d.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(&b)));

        for dic in d {
            let mut i = -1;
            let mut found = true;
            for c in dic.chars() {
                if let Some(j) = char_indexes[c as usize - 0x61]
                    .iter()
                    .find(|j| i < (**j as i32))
                {
                    i = *j as i32;
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                return dic;
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0524() {
        assert_eq!(
            "apple".to_string(),
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec![
                    "ale".to_string(),
                    "apple".to_string(),
                    "monkey".to_string(),
                    "plea".to_string()
                ]
            )
        );

        assert_eq!(
            "a".to_string(),
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()]
            )
        );

        assert_eq!(
            "".to_string(),
            Solution::find_longest_word(
                "abcd".to_string(),
                vec!["e".to_string(), "fg".to_string(), "abcdx".to_string()]
            )
        );

        assert_eq!(
            "".to_string(),
            Solution::find_longest_word(
                "".to_string(),
                vec!["e".to_string(), "fg".to_string(), "abcdx".to_string()]
            )
        );

        assert_eq!(
            "".to_string(),
            Solution::find_longest_word(
                "".to_string(),
                vec!["".to_string(), "fg".to_string(), "abcdx".to_string()]
            )
        );

        assert_eq!(
            "ab".to_string(),
            Solution::find_longest_word(
                "bab".to_string(),
                vec![
                    "ba".to_string(),
                    "ab".to_string(),
                    "a".to_string(),
                    "b".to_string()
                ]
            )
        );
    }
}
