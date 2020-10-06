use std::collections::HashMap;

#[derive(Default, Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    is_terminal: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut entry = self;
        for c in word.chars() {
            entry = entry.children.entry(c).or_insert(Default::default());
        }
        entry.is_terminal = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
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

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut entry = self;
        for c in prefix.chars() {
            if let Some(child) = entry.children.get(&c) {
                entry = child;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0208() {
        let mut obj = Trie::new();
        obj.insert("apple".to_string());
        assert!(obj.search("apple".to_string()));
        assert!(!obj.search("app".to_string()));
        assert!(obj.starts_with("app".to_string()));
        obj.insert("app".to_string());
        assert!(obj.search("app".to_string()));
    }
}
