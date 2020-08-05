struct WordDictionary {
    root: HashMap<char, Trie>,
}

use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    c: char,
    children: HashMap<char, Trie>,
    terminal: bool,
}

impl Trie {
    fn new(c: char) -> Self {
        Trie {
            c: c,
            children: HashMap::new(),
            terminal: false,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        WordDictionary {
            root: HashMap::new(),
        }
    }

    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        let chars = word.chars().collect::<Vec<char>>();
        if chars.len() > 0 {
            let start = chars[0];
            let mut trie = self.root.entry(start).or_insert(Trie::new(start));
            for i in 1..chars.len() {
                trie = trie.children.entry(chars[i]).or_insert(Trie::new(chars[i]));
            }
            trie.terminal = true;
        }
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        fn check(t: &Trie, chars: &mut std::str::Chars) -> bool {
            match chars.next() {
                Some('.') => t
                    .children
                    .values()
                    .any(|child| check(child, &mut chars.clone())),
                Some(c) => match t.children.get(&c) {
                    Some(child) => check(child, chars),
                    None => false,
                },
                None => t.terminal,
            }
        }

        let mut chars = word.chars();
        match chars.next() {
            Some('.') => self
                .root
                .values()
                .any(|child| check(child, &mut chars.clone())),
            Some(c) => match self.root.get(&c) {
                Some(child) => check(child, &mut chars),
                None => false,
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day5() {
        let mut obj = WordDictionary::new();
        obj.add_word("at".to_string());
        obj.add_word("and".to_string());
        obj.add_word("an".to_string());
        obj.add_word("add".to_string());
        assert_eq!(false, obj.search("a".to_string()));
        assert_eq!(false, obj.search("bat".to_string()));
        obj.add_word("bat".to_string());
        assert_eq!(true, obj.search(".at".to_string()));
        assert_eq!(true, obj.search("an.".to_string()));
        assert_eq!(false, obj.search("a.d.".to_string()));
        assert_eq!(false, obj.search("b.".to_string()));
        assert_eq!(true, obj.search("a.d".to_string()));
        assert_eq!(false, obj.search(".".to_string()));
    }
}
