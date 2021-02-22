use std::collections::{HashMap, HashSet};

struct MagicDictionary {
    dic: HashMap<String, HashSet<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            dic: HashMap::new(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in dictionary {
            for key in Self::serach_words(&s) {
                (*self.dic.entry(key).or_insert(HashSet::new())).insert(s.clone());
            }
        }
    }

    fn search(&self, search_word: String) -> bool {
        Self::serach_words(&search_word).into_iter().any(|w| {
            if let Some(set) = self.dic.get(&w) {
                !set.contains(&search_word) || set.len() > 1
            } else {
                false
            }
        })
    }

    fn serach_words(s: &String) -> Vec<String> {
        let chars = s.chars().collect::<Vec<char>>();
        (0..chars.len())
            .map(|i| {
                chars[..i]
                    .iter()
                    .chain(std::iter::once(&'*'))
                    .chain(chars[(i + 1)..].iter())
                    .collect::<String>()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_676() {
        let mut obj = MagicDictionary::new();
        obj.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
        assert!(!obj.search("hello".to_string()));
        assert!(obj.search("hhllo".to_string()));
        assert!(!obj.search("hell".to_string()));
        assert!(!obj.search("leetcoded".to_string()));
    }
}
