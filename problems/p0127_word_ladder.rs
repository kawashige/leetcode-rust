pub struct Solution {}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet};

        fn make_state(word: &str) -> Vec<String> {
            (0..word.len())
                .map(|i| {
                    word.chars()
                        .enumerate()
                        .map(|(j, c)| if i == j { '*' } else { c })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
        }

        let dictionary = word_list.iter().fold(HashMap::new(), |mut dic, w| {
            make_state(w).into_iter().for_each(|s| {
                dic.entry(s).or_insert(Vec::new()).push(w.to_string());
            });
            dic
        });

        let mut result = 0;
        let mut opened: HashSet<String> = HashSet::new();
        let mut current = vec![begin_word];
        while current.len() > 0 {
            result += 1;
            let mut next = Vec::new();
            for c in current {
                for s in make_state(&c) {
                    match dictionary.get(&s) {
                        Some(words) => {
                            for w in words {
                                if w == &end_word {
                                    return result + 1;
                                }
                                if !opened.contains(w) {
                                    next.push(w.to_string());
                                    opened.insert(w.to_string());
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            current = next;
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0127() {
        assert_eq!(
            0,
            Solution::ladder_length(
                "hot".to_string(),
                "dog".to_string(),
                vec!["hot".to_string(), "dog".to_string()]
            )
        );
        assert_eq!(
            2,
            Solution::ladder_length(
                "a".to_string(),
                "c".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()]
            )
        );
        assert_eq!(
            5,
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            )
        );
        assert_eq!(
            0,
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                ]
            )
        );
        assert_eq!(
            5,
            Solution::ladder_length(
                "qa".to_string(),
                "sq".to_string(),
                vec![
                    "si".to_string(),
                    "go".to_string(),
                    "se".to_string(),
                    "cm".to_string(),
                    "so".to_string(),
                    "ph".to_string(),
                    "mt".to_string(),
                    "db".to_string(),
                    "mb".to_string(),
                    "sb".to_string(),
                    "kr".to_string(),
                    "ln".to_string(),
                    "tm".to_string(),
                    "le".to_string(),
                    "av".to_string(),
                    "sm".to_string(),
                    "ar".to_string(),
                    "ci".to_string(),
                    "ca".to_string(),
                    "br".to_string(),
                    "ti".to_string(),
                    "ba".to_string(),
                    "to".to_string(),
                    "ra".to_string(),
                    "fa".to_string(),
                    "yo".to_string(),
                    "ow".to_string(),
                    "sn".to_string(),
                    "ya".to_string(),
                    "cr".to_string(),
                    "po".to_string(),
                    "fe".to_string(),
                    "ho".to_string(),
                    "ma".to_string(),
                    "re".to_string(),
                    "or".to_string(),
                    "rn".to_string(),
                    "au".to_string(),
                    "ur".to_string(),
                    "rh".to_string(),
                    "sr".to_string(),
                    "tc".to_string(),
                    "lt".to_string(),
                    "lo".to_string(),
                    "as".to_string(),
                    "fr".to_string(),
                    "nb".to_string(),
                    "yb".to_string(),
                    "if".to_string(),
                    "pb".to_string(),
                    "ge".to_string(),
                    "th".to_string(),
                    "pm".to_string(),
                    "rb".to_string(),
                    "sh".to_string(),
                    "co".to_string(),
                    "ga".to_string(),
                    "li".to_string(),
                    "ha".to_string(),
                    "hz".to_string(),
                    "no".to_string(),
                    "bi".to_string(),
                    "di".to_string(),
                    "hi".to_string(),
                    "qa".to_string(),
                    "pi".to_string(),
                    "os".to_string(),
                    "uh".to_string(),
                    "wm".to_string(),
                    "an".to_string(),
                    "me".to_string(),
                    "mo".to_string(),
                    "na".to_string(),
                    "la".to_string(),
                    "st".to_string(),
                    "er".to_string(),
                    "sc".to_string(),
                    "ne".to_string(),
                    "mn".to_string(),
                    "mi".to_string(),
                    "am".to_string(),
                    "ex".to_string(),
                    "pt".to_string(),
                    "io".to_string(),
                    "be".to_string(),
                    "fm".to_string(),
                    "ta".to_string(),
                    "tb".to_string(),
                    "ni".to_string(),
                    "mr".to_string(),
                    "pa".to_string(),
                    "he".to_string(),
                    "lr".to_string(),
                    "sq".to_string(),
                    "ye".to_string()
                ]
            )
        );
    }
}
