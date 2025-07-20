use std::collections::HashMap;
pub struct Solution {}

#[derive(Debug, Default)]
struct Trie {
    serial: String,
    subfolder: HashMap<String, Trie>,
}

impl Solution {
    pub fn serialize(trie: &mut Trie, freq: &mut HashMap<String, usize>) {
        if trie.subfolder.is_empty() {
            return;
        }

        let mut v = Vec::new();
        for (folder, subfolder) in trie.subfolder.iter_mut() {
            Self::serialize(subfolder, freq);
            v.push(format!("{}({})", folder, subfolder.serial));
        }

        v.sort_unstable();
        trie.serial = v.join("");
        *freq.entry(trie.serial.clone()).or_insert(0) += 1;
    }

    pub fn delete(
        trie: &Trie,
        freq: &HashMap<String, usize>,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if &1 < freq.get(&trie.serial).unwrap_or(&0) {
            return;
        }

        if !path.is_empty() {
            result.push(path.clone())
        }

        for (folder, subfolder) in &trie.subfolder {
            path.push(folder.clone());
            Self::delete(subfolder, freq, path, result);
            path.pop();
        }
    }

    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Trie::default();

        for path in paths {
            let mut trie = &mut root;
            for folder in path {
                trie = trie.subfolder.entry(folder).or_insert(Trie::default());
            }
        }

        let mut freq = HashMap::new();
        Self::serialize(&mut root, &mut freq);

        let mut path = Vec::new();
        let mut result = Vec::new();
        Self::delete(&root, &freq, &mut path, &mut result);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1948() {
        assert_eq!(
            Solution::delete_duplicate_folder(vec![
                vec!["a".to_string()],
                vec!["c".to_string()],
                vec!["d".to_string()],
                vec!["a".to_string(), "b".to_string()],
                vec!["c".to_string(), "b".to_string()],
                vec!["d".to_string(), "a".to_string()]
            ]),
            vec![
                vec!["d".to_string()],
                vec!["d".to_string(), "a".to_string()]
            ]
        );
        assert_eq!(
            Solution::delete_duplicate_folder(vec![
                vec!["a".to_string()],
                vec!["c".to_string()],
                vec!["a".to_string(), "b".to_string()],
                vec!["c".to_string(), "b".to_string()],
                vec!["a".to_string(), "b".to_string(), "x".to_string()],
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "x".to_string(),
                    "y".to_string()
                ],
                vec!["w".to_string()],
                vec!["w".to_string(), "y".to_string()]
            ]),
            vec![
                vec!["c".to_string()],
                vec!["c".to_string(), "b".to_string()],
                vec!["a".to_string()],
                vec!["a".to_string(), "b".to_string()]
            ]
        );
        assert_eq!(
            Solution::delete_duplicate_folder(vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["c".to_string(), "d".to_string()],
                vec!["c".to_string()],
                vec!["a".to_string()]
            ]),
            vec![
                vec!["c".to_string()],
                vec!["c".to_string(), "d".to_string()],
                vec!["a".to_string()],
                vec!["a".to_string(), "b".to_string()]
            ]
        );
    }
}
