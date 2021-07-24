use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn distance(s1: &str, s2: &str) -> usize {
        s1.as_bytes()
            .iter()
            .zip(s2.as_bytes().iter())
            .filter(|(b1, b2)| b1 != b2)
            .count()
    }

    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let e = (0..word_list.len())
            .find(|i| word_list[*i] == end_word)
            .unwrap_or(word_list.len());
        if e == word_list.len() {
            return Default::default();
        }

        let s = word_list.len();
        word_list.push(begin_word);

        let mut list = vec![vec![]; word_list.len()];
        for i in 0..word_list.len() {
            for j in (i + 1)..word_list.len() {
                if Self::distance(&word_list[i], &word_list[j]) == 1 {
                    list[i].push(j);
                    list[j].push(i);
                }
            }
        }

        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(vec![s]);
        let mut pos = vec![std::usize::MAX; word_list.len()];
        pos[s] = 0;

        while let Some(v) = queue.pop_front() {
            let last = *v.last().unwrap();
            if last == e {
                result.push(
                    v.iter()
                        .map(|i| word_list[*i].clone())
                        .collect::<Vec<String>>(),
                );
            }

            if !result.is_empty() && v.len() == result[0].len() {
                continue;
            }

            for next in &list[last] {
                if pos[*next] < v.len() {
                    continue;
                }
                pos[*next] = v.len();
                let mut next_v = v.clone();
                next_v.push(*next);
                queue.push_back(next_v);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day24() {
        assert_eq!(
            Solution::find_ladders(
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
            ),
            vec![
                vec![
                    "hit".to_string(),
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "cog".to_string()
                ],
                vec![
                    "hit".to_string(),
                    "hot".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            ]
        );

        assert_eq!(
            Solution::find_ladders(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                ]
            ),
            vec![] as Vec<Vec<String>>
        );
    }
}
