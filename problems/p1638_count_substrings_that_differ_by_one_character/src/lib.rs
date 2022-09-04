use std::collections::HashMap;

pub struct Solution {}

#[derive(Default, Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    count: i32,
}
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut entry = self;
        for c in word.chars() {
            entry = entry.children.entry(c).or_insert(Default::default());
            entry.count += 1;
        }
    }

    fn search(&self, bytes: &[u8]) -> i32 {
        Self::recurse(self, bytes, false)
    }

    fn recurse(trie: &Trie, bytes: &[u8], changed: bool) -> i32 {
        let mut count = if changed { trie.count } else { 0 };
        if bytes.is_empty() {
            return count;
        }

        for (c, t) in &trie.children {
            if *c as u8 == bytes[0] {
                count += Self::recurse(t, &bytes[1..], changed);
            } else if !changed {
                count += Self::recurse(t, &bytes[1..], true);
            }
        }

        count
    }
}

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let mut trie = Trie::new();
        for i in 0..t.len() {
            trie.insert(t[i..].to_string());
        }

        let mut count = 0;

        for i in 0..s.len() {
            count += trie.search(&s.as_bytes()[i..])
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1638() {
        assert_eq!(
            Solution::count_substrings(
                "azazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazaz"
                    .to_string(),
                "zazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazazaza"
                    .to_string()
            ),
            3200
        );
        assert_eq!(
            Solution::count_substrings("aba".to_string(), "baba".to_string()),
            6
        );
        assert_eq!(
            Solution::count_substrings("ab".to_string(), "bb".to_string()),
            3
        );
    }
}
