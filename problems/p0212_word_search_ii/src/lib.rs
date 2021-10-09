pub struct Solution {}

use std::collections::{HashMap, HashSet};
pub struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn insert(&mut self, key: &str) {
        let mut trie = self;
        for c in key.chars() {
            trie = trie.children.entry(c).or_insert(Trie::new());
        }
        trie.is_end = true;
    }
}

impl Solution {
    pub fn dfs(
        i: usize,
        j: usize,
        board: &Vec<Vec<char>>,
        trie: &Trie,
        seen: &mut Vec<Vec<bool>>,
        s: &mut String,
        set: &mut HashSet<String>,
    ) {
        if trie.is_end {
            set.insert(s.clone());
        }

        for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter() {
            let (c, r) = (i as i32 + x, j as i32 + y);
            if c < 0
                || r < 0
                || c >= board.len() as i32
                || r >= board[0].len() as i32
                || seen[c as usize][r as usize]
                || !trie.children.contains_key(&board[c as usize][r as usize])
            {
                continue;
            }
            seen[c as usize][r as usize] = true;
            s.push(board[c as usize][r as usize]);
            Self::dfs(
                c as usize,
                r as usize,
                board,
                &trie.children[&board[c as usize][r as usize]],
                seen,
                s,
                set,
            );
            s.pop();
            seen[c as usize][r as usize] = false;
        }
    }

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(&word);
        }

        let mut set = HashSet::new();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if trie.children.contains_key(&board[i][j]) {
                    let mut seen = vec![vec![false; board[0].len()]; board.len()];
                    let mut s = board[i][j].clone().to_string();
                    seen[i][j] = true;
                    Self::dfs(
                        i,
                        j,
                        &board,
                        &trie.children[&board[i][j]],
                        &mut seen,
                        &mut s,
                        &mut set,
                    );
                }
            }
        }

        set.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0212() {
        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec![
                    "oath".to_string(),
                    "pea".to_string(),
                    "eat".to_string(),
                    "rain".to_string()
                ]
            ),
            vec!["eat".to_string(), "oath".to_string()]
        );
        assert_eq!(
            Solution::find_words(
                vec![vec!['a', 'b'], vec!['c', 'd']],
                vec!["abcb".to_string()]
            ),
            vec![] as Vec<String>
        );
        assert_eq!(
            Solution::find_words(vec![vec!['a']], vec!["a".to_string()]),
            vec!["a".to_string()]
        );
    }
}
