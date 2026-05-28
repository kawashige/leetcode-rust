use std::collections::HashMap;

pub struct Solution {}

#[derive(Clone, Debug)]
struct Trie {
    index: usize,
    children: HashMap<u8, Trie>,
}

impl Trie {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            children: HashMap::new(),
        }
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let not_found_index = (0..words_container.len())
            .min_by_key(|i| (words_container[*i].len(), *i))
            .unwrap();

        let mut root = Trie::new(not_found_index);

        for i in 0..words_container.len() {
            let w = &words_container[i];
            let mut trie = &mut root;
            for b in w.as_bytes().iter().rev() {
                trie = trie.children.entry(*b).or_insert(Trie::new(i));
                if w.len() < words_container[trie.index].len() {
                    trie.index = i;
                }
            }
        }

        words_query
            .into_iter()
            .map(|q| {
                let mut trie = &root;
                for b in q.as_bytes().iter().rev() {
                    if let Some(t) = trie.children.get(b) {
                        trie = t;
                    } else {
                        break;
                    }
                }
                trie.index as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3093() {
        assert_eq!(
            Solution::string_indices(
                vec!["abcd".to_string(), "bcd".to_string(), "xbcd".to_string()],
                vec!["cd".to_string(), "bcd".to_string(), "xyz".to_string()]
            ),
            vec![1, 1, 1]
        );
        assert_eq!(
            Solution::string_indices(
                vec![
                    "abcdefgh".to_string(),
                    "poiuygh".to_string(),
                    "ghghgh".to_string()
                ],
                vec![
                    "gh".to_string(),
                    "acbfgh".to_string(),
                    "acbfegh".to_string()
                ]
            ),
            vec![2, 0, 2]
        );
    }
}

fn main() {}
