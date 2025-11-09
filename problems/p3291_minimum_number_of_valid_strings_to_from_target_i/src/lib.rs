use std::collections::HashMap;

pub struct Solution {}

#[derive(Debug)]
struct Trie {
    c: char,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new(c: char) -> Self {
        Trie {
            c,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str) {
        let mut trie = self;
        for c in key.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new(c));
        }
    }

    fn find(&self, prefix: &str) -> usize {
        let mut trie = self;
        let mut index = 0;
        for c in prefix.chars() {
            if let Some(t) = trie.children.get(&c) {
                index += 1;
                trie = t;
            } else {
                return index;
            }
        }
        index
    }
}
impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let mut trie = Trie::new('?');
        for w in words {
            trie.insert(&w);
        }

        let mut dp = vec![std::i32::MAX; target.len() + 1];
        dp[0] = 0;

        for i in 0..target.len() {
            if dp[i] == std::i32::MAX {
                continue;
            }
            let len = trie.find(&target[i..]);
            for j in i..i + len {
                dp[j + 1] = dp[j + 1].min(dp[i] + 1);
            }
        }

        if dp[dp.len() - 1] == std::i32::MAX {
            -1
        } else {
            dp[dp.len() - 1]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3291() {
        assert_eq!(
            Solution::min_valid_strings(
                vec!["abc".to_string(), "aaaaa".to_string(), "bcdef".to_string()],
                "aabcdabc".to_string()
            ),
            3
        );
        assert_eq!(
            Solution::min_valid_strings(
                vec!["abababab".to_string(), "ab".to_string()],
                "ababaababa".to_string()
            ),
            2
        );
        assert_eq!(
            Solution::min_valid_strings(vec!["abcdef".to_string()], "xyz".to_string()),
            -1
        );
    }
}
