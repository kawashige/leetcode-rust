pub struct Solution {}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut acc = vec![0; words.len() + 1];
        for i in 0..words.len() {
            if words[i].starts_with(&vowels) && words[i].ends_with(&vowels) {
                acc[i + 1] = 1;
            }
            acc[i + 1] += acc[i];
        }

        queries
            .into_iter()
            .map(|q| acc[q[1] as usize + 1] - acc[q[0] as usize])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2559() {
        assert_eq!(
            Solution::vowel_strings(
                vec![
                    "aba".to_string(),
                    "bcb".to_string(),
                    "ece".to_string(),
                    "aa".to_string(),
                    "e".to_string()
                ],
                vec![vec![0, 2], vec![1, 4], vec![1, 1]]
            ),
            vec![2, 3, 0]
        );
        assert_eq!(
            Solution::vowel_strings(
                vec!["a".to_string(), "e".to_string(), "i".to_string()],
                vec![vec![0, 2], vec![0, 1], vec![2, 2]]
            ),
            vec![3, 2, 1]
        );
    }
}
