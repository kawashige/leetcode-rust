use std::collections::HashMap;

pub struct Solution {}

#[derive(Clone, Debug)]
struct Trie {
    children: HashMap<u8, Trie>,
    count: i32,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            count: 0,
        }
    }

    fn insert(&mut self, s: &str) {
        let mut trie = self;
        for b in s.as_bytes().iter() {
            trie = trie.children.entry(*b).or_insert(Trie::new());
            trie.count += 1;
        }
    }

    fn find(&self, s: &str) -> i32 {
        let mut trie = self;
        let mut count = 0;
        for b in s.as_bytes().iter() {
            if let Some(t) = trie.children.get(b) {
                count += t.count;
                trie = t;
            } else {
                break;
            }
        }
        count
    }
}
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for word in &words {
            trie.insert(word);
        }
        words.into_iter().map(|w| trie.find(&w)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2416() {
        assert_eq!(
            Solution::sum_prefix_scores(vec![
                "abc".to_string(),
                "ab".to_string(),
                "bc".to_string(),
                "b".to_string()
            ]),
            vec![5, 4, 3, 2]
        );
        assert_eq!(
            Solution::sum_prefix_scores(vec!["abcd".to_string()]),
            vec![4]
        );
    }
}
