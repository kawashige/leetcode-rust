pub struct Solution {}

#[derive(Default)]
struct Trie {
    word: Option<String>,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, key: String) {
        let mut trie = self;
        for c in key.chars() {
            trie = trie.children[c as usize - 0x61].get_or_insert(Box::new(Trie::new()));
        }
        trie.word = key.into();
    }

    fn find(&self, prefix: &str) -> Vec<String> {
        let mut trie = self;
        for c in prefix.chars() {
            if let Some(t) = trie.children[c as usize - 0x61].as_ref() {
                trie = t;
            } else {
                return Default::default();
            }
        }

        let mut results = Vec::new();
        trie.collect(&mut results);
        results
    }

    fn collect(&self, results: &mut Vec<String>) {
        if results.len() == 3 {
            return;
        }

        if let Some(w) = self.word.as_ref() {
            results.push(w.to_string());
        }

        for c in &self.children {
            if let Some(trie) = c.as_ref() {
                trie.collect(results);
            }
        }
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for p in products {
            trie.insert(p);
        }

        (1..=search_word.len())
            .map(|i| trie.find(&search_word[..i]))
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
