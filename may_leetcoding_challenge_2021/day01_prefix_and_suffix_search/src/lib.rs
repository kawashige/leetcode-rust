use std::collections::{BTreeSet, HashMap};

struct Trie {
    c: char,
    indices: BTreeSet<usize>,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new(c: char) -> Self {
        Trie {
            c,
            indices: BTreeSet::new(),
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str, index: usize) {
        let mut trie = self;
        for c in key.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new(c));
            trie.indices.insert(index);
        }
    }

    fn find(&self, prefix: String) -> BTreeSet<usize> {
        let mut trie = self;
        for c in prefix.chars() {
            if let Some(t) = trie.children.get(&c) {
                trie = t;
            } else {
                return BTreeSet::new();
            }
        }
        trie.indices.clone()
    }
}

struct WordFilter {
    prefix_trie: Trie,
    suffix_trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut wf = Self {
            prefix_trie: Trie::new('*'),
            suffix_trie: Trie::new('*'),
        };

        for (i, w) in words.into_iter().enumerate() {
            wf.prefix_trie.insert(&w, i);
            wf.suffix_trie
                .insert(&w.chars().rev().collect::<String>(), i);
        }

        wf
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let prefix = self.prefix_trie.find(prefix);
        let suffix = self.suffix_trie.find(suffix.chars().rev().collect());

        for p in prefix.iter().rev() {
            if suffix.contains(p) {
                return *p as i32;
            }
        }
        -1
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
