use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let chars = expression.chars().collect::<Vec<_>>();
        let mut level = vec![0; chars.len()];
        for i in 0..chars.len() {
            if 0 < i {
                level[i] = level[i - 1];
            }
            match chars[i] {
                '{' => level[i] += 1,
                '}' => level[i] -= 1,
                _ => {}
            }
        }

        Self::parse(&chars, 0, expression.len() - 1, &level)
    }

    pub fn parse(chars: &[char], l: usize, r: usize, level: &[usize]) -> Vec<String> {
        let mut i = l;
        let mut result = BTreeSet::new();
        let mut tmp = Vec::new();
        while i <= r {
            match chars[i] {
                '{' => {
                    let j = (i + 1..=r)
                        .find(|j| level[*j] == level[i] - 1 && chars[*j] == '}')
                        .unwrap();
                    let mut new_tmp = Vec::new();
                    let parsed = Self::parse(chars, i + 1, j - 1, level);
                    if tmp.is_empty() {
                        tmp = parsed;
                    } else {
                        for r in parsed {
                            for t in &tmp {
                                new_tmp.push(format!("{}{}", t, r));
                            }
                        }
                        tmp = new_tmp;
                    }
                    i = j + 1;
                }
                ',' => {
                    for r in &tmp {
                        result.insert(r.clone());
                    }
                    tmp.clear();
                    i += 1;
                }
                _ => {
                    if tmp.is_empty() {
                        tmp.push(chars[i].to_string());
                    } else {
                        tmp = tmp
                            .into_iter()
                            .map(|t| format!("{}{}", t, chars[i]))
                            .collect();
                    }
                    i += 1;
                }
            }
        }
        for t in tmp {
            result.insert(t);
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1096() {
        assert_eq!(
            Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
            vec![
                "ac".to_string(),
                "ad".to_string(),
                "ae".to_string(),
                "bc".to_string(),
                "bd".to_string(),
                "be".to_string()
            ]
        );
        assert_eq!(
            Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
            vec![
                "a".to_string(),
                "ab".to_string(),
                "ac".to_string(),
                "z".to_string()
            ]
        );
    }
}

fn main() {}
