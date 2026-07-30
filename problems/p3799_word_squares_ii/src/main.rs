pub struct Solution {}

impl Solution {
    pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut words = words;
        words.sort_unstable();

        for top in 0..words.len() {
            for left in 0..words.len() {
                if left == top {
                    continue;
                }
                for right in 0..words.len() {
                    if right == top || right == left {
                        continue;
                    }
                    for bottom in 0..words.len() {
                        if bottom == top || bottom == left || bottom == right {
                            continue;
                        }
                        if words[top].as_bytes()[0] == words[left].as_bytes()[0]
                            && words[top].as_bytes()[3] == words[right].as_bytes()[0]
                            && words[bottom].as_bytes()[0] == words[left].as_bytes()[3]
                            && words[bottom].as_bytes()[3] == words[right].as_bytes()[3]
                        {
                            result.push(vec![
                                words[top].clone(),
                                words[left].clone(),
                                words[right].clone(),
                                words[bottom].clone(),
                            ]);
                        }
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
    fn test_3799() {
        assert_eq!(
            Solution::word_squares(vec![
                "able".to_string(),
                "area".to_string(),
                "echo".to_string(),
                "also".to_string()
            ]),
            vec![
                vec![
                    "able".to_string(),
                    "area".to_string(),
                    "echo".to_string(),
                    "also".to_string()
                ],
                vec![
                    "area".to_string(),
                    "able".to_string(),
                    "also".to_string(),
                    "echo".to_string()
                ]
            ]
        );
        assert_eq!(
            Solution::word_squares(vec![
                "code".to_string(),
                "cafe".to_string(),
                "eden".to_string(),
                "edge".to_string()
            ]),
            vec![] as Vec<Vec<String>>
        );
    }
}

fn main() {}
