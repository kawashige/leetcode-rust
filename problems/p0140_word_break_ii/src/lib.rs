pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        dict_indices: &Vec<Vec<usize>>,
        word_dict: &[String],
        indices: &mut Vec<usize>,
        result: &mut Vec<String>,
    ) {
        if i == dict_indices.len() {
            result.push(
                indices
                    .iter()
                    .map(|j| word_dict[*j].clone())
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            return;
        }

        for j in &dict_indices[i] {
            indices.push(*j);
            Self::recurse(
                i + word_dict[*j].len(),
                dict_indices,
                word_dict,
                indices,
                result,
            );
            indices.pop();
        }
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut dict_indices = vec![vec![]; s.len()];
        for i in 0..s.len() {
            for j in 0..word_dict.len() {
                if s[i..].starts_with(&word_dict[j]) {
                    dict_indices[i].push(j);
                }
            }
        }

        let mut result = Vec::new();
        Self::recurse(0, &dict_indices, &word_dict, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1400() {
        assert_eq!(
            Solution::word_break(
                "catsanddog".to_string(),
                vec![
                    "cat".to_string(),
                    "cats".to_string(),
                    "and".to_string(),
                    "sand".to_string(),
                    "dog".to_string()
                ]
            ),
            vec!["cat sand dog".to_string(), "cats and dog".to_string()]
        );
        assert_eq!(
            Solution::word_break(
                "pineapplepenapple".to_string(),
                vec![
                    "apple".to_string(),
                    "pen".to_string(),
                    "applepen".to_string(),
                    "pine".to_string(),
                    "pineapple".to_string()
                ]
            ),
            vec![
                "pine apple pen apple".to_string(),
                "pine applepen apple".to_string(),
                "pineapple pen apple".to_string(),
            ]
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            vec![] as Vec<String>
        );
    }
}
