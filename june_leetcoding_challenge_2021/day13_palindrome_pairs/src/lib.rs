use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_palilndrome(s: &str) -> bool {
        let b = s.as_bytes();
        for i in 0..(b.len() / 2) {
            if b[i] != b[b.len() - 1 - i] {
                return false;
            }
        }
        true
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let words_rev = words
            .iter()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<String>>();

        let mut words_rev_map = HashMap::new();

        for i in 0..words.len() {
            (*words_rev_map
                .entry((words_rev[i].as_str(), true))
                .or_insert(vec![]))
            .push(i);

            for j in 0..words[i].len() {
                if Self::is_palilndrome(&words_rev[i][j..]) {
                    (*words_rev_map
                        .entry((&words_rev[i][..j], false))
                        .or_insert(vec![]))
                    .push(i);
                }
            }
        }

        let mut result = Vec::new();

        for i in 0..words.len() {
            if words[i].is_empty() {
                continue;
            }

            for j in 0..words[i].len() {
                if Self::is_palilndrome(&words[i][j..]) {
                    if let Some(v) = words_rev_map.get(&(&words[i][..j], true)) {
                        for k in v {
                            if &i == k {
                                continue;
                            }
                            result.push(vec![i as i32, *k as i32]);
                            if words[i][..j].is_empty() {
                                result.push(vec![*k as i32, i as i32]);
                            }
                        }
                    }
                }
            }

            for b in [true, false].iter() {
                if let Some(v) = words_rev_map.get(&(words[i].as_str(), *b)) {
                    for j in v {
                        if &i == j {
                            continue;
                        }
                        result.push(vec![i as i32, *j as i32]);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day13() {
        // assert_eq!(
        //     Solution::palindrome_pairs(vec![
        //         "abcd".to_string(),
        //         "dcba".to_string(),
        //         "lls".to_string(),
        //         "s".to_string(),
        //         "sssll".to_string()
        //     ]),
        //     vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]]
        // );

        assert_eq!(
            Solution::palindrome_pairs(vec![
                "bat".to_string(),
                "tab".to_string(),
                "cat".to_string(),
            ]),
            vec![vec![0, 1], vec![1, 0]]
        );

        assert_eq!(
            Solution::palindrome_pairs(vec!["a".to_string(), "".to_string()]),
            vec![vec![0, 1], vec![1, 0]]
        );
    }
}
