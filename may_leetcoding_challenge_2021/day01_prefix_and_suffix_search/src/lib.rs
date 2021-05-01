use std::collections::HashMap;
struct Trie {
    c: char,
    index: i32,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new(c: char) -> Self {
        Trie {
            c,
            index: 0,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str, index: i32) {
        let mut trie = self;
        for c in key.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new(c));
            trie.index = index;
        }
    }

    fn find(&self, prefix: String) -> i32 {
        let mut trie = self;
        for c in prefix.chars() {
            if let Some(t) = trie.children.get(&c) {
                trie = t;
            } else {
                return -1;
            }
        }
        trie.index
    }
}

struct WordFilter {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new('*');

        for (i, w) in words.into_iter().enumerate() {
            let key = format!("{}#{}", w, w);
            for j in 0..w.len() {
                trie.insert(&key[j..], i as i32);
            }
        }

        Self { trie }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let key = format!("{}#{}", suffix, prefix);
        self.trie.find(key)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day01() {
        let wf = WordFilter::new(vec!["apple".to_string()]);
        assert_eq!(wf.f("a".to_string(), "e".to_string()), 0);
    }
}
