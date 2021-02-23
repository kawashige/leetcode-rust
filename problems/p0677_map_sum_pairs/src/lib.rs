use std::collections::HashMap;

struct MapSum {
    root: Trie,
}

#[derive(Debug)]
struct Trie {
    c: char,
    val: HashMap<String, i32>,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new(c: char) -> Self {
        Trie {
            c,
            val: HashMap::new(),
            children: HashMap::new(),
        }
    }
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            root: Trie::new('*'),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut trie = &mut self.root;
        for c in key.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new(c));
            *trie.val.entry(key.clone()).or_insert(0) = val;
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut trie = &self.root;
        for c in prefix.chars() {
            if let Some(t) = trie.children.get(&c) {
                trie = t;
            } else {
                return 0;
            }
        }
        trie.val.values().sum()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0677() {
        let mut obj = MapSum::new();
        obj.insert("apple".to_string(), 3);
        assert_eq!(obj.sum("ap".to_string()), 3);
        obj.insert("app".to_string(), 2);
        assert_eq!(obj.sum("ap".to_string()), 5);
        obj.insert("apple".to_string(), 2);
        assert_eq!(obj.sum("ap".to_string()), 4);
    }
}
