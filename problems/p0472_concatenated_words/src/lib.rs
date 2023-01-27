pub struct Solution {}

use std::collections::HashMap;
pub struct Trie {
    children: HashMap<char, Trie>,
    is_terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    fn insert(&mut self, key: &str) {
        let mut trie = self;
        for c in key.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new());
        }
        trie.is_terminal = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut entry = self;
        for c in word.chars() {
            if let Some(child) = entry.children.get(&c) {
                entry = child;
            } else {
                return false;
            }
        }
        entry.is_terminal
    }
}

impl Solution {
    pub fn is_concateneted(word: &str, trie: &Trie, count: usize) -> bool {
        if word.is_empty() {
            return count != 1;
        }
        for i in 0..word.len() {
            if trie.search(&word[..=i]) && Self::is_concateneted(&word[i + 1..], trie, count + 1) {
                return true;
            }
        }
        false
    }

    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for word in &words {
            trie.insert(word);
        }

        words
            .into_iter()
            .filter(|word| Self::is_concateneted(word, &trie, 0))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0472() {
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(vec![
                "cat".to_string(),
                "cats".to_string(),
                "catsdogcats".to_string(),
                "dog".to_string(),
                "dogcatsdog".to_string(),
                "hippopotamuses".to_string(),
                "rat".to_string(),
                "ratcatdogcat".to_string()
            ]),
            vec![
                "catsdogcats".to_string(),
                "dogcatsdog".to_string(),
                "ratcatdogcat".to_string()
            ]
        );
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(vec![
                "cat".to_string(),
                "dog".to_string(),
                "catdog".to_string()
            ]),
            vec!["catdog"]
        );
    }
}
