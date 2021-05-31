use std::collections::HashMap;
pub struct Solution {}
struct Trie {
    indices: Vec<usize>,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            indices: Vec::new(),
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str, index: usize) {
        let mut trie = self;
        for c in key.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new());
            trie.indices.push(index);
        }
    }

    fn find(&self, prefix: &str) -> Vec<usize> {
        let mut trie = self;
        for c in prefix.chars() {
            if let Some(t) = trie.children.get(&c) {
                trie = t;
            } else {
                return Default::default();
            }
        }
        trie.indices.clone()
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for i in 0..products.len() {
            trie.insert(&products[i], i);
        }

        (1..=search_word.len())
            .map(|i| {
                let mut v = trie
                    .find(&search_word[..i])
                    .into_iter()
                    .map(|j| &products[j])
                    .collect::<Vec<&String>>();
                v.sort_unstable();
                v.into_iter().take(3).cloned().collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day31() {
        assert_eq!(
            Solution::suggested_products(
                vec![
                    "mobile".to_string(),
                    "mouse".to_string(),
                    "moneypot".to_string(),
                    "monitor".to_string(),
                    "mousepad".to_string()
                ],
                "mouse".to_string()
            ),
            vec![
                vec![
                    "mobile".to_string(),
                    "moneypot".to_string(),
                    "monitor".to_string()
                ],
                vec![
                    "mobile".to_string(),
                    "moneypot".to_string(),
                    "monitor".to_string()
                ],
                vec!["mouse".to_string(), "mousepad".to_string()],
                vec!["mouse".to_string(), "mousepad".to_string()],
                vec!["mouse".to_string(), "mousepad".to_string()]
            ]
        );

        assert_eq!(
            Solution::suggested_products(vec!["havana".to_string()], "havana".to_string()),
            vec![
                vec!["havana".to_string()],
                vec!["havana".to_string()],
                vec!["havana".to_string()],
                vec!["havana".to_string()],
                vec!["havana".to_string()],
                vec!["havana".to_string()]
            ]
        );

        assert_eq!(
            Solution::suggested_products(
                vec![
                    "bags".to_string(),
                    "baggage".to_string(),
                    "banner".to_string(),
                    "box".to_string(),
                    "cloths".to_string()
                ],
                "bags".to_string()
            ),
            vec![
                vec![
                    "baggage".to_string(),
                    "bags".to_string(),
                    "banner".to_string()
                ],
                vec![
                    "baggage".to_string(),
                    "bags".to_string(),
                    "banner".to_string()
                ],
                vec!["baggage".to_string(), "bags".to_string()],
                vec!["bags".to_string()]
            ]
        );

        assert_eq!(
            Solution::suggested_products(vec!["havana".to_string()], "tatiana".to_string()),
            vec![
                vec![] as Vec<String>,
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![]
            ]
        );
    }
}
